#![allow(unused)]
use kelake::vnode::{format, Component, ToVNodeChild, VNode, VNodeChild};
use kelake_dom::render;
use kelake_macro::react;
// use serde_json::{json, Value};
use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::{console, Document, Element, Node, Text, Window};

struct Select {
    s: i32,
    props: SelectProps,
}
struct SelectProps {
    age: i32,
}

impl Component<SelectProps> for Select {
    fn create(props: SelectProps, c: Vec<VNodeChild>) -> Self {
        return Self { s: 1, props };
    }
    fn render(&self) -> VNodeChild {
        unsafe {
            // let x = std::mem::transmute::<&mut Select, *mut Select>(self);
            return react!(<div onClick={|| unsafe {
                // (*x).s = 3;
                console::log_1(&JsValue::from_str("string"));
            }}>{self.props.age}</div>);
        }
    }
}

impl ToVNodeChild for Select {
    fn to(&self) -> VNodeChild {
        unsafe {
            let x = std::mem::transmute::<*const Select, &mut Select>(self);
            Select::render(x)
        }
    }
}

pub fn start1() -> Result<(), JsValue> {
    let x = 1.to();
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
