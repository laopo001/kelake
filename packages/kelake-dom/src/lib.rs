#![allow(unused)]
#[macro_use]
extern crate lazy_static;
use console::log_1;
use js_sys::Function;
use kelake::vnode::{
    format, Component, ComponentUpdate, PropsValue, Task, ToVNodeChild, VNode, VNodeChild,
};
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, ops::DerefMut};
use wasm_bindgen::prelude::*;
use web_sys::{console, Document, Element, Node, Text, Window};

struct App;

unsafe impl Sync for App {}

lazy_static! {
    // static ref ARRAY: Mutex<Vec<HashMap<String, Task>>> = Mutex::new(vec![]);

}
static mut ARRAY: Vec<HashMap<String,*mut Task>> = vec![];

#[wasm_bindgen]
pub fn call_task(task_id: usize, string: &str) {
    unsafe {
        // console::log_1(&JsValue::from_f64(task_id as f64));
        console::log_1(&JsValue::from_str(string));

        if let Some(x) = ARRAY.get_mut(task_id).expect("error").get_mut(string) {
            console::log_1(&JsValue::from_f64(task_id as f64));
            let (string,mut this) = std::ptr::read(*(x));
            // (*(*x)).1.update((*(*x)).0.to_string());
            // let (string, this) = **x;
            // this.deref_mut().update(string.to_string());
            (this).update(string.to_string());
        }
    }
}

pub fn render(mut vnode: VNodeChild, element: Element) -> Result<(), JsValue> {
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
                        let mut map: HashMap<String,*mut Task> = HashMap::new();
                        map.insert(key.to_string(), x);
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
            // element.set_inner_html("Hello from Rust!!!");

            return Some(element.into());
        },
        VNodeChild::NodeList(nodes) => {
            unimplemented!();
        }
        VNodeChild::Component(node) => {
            return render_vnode(&mut node.render());
        }
    }
    return None;
}

fn diff(vnode: VNodeChild, pre_vnode: VNodeChild, mut html_node: Node) {}
