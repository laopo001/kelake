#![allow(unused)]
#[macro_use]
extern crate lazy_static;
use kelake::vnode::{format, Component, PropsValue, ToVNodeChild, VNode, VNodeChild};
use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Node, Text, Window, console};

pub fn render(vnode: VNodeChild, element: Element) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    let child = render_vnode(vnode).expect("error");

    element.append_child(&child).unwrap();
    Ok(())
}

fn render_vnode(mut vnode: VNodeChild) -> Option<Node> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    match vnode {
        VNodeChild::Text(string) => {
            return Some(document.create_text_node(&string).into());
        }

        VNodeChild::Node(mut node) => unsafe {
            let ptr = &node as *const VNode;
            let element = document.create_element(&node.name).unwrap();
            for (key, value) in node.props {
                match value {
                    PropsValue::String(string) => {
                        element.set_attribute(&key, &string);
                    }
                    PropsValue::Task(x) => {
                        let mut rng = rand::thread_rng();
                        console::log_1(&JsValue::from_str(&format!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())));
                    }
                    _ => {}
                }
            }

            for mut x in node.children {
                match (&mut x) {
                    VNodeChild::Node(c_node) => {
                        c_node.set_parent(ptr);
                    }
                    _ => {}
                }
                let html_node = render_vnode(x).unwrap();

                element.append_child(&html_node);
            }
            // element.set_inner_html("Hello from Rust!!!");

            return Some(element.into());
        },
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
