use crate::html_element::{HtmlElement, HtmlElementChildren};
use crate::PeekValue;
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use regex::Regex;
use syn::{
    buffer::Cursor,
    parse::{Parse, ParseBuffer, ParseStream, Result},
    Token,
};

pub enum HtmlVNode {
    // ElementList(Box<HtmlElementChildren>),
    Element(Box<HtmlElement>),
    HtmlString(Box<HtmlString>),
    Empty,
}
impl Parse for HtmlVNode {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(HtmlVNode::Empty);
        } else if HtmlElement::peek(input.cursor()).is_some() {
            return Ok(HtmlVNode::Element(Box::new(input.parse::<HtmlElement>()?)));
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
        // tokens.extend(quote! {
        //     "123"
        // });
        // return;
        match self {
            Self::Element(element) => element.to_tokens(tokens),
            Self::HtmlString(s) => {
                s.as_ref().to_tokens(tokens);
            }
            _ => panic!("error"),
        }
    }
}

pub struct HtmlString(String);
impl PeekValue<()> for HtmlString {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.ident().map(|_| ())
    }
}

impl Parse for HtmlString {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let s = input.parse::<Ident>()?.to_string();
        Ok(HtmlString(s))
    }
}

impl ToTokens for HtmlString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = self.0.to_string();   
        tokens.extend(quote! { VNodeChild::VText(#s.to_string()) });
    }
}
