#![allow(unused)]
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         vnode::VNode::from("123");

//         assert_eq!(2 + 2, 4);
//     }
// }
mod react_vnode;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use react_vnode::HtmlRootVNode;
use syn::parse_macro_input;

#[proc_macro]
pub fn react(input: TokenStream) -> TokenStream {
    // let root = parse_macro_input!(input as HtmlRootVNode);
    let root = HtmlRootVNode;
    TokenStream::from(root.into_token_stream())
}
