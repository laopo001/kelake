#![allow(unused)]
use kelake::vnode::{format, Component, ToVNodeChild, VNode, VNodeChild};
use kelake_dom::render;
use kelake_macro::react;
// use serde_json::{json, Value};
use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::{console, Document, Element, Node, Text, Window};
#[derive(Copy, Clone)]
struct Select {
    s: i32,
    props: SelectProps,
}
#[derive(Copy, Clone)]
struct SelectProps {
    age: i32,
}

enum SelectEvent {
    Connect,
}

unsafe impl Send for Select {}

impl Component<SelectProps> for Select {
    type Message = SelectEvent;
    fn create(props: SelectProps, c: Vec<VNodeChild>) -> Self {
        return Self { s: 1, props };
    }
    fn update(&mut self, event: Self::Message) {
        match event {
            SelectEvent::Connect => {
                panic!("123")
            }
            _ => {
                panic!("123")
            }
        }
    }
    fn render(&self) -> VNodeChild {
        unsafe {
            let x = std::mem::transmute::<&Select, *mut Select>(self);
            return react!(<div onClick={move || {
                // (*x).s = 3;
                console::log_1(&JsValue::from_str("string"));
            }}>{self.props.age}</div>);
        }
    }
}

impl ToVNodeChild for Select {
    fn to(&self) -> VNodeChild {
        unsafe {
            // let x = std::mem::transmute::<*const Select, &mut Select>(self);
            Select::render(self)
        }
    }
}

pub fn start1() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");

    let document: web_sys::Document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let a = react!(<Select age={9999}></Select>);
    // render(react!(
    //     <div>
    //         123<div>qwr{ "asdf" }{a}</div><a href="https://www.baidu.com/">baidu_link</a>
    //         <button >button</button>
    //     </div>
    // ),body.into());
    Ok(())
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    start1();
    Ok(())
}
