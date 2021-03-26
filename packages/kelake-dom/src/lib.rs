#![allow(unused)]
#[macro_use]
extern crate lazy_static;
use console::log_1;
use js_sys::Function;
use kelake::vnode::{
    format, Component, ComponentUpdate, PropsValue, Task, ToVNodeChild, VNode, VNodeChild,
};

use rand::Rng;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use std::{
    sync::{Arc, Mutex},
    vec,
};
use wasm_bindgen::prelude::*;
use web_sys::{console, Document, Element, Node, Text, Window};
// use web_sys::features::gen_Node;
struct App;

unsafe impl Sync for App {}

lazy_static! {
    // static ref ARRAY: Mutex<Vec<HashMap<String, Task>>> = Mutex::new(vec![]);
}
static mut ARRAY: Vec<HashMap<String, Task>> = vec![];
static mut VNODE: VNodeChild = VNodeChild::NodeList(vec![]);
#[wasm_bindgen]
pub fn call_task(task_id: usize, string: &str) {
    unsafe {
        // console::log_1(&JsValue::from_f64(task_id as f64));
        // console::log_1(&JsValue::from_str(string));
        let map = ARRAY.get_mut(task_id).expect("msg");
        let task = map.get_mut(string).expect("msg");
        // console::log_1(&JsValue::from_str(&format!("{:?}",task.borrow_mut().1)));
        let s = task.borrow_mut().0.to_string();
        task.borrow_mut().1.expect("msg").as_mut().update(s);
    }
}

pub fn render(mut vnode: VNodeChild, element: Element) -> Result<(), JsValue> {
    unsafe {
        let window = web_sys::window().expect("no global `window` exists");
        let document: Document = window.document().expect("should have a document on window");
        let child = render_vnode(&mut vnode).expect("error");

        element.append_child(&child).unwrap();
        document.add_event_listener_with_callback(
        "click",
        &Function::new_with_args("e", " try{ 
            if(e.target.attributes[ 'on' + e.type ]){
                call_task(e.target.attributes[ 'on' + e.type ].nodeValue, 'on' + e.type.slice(0,1).toUpperCase() + e.type.slice(1,e.type.length)); 
            }
            } catch (e){
                console.error(e);
            }"),
        );
        Ok(())
    }
}

fn render_vnode(vnode: &mut VNodeChild) -> Option<Node> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    match vnode {
        VNodeChild::Text(string) => {
            return Some(document.create_text_node(&string).into());
        }

        VNodeChild::Node(node) => unsafe {
            let ptr = node as *const VNode;
            let element = document.create_element(&node.name).unwrap();
            for (key, value) in node.props.iter_mut() {
                match value {
                    PropsValue::String(string) => {
                        element.set_attribute(&key, &string);
                    }
                    PropsValue::Task(x) => {
                        // let mut rng = rand::thread_rng();
                        // console::log_1(&JsValue::from_str(&format!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())));
                        // let mut mut_arr = ARRAY.lock().expect("error");
                        let mut map: HashMap<String, Task> = HashMap::new();
                        map.insert(key.to_string(), x.clone());
                        ARRAY.push(map);
                        element.set_attribute(&key, &(ARRAY.len() - 1).to_string());
                    }
                    _ => {}
                }
            }

            for mut x in node.children.iter_mut() {
                match (&mut x) {
                    VNodeChild::Node(c_node) => {
                        c_node.set_parent(ptr);
                    }
                    _ => {}
                }
                let html_node = render_vnode(x).unwrap();

                element.append_child(&html_node);
            }

            return Some(element.into());
        },
        VNodeChild::NodeList(nodes) => {
            unimplemented!();
        }
        VNodeChild::Component(component) => {
            return render_vnode(&mut component.render());
        }
    }
    return None;
}

fn diff(vnode: &mut VNodeChild, pre_vnode: &VNodeChild, mut html_node: Element) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    match vnode {
        VNodeChild::Text(string) => {
            match pre_vnode {
                VNodeChild::Text(pre_string) => {
                    if (string != pre_string) {
                        html_node.set_text_content(Some(&string));
                    }
                }
                _ => {
                    unimplemented!();
                }
            };
        }

        VNodeChild::Node(node) => unsafe {
            match pre_vnode {
                VNodeChild::Node(pre_node) => unsafe {
                    if (node.name != pre_node.name) {
                        let ptr = node as *const VNode;
                        let element = document.create_element(&node.name).unwrap();
                        for (key, value) in node.props.iter_mut() {
                            match value {
                                PropsValue::String(string) => {
                                    element.set_attribute(&key, &string);
                                }
                                PropsValue::Task(x) => {
                                    // let mut rng = rand::thread_rng();
                                    // console::log_1(&JsValue::from_str(&format!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())));
                                    // let mut mut_arr = ARRAY.lock().expect("error");
                                    let mut map: HashMap<String, Task> = HashMap::new();
                                    map.insert(key.to_string(), x.clone());
                                    ARRAY.push(map);
                                    element.set_attribute(&key, &(ARRAY.len() - 1).to_string());
                                }
                                _ => {}
                            }
                        }

                        for mut i in 0..node.children.len() {
                            let x = &mut node.children[i];
                            match (&mut x) {
                                VNodeChild::Node(c_node) => {
                                    c_node.set_parent(ptr);
                                }
                                _ => {}
                            }
                            let y = node.children.get_mut(i);
                            let z = (html_node).child_nodes[i];
                            if y.is_none() {
                                z.remove();
                                return;
                            }
                            diff(x, y.unwrap(), z);
                        }
                    } else {
                        let element = html_node;
                        for (key, value) in node.props.iter_mut() {
                            match value {
                                PropsValue::String(string) => {
                                    element.set_attribute(&key, &string);
                                }
                                PropsValue::Task(x) => {
                                    // let mut rng = rand::thread_rng();
                                    // console::log_1(&JsValue::from_str(&format!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())));
                                    // let mut mut_arr = ARRAY.lock().expect("error");
                                    let mut map: HashMap<String, Task> = HashMap::new();
                                    map.insert(key.to_string(), x.clone());
                                    ARRAY.push(map);
                                    element.set_attribute(&key, &(ARRAY.len() - 1).to_string());
                                }
                                _ => {}
                            }
                        }
                    }
                },
                _ => {
                    unimplemented!();
                }
            };
        },
        VNodeChild::NodeList(nodes) => {
            unimplemented!();
        }
        VNodeChild::Component(component) => {
            unimplemented!();
        }
    }
}
