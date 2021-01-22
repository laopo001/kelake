use crate::html_element::{HtmlElement, HtmlElementChildren, HtmlElementClose, HtmlElementOpen};
use crate::react;
use crate::PeekValue;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use regex::Regex;
use syn::spanned::Spanned;
use syn::{
    braced,
    buffer::Cursor,
    parse::{Parse, ParseBuffer, ParseStream, Result},
    Expr, Token,
};
pub struct HtmlRootVNode(HtmlVNode);
impl Parse for HtmlRootVNode {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for HtmlRootVNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut t = quote! {};
        self.0.to_tokens(&mut t);
        println!("解析后:{}", &t.to_string());
        tokens.extend(quote! {
            {
                use kelake::vnode::{VNode, VNodeChild, format};
                #t
            }
        });
    }
}

pub enum HtmlVNode {
    Element(Box<HtmlElement>),
    HtmlString(Box<HtmlString>),
    Block(Box<HtmlBlock>),
    Empty,
}
impl Parse for HtmlVNode {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(HtmlVNode::Empty);
        } else if HtmlElement::peek(input.cursor()).is_some() {
            return Ok(HtmlVNode::Element(Box::new(input.parse::<HtmlElement>()?)));
        } else if HtmlBlock::peek(input.cursor()).is_some() {
            return Ok(HtmlVNode::Block(Box::new(input.parse::<HtmlBlock>()?)));
        } else if HtmlString::peek(input.cursor()).is_some() {
            return Ok(HtmlVNode::HtmlString(Box::new(
                input.parse::<HtmlString>()?,
            )));
        } else {
            return Err(syn::Error::new_spanned(quote! {}, "格式错误"));
        }
    }
}


impl ToTokens for HtmlVNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Element(element) => element.to_tokens(tokens),
            Self::HtmlString(s) => {
                s.to_tokens(tokens);
            }
            Self::Block(s) => {
                s.to_tokens(tokens);
            }
            _ => panic!("error"),
        }
    }
}

pub struct HtmlString(String);
impl PeekValue<()> for HtmlString {
    fn peek(cursor: Cursor) -> Option<()> {
        if HtmlElementClose::peek(cursor).is_some() || HtmlElementOpen::peek(cursor).is_some() {
            None
        } else {
            Some(())
        }
    }
}

impl Parse for HtmlString {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut s = "".to_string();

        if HtmlElementClose::peek(input.cursor()).is_some()
            || HtmlElementOpen::peek(input.cursor()).is_some()
        {
            return Ok(HtmlString(s));
        }
        if let Ok(ident) = input.parse::<Ident>() {
            s += &ident.to_string();
        } else if let Ok(l) = input.parse::<Literal>() {
            s += &l.to_string();
        } else if let Ok(punct) = input.parse::<Punct>() {
            s += &punct.to_string();
        } else if let Ok(g) = input.parse::<Group>() {
            panic!("Group")
            // dbg!(&g.stream().to_string());
            // s += &g.stream().to_string();
        }
        Ok(HtmlString(s))
    }
}

impl ToTokens for HtmlString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = self.0.to_string();
        tokens.extend(quote! { VNodeChild::Text(#s.to_string()) });
    }
}


pub enum HtmlBlock {
    // Node(HtmlVNode),
    Expr(Expr),
}

impl PeekValue<()> for HtmlBlock {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.group(Delimiter::Brace).map(|_| ())
    }
}

impl Parse for HtmlBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace = braced!(content in input);

        Ok(HtmlBlock::Expr(content.parse()?))
    }
}

impl ToTokens for HtmlBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlBlock::Expr(expr) => {
                tokens.extend(quote! {{
                    format(#expr)
                }});
            }
        }
    }
}
