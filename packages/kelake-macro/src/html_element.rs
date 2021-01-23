use crate::react::{HtmlBlock, HtmlVNode};
use crate::tag::TagTokens;
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
        if close._name != open.name {
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
        let props = self.props.0.clone();
        // let children = "abc".to_string();
        // dbg!(&name,&children);
        tokens.extend(quote! {
            VNodeChild::Node(VNode::new(#name.to_string(), json!(#props) , #t))
        });
        // dbg!(&tokens.to_string());
    }
}

pub struct HtmlElementOpen {
    tag: TagTokens,
    name: String,
    props: ElementProps,
}
impl HtmlElementOpen {
    fn is_self_closing(&self) -> bool {
        self.tag.div.is_some()
    }

    fn to_spanned(&self) -> impl ToTokens {
        self.tag.to_spanned()
    }
}

impl PeekValue<()> for HtmlElementOpen {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;
        Some(())
    }
}

impl Parse for HtmlElementOpen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_start_content(input, |input, tag| {
            // let at = input.parse::<TokenTree>()?;
            let name = input.parse::<Ident>()?.to_string();
            let mut props = input.parse::<ElementProps>()?;

            Ok(Self { tag, name, props })
        })
    }
}

pub struct HtmlElementClose {
    tag: TagTokens,
    _name: String,
}
impl HtmlElementClose {
    fn to_spanned(&self) -> impl ToTokens {
        self.tag.to_spanned()
    }
}

impl PeekValue<()> for HtmlElementClose {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;

        // let (tag_key, cursor) = TagName::peek(cursor)?;
        // if let TagKey::Lit(name) = &tag_key {
        //     non_capitalized_ascii(&name.to_string()).as_option()?;
        // }
        let (ident, cursor) = cursor.ident()?;
        // dbg!(ident.to_string());
        // dbg!(punct.as_char());
        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;
        // dbg!(punct.as_char());
        Some(())
    }
}

impl Parse for HtmlElementClose {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_end_content(input, |input, tag| {
            let name = input.parse::<Ident>()?.to_string();

            // if let TagName::Expr(name) = &name {
            //     if let Some(expr) = &name.expr {
            //         return Err(syn::Error::new_spanned(
            //         expr,
            //         "dynamic closing tags must not have a body (hint: replace it with just `</@>`)",
            //     ));
            //     }
            // }

            Ok(Self { tag, _name: name })
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

pub struct ElementProps(TokenStream);

impl Parse for ElementProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.cursor().ident().is_none() {
            return Ok(ElementProps((quote! {{}})));
        }
      
        let key = input.parse::<Ident>()?.to_string();
        (input.parse::<Punct>()?.to_string() == "=").as_option();
        if HtmlBlock::peek(input.cursor()).is_some() {
            let mut t = quote! {};
            let block = input.parse::<HtmlBlock>()?;
            block.to_tokens(&mut t);
            dbg!(&t);
            let value = quote!({ #key: #t });
            return Ok(ElementProps(value));
        } else {
            let value = input.parse::<Literal>()?.to_string();
            let value = quote!({ #key: #value});
            Ok(ElementProps(value))
        }
    }
}
