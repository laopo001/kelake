#![allow(unused)]
pub mod vnode;
use kelake_macro::react;
use serde_json::{json, Value};
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

#[test]
fn test() {
    react!(<div><div>abc</div></div>);

    assert_eq!(2 + 2, 4);
}
