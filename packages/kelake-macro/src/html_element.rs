use crate::react::{HtmlBlock, HtmlVNode};
use crate::PeekValue;
use boolinator::Boolinator;
use proc_macro2::{Ident, Literal, Punct, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use serde_json::{json, Value};
use syn::buffer::Cursor;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream, Result},
    Token,
};

pub struct HtmlElement {
    name: String,
    props: ElementProps,
    children: HtmlElementChildren,
}

impl PeekValue<()> for HtmlElement {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlElementOpen::peek(cursor)
            .or_else(|| HtmlElementClose::peek(cursor))
            .map(|_| ())
    }
}

impl Parse for HtmlElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if HtmlElementClose::peek(input.cursor()).is_some() {
            return match input.parse::<HtmlElementClose>() {
                Ok(close) => Err(syn::Error::new_spanned(
                    close.to_spanned(),
                    "this closing tag has no corresponding opening tag",
                )),
                Err(err) => Err(err),
            };
        }
        let open = input.parse::<HtmlElementOpen>()?;
        // Return early if it's a self-closing tag

        if open.is_self_closing() {
            return Ok(HtmlElement {
                name: open.name,
                props: open.props,
                children: HtmlElementChildren::new(),
            });
        }

        // let open_key = open.name.get_key();
        let mut children = HtmlElementChildren::new();
        // let mut count = 1;
        let mut a = 1;

        loop {
            dbg!(input.to_string());
            if input.is_empty() {
                return Err(syn::Error::new_spanned(open.to_spanned(), "没有关闭标签"));
            }

            if HtmlElementClose::peek(input.cursor()).is_some() {
                break;
            }
            a += 1;
            if (a == 10000) {
                return Err(syn::Error::new_spanned(open.to_spanned(), "无限循环"));
            }
            children.parse_child(input)?;
        }

        let close = input.parse::<HtmlElementClose>()?;
        if close.name != open.name {
            return Err(syn::Error::new_spanned(
                close.to_spanned(),
                "标签前后不一致",
            ));
        }

        Ok(Self {
            name: open.name.clone(),
            props: open.props,
            children,
        })
    }
}

impl ToTokens for HtmlElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name.to_string();
        let mut t = quote! {};
        self.children.to_tokens(&mut t);
        // let props = self.props.0.clone();

        let mut props_t = quote! {};
        self.props.to_tokens(&mut props_t);
        // let children = "abc".to_string();
        // dbg!(&name,&children);
        tokens.extend(quote! {
            VNodeChild::Node(VNode::new(#name.to_string(), #props_t , #t))
        });
        // dbg!(&tokens.to_string());
    }
}

pub struct HtmlElementOpen {
    pub lt: Token![<],
    name: String,
    props: ElementProps,
    pub div: Option<Token![/]>,
    pub gt: Token![>],
    // is_component: Bool
}
impl HtmlElementOpen {
    fn is_self_closing(&self) -> bool {
        self.div.is_some()
    }

    fn to_spanned(&self) -> impl ToTokens {
        let Self { lt, gt, .. } = self;
        quote! {#lt#gt}
    }
}

impl PeekValue<()> for HtmlElementOpen {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;
        let (ident, cursor) = cursor.ident()?;
        Some(())
    }
}

impl Parse for HtmlElementOpen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punct = input.parse::<Punct>()?;
        let lt = syn::token::Lt {
            spans: [punct.span()],
        };
        (punct.to_string() == "<").as_option();
        let name = input.parse::<Ident>()?.to_string();
        let mut props = input.parse::<ElementProps>()?;

        let mut is_close = false;
        let s = input.parse::<Punct>()?;
        let mut div: Option<Token![/]> = None;
        if (s.to_string() == "/").as_option().is_some() {
            div = Some(syn::token::Div { spans: [s.span()] });
            (input.parse::<Punct>()?.to_string() != ">").as_option();
        };
        (s.to_string() == ">").as_option();
        let gt = syn::token::Gt { spans: [s.span()] };

        Ok(Self {
            lt,
            name,
            props,
            div,
            gt,
        })
    }
}

pub struct HtmlElementClose {
    pub lt: Token![<],
    pub div: Token![/],
    name: String,
    pub gt: Token![>],
}
impl HtmlElementClose {
    fn to_spanned(&self) -> impl ToTokens {
        let Self { lt, gt, .. } = self;
        quote! {#lt#gt}
    }
}

impl PeekValue<()> for HtmlElementClose {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;
        let (ident, cursor) = cursor.ident()?;
        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;
        Some(())
    }
}

impl Parse for HtmlElementClose {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punct = input.parse::<Punct>()?;
        let lt = syn::token::Lt {
            spans: [punct.span()],
        };
        (punct.to_string() == "<").as_option();
        let s = input.parse::<Punct>()?;
        let div = syn::token::Div { spans: [s.span()] };
        let name = input.parse::<Ident>()?.to_string();
        let s = input.parse::<Punct>()?;
        let gt = syn::token::Gt { spans: [s.span()] };

        Ok(Self {
            lt,
            div,
            name,
            gt,
        })
    }
}

pub struct HtmlElementChildren(Vec<HtmlVNode>);

impl HtmlElementChildren {
    pub fn new() -> Self {
        HtmlElementChildren(vec![])
    }
    pub fn parse_child(&mut self, input: ParseStream) -> Result<()> {
        self.0.push(input.parse()?);
        Ok(())
    }
}

impl ToTokens for HtmlElementChildren {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let children = self.0.iter().map(|x| {
            let mut t = quote! {};
            x.to_tokens(&mut t);
            t
        });

        tokens.extend(quote! {
            vec![
                #(
                    #children
                ),*
            ]
        });
    }
}

pub struct ElementProps(Vec<TokenStream>);

impl Parse for ElementProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.cursor().ident().is_none() {
            return Ok(ElementProps(vec![quote! {}]));
        }
        let mut arr = vec![];
        loop {
            let key = input.parse::<Ident>()?.to_string();
            (input.parse::<Punct>()?.to_string() == "=").as_option();
            if HtmlBlock::peek(input.cursor()).is_some() {
                let mut t = quote! {};
                let block = input.parse::<HtmlBlock>()?;
                block.to_tokens(&mut t);
                t = block.get_real_tokens();
                let q = quote!(( #key.to_string(), format!("{}", #t) ));
                arr.push(q);
            } else {
                let value = input.parse::<Literal>()?;
                let q = quote!(( #key.to_string(), #value.to_string()));
                arr.push(q);
            }
            if let Some((punct, cursor)) = input.cursor().punct() {
                if (punct.as_char() == '/') {
                    break;
                }
                if (punct.as_char() == '>') {
                    break;
                }
            }
        }
        Ok(ElementProps(arr))
    }
}

impl ToTokens for ElementProps {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let children = self.0.iter();

        tokens.extend(quote! {
            vec![
                #(
                    #children
                ),*
            ]
        });
    }
}
