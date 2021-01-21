use crate::html_element::{HtmlElement, HtmlElementChildren};
use crate::PeekValue;
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{
    buffer::Cursor,
    parse::{Parse, ParseBuffer, ParseStream, Result},
    Token,
};
pub struct HtmlRootVNode(HtmlVNode);
impl Parse for HtmlRootVNode {
    fn parse(input: ParseStream) -> Result<Self> {
        // dbg!(input.parse::<TokenTree>());
        // dbg!(input.parse::<TokenTree>());
        // let (punct, cursor) =input.cursor().punct().unwrap();
        // dbg!(punct.as_char());
        // let (punct, _) =cursor.punct().unwrap();
        // dbg!(punct.as_char());

        // let arr = input.to_string().split(' ').map(|x|x.to_string()).collect::<Vec<String>>();
        // is_element(input.to_string());
        // while let Ok(_) = input.parse::<TokenTree>() {}
        // return Ok(HtmlRootVNode);
        input.parse().map(Self)
    }
}

impl ToTokens for HtmlRootVNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            println!("Hello, {}!", "world");
        });
    }
}

// fn is_element(s: String) -> Option<String> {
//     dbg!(&s);
//     let re = Regex::new(r"^< (\w+) > (.*) < / (\w+) >").unwrap();
//     if !re.is_match(&s) {
//         panic!("html标签错误");
//     }
//     for cap in re.captures_iter(&s) {
//         if (&cap[1] != &cap[3]) {
//             panic!("标签名称前后不一");
//         }
//         println!("tag: {} children: {} tag: {}", &cap[1], &cap[2], &cap[3]);
//     }
//     None
// }

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
            return Ok(HtmlVNode::HtmlString(Box::new(input.parse::<HtmlString>()?)));
        } else {
            return Err(syn::Error::new_spanned(quote! {}, "格式错误"));
        }
    }
}

impl ToTokens for HtmlVNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            println!("Hello, {}!", "world");
        });
    }
}

pub struct HtmlString(String);
impl PeekValue<()> for HtmlString {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor.ident().map(|_|{()})
    }
}


impl Parse for HtmlString {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let s = input.parse::<Ident>()?.to_string();
        Ok(HtmlString(s))
    }
}