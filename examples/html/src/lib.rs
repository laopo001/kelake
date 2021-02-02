#![allow(unused)]
use console::log_1;
use kelake::vnode::{format, Component, ComponentUpdate, ToVNodeChild, VNode, VNodeChild};
use kelake_dom::render;
use kelake_macro::react;
// use serde_json::{json, Value};
use js_sys::Function;
use std::io::Write;
use wasm_bindgen::prelude::*;
use web_sys::{console, Document, Element, Node, Text, Window};
#[derive(Debug, Copy, Clone)]
struct Select {
    s: i32,
    props: SelectProps,
}
#[derive(Debug, Copy, Clone)]
struct SelectProps {
    age: i32,
}

enum SelectEvent {
    Connect,
}

impl Component for Select {
    type Props = SelectProps;
    fn create(props: SelectProps, c: Vec<VNodeChild>) -> Self {
        return Self { s: 1, props };
    }
}
impl ComponentUpdate for Select {
    fn update(&mut self, event: String) {
        unsafe {
            console::log_1(&JsValue::from_str(&event));
            if event == "123" {
                self.s += 123;
                console::log_1(&JsValue::from_f64(self.s as f64));
            }
        }
    }
    fn render(&self) -> VNodeChild {
        unsafe {
            // let x = std::mem::transmute::<&Select, *mut Select>(self);
            let mut c = self.clone();
            // let mut b =  self.clone();
            // let z = &mut b;
            // z.write(c).unwrap();
            return react!(<div onClick={
                // console::log_1(&JsValue::from_str("string"));
                // c.update("electEvent::Connect".to_string());
                "123".to_string()
            }>{self.props.age}</div>);
        }
    }
}

impl ToVNodeChild for Select {
    fn to(self) -> VNodeChild {
        unsafe {
            // let x = std::mem::transmute::<*const Select, &mut Select>(self);
            VNodeChild::Component(Box::new(self))
            // Select::render(&self)
        }
    }
}

pub fn start1() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");

    let document: web_sys::Document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let a = react!(<Select age={9999}></Select>);
    render(
        react!(
            <div>
                123<div>qwr{ "asdf" }{a}</div><a href="https://www.baidu.com/">baidu_link</a>
                <button >button</button>
            </div>
        ),
        body.into(),
    );
    Ok(())
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    start1();
    Ok(())
}
