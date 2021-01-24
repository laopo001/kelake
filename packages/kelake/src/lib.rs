#![allow(unused)]
// #![feature(core_intrinsics)]
#[macro_use]
pub mod vnode;
// pub mod render;
use serde_json::{json, Value};

use vnode::ToVNodeChild;
#[macro_export]
macro_rules! to_vnode_child_vec {
    ($($child: expr),*) => {
        {
            let mut arr = Vec::new();
            $(
                arr.push(ToVNodeChild::to($child));
            )*
            arr
        }
    };
}

// pub trait Component {
//     // type Properties: Properties;
// }

// macro_rules! quick {
//     (VNodeChild::$next:) => {
//         VNodeChild::$next
//     };
//     ($x:expr)=>{
//         react!($x)
//     }
// }
    // ($(<$tag_start:ident>$children:tt</$tag_end:ident>)* ) => {
    //     {
    //         let mut arr = vec![];
    //         $(
    //             arr.push(vnode::create_base_element(
    //                 stringify!($tag_start),
    //                 json! {{}},
    //                 vnode::VNode::from(html!($children)),
    //             ));
    //         )*
    //         arr
    //     }
    // };
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
