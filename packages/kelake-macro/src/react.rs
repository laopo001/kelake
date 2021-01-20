use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use regex::Regex;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseBuffer, ParseStream, Result};
pub struct HtmlRootVNode;
impl Parse for HtmlRootVNode {
    fn parse(input: ParseStream) -> Result<Self> {
        // dbg!(input.parse::<TokenTree>());
        // dbg!(input.parse::<TokenTree>());
        // let (punct, cursor) =input.cursor().punct().unwrap();
        // dbg!(punct.as_char());
        // let (punct, _) =cursor.punct().unwrap();
        // dbg!(punct.as_char());
   
        // let arr = input.to_string().split(' ').map(|x|x.to_string()).collect::<Vec<String>>();
        is_element(input.to_string());
        while let Ok(_) = input.parse::<TokenTree>() {

        }
        return Ok(HtmlRootVNode);
        // input.parse()
    }
}

impl ToTokens for HtmlRootVNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            println!("Hello, {}!", "world");
        });
    }
}

fn is_element(s: String) -> Option<String> {
    dbg!(&s);
    let re = Regex::new(r"^< (\w+) > (.*) < / (\w+) >").unwrap();
    for cap in re.captures_iter(&s) {
        if(&cap[1] != &cap[3]) {
            panic!("标签名称前后不一");
        }
        println!("tag: {} children: {} tag: {}", &cap[1], &cap[2], &cap[3]);
    }
    None
}

pub struct HtmlVNode;
impl Parse for HtmlVNode {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse()
    }
}

impl ToTokens for HtmlVNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            println!("Hello, {}!", "world");
        });
    }
}
