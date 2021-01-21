#![allow(unused)]
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         vnode::VNode::from("123");

//         assert_eq!(2 + 2, 4);
//     }
// }
mod html_element;
mod react;
mod tag;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use react::HtmlVNode;
use syn::{buffer::Cursor, parse_macro_input};

#[proc_macro]
pub fn react(input: TokenStream) -> TokenStream {
    dbg!(&input);
    let root = parse_macro_input!(input as HtmlVNode);
    // let root = HtmlRootVNode;
    TokenStream::from(root.into_token_stream())
}

trait PeekValue<T> {
    fn peek(cursor: Cursor) -> Option<T>;
}

fn join_errors(mut it: impl Iterator<Item = syn::Error>) -> syn::Result<()> {
    it.next().map_or(Ok(()), |mut err| {
        for other in it {
            err.combine(other);
        }
        Err(err)
    })
}
