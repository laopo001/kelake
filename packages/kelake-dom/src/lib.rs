#![allow(unused)]
#[macro_use]
extern crate lazy_static;
use kelake::vnode::{format, Component, ToVNodeChild, VNode, VNodeChild};
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Node, Text, Window};

pub fn render(vnode: VNodeChild, element: Element) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    let child = render_vnode(vnode).expect("error");

    element.append_child(&child).unwrap();
    Ok(())
}



fn render_vnode(vnode: VNodeChild) -> Option<Node> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    match vnode {
        VNodeChild::Text(string) => {
            return Some(document.create_text_node(&string).into());
        }

        VNodeChild::Node(node) => {
            let element = document.create_element(&node.name).unwrap();
            for x in node.children {
                let html_node = render_vnode(x).unwrap();
                // for (key, value) in x.
                element.append_child(&html_node);
            }
            // element.set_inner_html("Hello from Rust!!!");

            return Some(element.into());
        }
        VNodeChild::NodeList(nodes) => {
            unimplemented!();
        //    for x in nodes {
        //     let html_node = render_vnode(x).unwrap();
        //     match html_node {
        //         HtmlNode::Text(text) => {
        //             element.append_child(&text.into());
        //         }
        //         HtmlNode::Element(e) => {
        //             element.append_child(&e.into());
        //         }
        //     }
        //    }
        }
    }
    return None;
}
