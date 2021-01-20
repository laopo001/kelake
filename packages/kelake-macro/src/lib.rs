#![allow(unused)]
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         vnode::VNode::from("123");

//         assert_eq!(2 + 2, 4);
//     }
// }
mod react;
mod tag;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use react::HtmlRootVNode;
use syn::{parse_macro_input, buffer::Cursor};

#[proc_macro]
pub fn react(input: TokenStream) -> TokenStream {
    dbg!(&input);
    let root = parse_macro_input!(input as HtmlRootVNode);
    // let root = HtmlRootVNode;
    TokenStream::from(root.into_token_stream())
}

trait PeekValue<T> {
    fn peek(cursor: Cursor) -> Option<T>;
}
