use crate::react::{HtmlBlock, HtmlVNode};
use crate::PeekValue;
use boolinator::Boolinator;
use proc_macro2::{Ident, Literal, Punct, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
// use serde_json::{json, Value};

use syn::buffer::Cursor;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream, Result},
    Token,
};

pub struct HtmlElement {
    name: String,
    element_props: Option<ElementProps>,
    is_component: bool,
    children: HtmlElementChildren,
    component_props: Option<ComponentProps>,
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
        let is_component = open.is_component;

        if open.is_self_closing() {
            return Ok(HtmlElement {
                name: open.name,
                element_props: open.element_props,
                component_props: open.component_props,
                is_component,
                children: HtmlElementChildren::new(),
            });
        }

        // let open_key = open.name.get_key();
        let mut children = HtmlElementChildren::new();
        let mut a = 1;

        loop {
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
            component_props: open.component_props,
            element_props: open.element_props,
            is_component,
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
        if self.is_component {
            let name_ident = Ident::new(&name, Span::call_site());
            let props_ident = Ident::new(&(name + "Props"), Span::call_site());
            props_t.extend(quote!(#props_ident));
            self.component_props
                .as_ref()
                .unwrap()
                .to_tokens(&mut props_t);

            tokens.extend(quote! {
                #name_ident::create(#props_t,  #t)
            });
        } else {
            self.element_props.as_ref().unwrap().to_tokens(&mut props_t);
            tokens.extend(quote! {
                VNodeChild::Node(VNode::new(#name.to_string(), #props_t , #t))
            });
        }
    }
}

pub struct HtmlElementOpen {
    pub lt: Token![<],
    name: String,
    element_props: Option<ElementProps>,
    pub div: Option<Token![/]>,
    pub gt: Token![>],
    is_component: bool,
    component_props: Option<ComponentProps>,
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

        let is_component = if name.bytes().next().unwrap().is_ascii_lowercase() {
            false
        } else {
            true
        };
        let mut element_props = None;
        let mut component_props = None;
        if is_component {
            component_props = Some(input.parse::<ComponentProps>()?);
        } else {
            element_props = Some(input.parse::<ElementProps>()?);
        }
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
            element_props,
            div,
            gt,
            is_component,
            component_props,
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

        Ok(Self { lt, div, name, gt })
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
            to_vnode_child_vec![
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
            return Ok(ElementProps(vec![]));
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
                if key.starts_with("on") {
                    let q = quote!(( #key.to_string(), PropsValue::Task( Rc::new( RefCell::new((#t .to_string(), Box::from_raw(unsafe {
                        let x = std::mem::transmute::<&Self, *mut Self>(self);
                        x
                    }))) )  ) ));
                    arr.push(q);
                } else {
                    let q = quote!(( #key.to_string(), format!("{:?}", #t) ));
                    arr.push(q);
                }
            } else {
                let value = input.parse::<Literal>()?;
                let q = quote!(( #key.to_string(), PropsValue::String(#value.to_string())));
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

        tokens.extend(quote! {{
            let mut m = ::std::collections::HashMap::<String,PropsValue>::new();

                #(
                    m.insert#children;
                ),*

            m
        }});
    }
}

pub struct ComponentProps(Vec<TokenStream>);

impl Parse for ComponentProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.cursor().ident().is_none() {
            return Ok(ComponentProps(vec![quote! {}]));
        }
        let mut arr = vec![];
        loop {
            let key = input.parse::<Ident>()?;
            (input.parse::<Punct>()?.to_string() == "=").as_option();
            if HtmlBlock::peek(input.cursor()).is_some() {
                let mut t = quote! {};
                let block = input.parse::<HtmlBlock>()?;
                block.to_tokens(&mut t);
                t = block.get_real_tokens();
                let q = quote!( #key :  #t );
                arr.push(q);
            } else {
                let value = input.parse::<Literal>()?;
                let q = quote!( #key : #value.to_string());
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
        Ok(ComponentProps(arr))
    }
}

impl ToTokens for ComponentProps {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let children = self.0.iter();

        tokens.extend(quote! {
            {
                #(
                    #children
                ),*
            }
        });
    }
}
