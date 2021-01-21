#![allow(unused)]
pub mod vnode;
use kelake_macro::react;
use serde_json::{json, Value};
use vnode::{VNode, VNodeChild};
pub trait Component {
    // type Properties: Properties;
}

// macro_rules! html {
//     (<$tag_start:ident>$children:tt</$tag_end:ident>) => {
//         vnode::create_base_element(
//             stringify!($tag_start),
//             json! {{}},
//             html!($children),
//         )
//     };
//     // ($(<$tag_start:ident>$children:tt</$tag_end:ident>)* ) => {
//     //     {
//     //         let mut arr = vec![];
//     //         $(
//     //             arr.push(vnode::create_base_element(
//     //                 stringify!($tag_start),
//     //                 json! {{}},
//     //                 vnode::VNode::from(html!($children)),
//     //             ));
//     //         )*
//     //         arr
//     //     }
//     // };
//     ($children:ident) => {
//         stringify!($children)
//     };
// }
// macro_rules! html_children {
//     (<$tag_start:ident>$children:ident</$tag_end:ident>) => {
//         // vnode::create_base_element(
//         //     stringify!($tag_start),
//         //     json! {{}},
//         //     vnode::VNode::from(html!($children)),
//         // )
//         123
//     };
//     ($children:ident) => {
//         stringify!($children)
//     };
// }

// macro_rules! var {
//     (<$tag_start:ident>$children:ident</$tag_end:ident>) => {};
// }

fn v(name:&str) -> String {
    name.to_string()
}

fn test2() {
    dbg!(react!(<div>abcs<div>asdasd</div>trertet</div>));
}

#[test]
fn test() {
    test2()
}
