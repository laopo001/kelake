use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result,ParseBuffer};

pub struct HtmlRootVNode;
impl Parse for HtmlRootVNode {
    fn parse(input: ParseStream) -> Result<Self> {
        // return Ok(HtmlRootVNode);
        input.parse()
    }
}

impl ToTokens for HtmlRootVNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend( quote! {
            println!("Hello, {}!", "world");
        });
    }
}


pub struct HtmlRoot;
impl Parse for HtmlRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse()
    }
}

impl ToTokens for HtmlRoot {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend( quote! {
            println!("Hello, {}!", "world");
        });
    }
}