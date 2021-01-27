#![allow(unused)]
use kelake::vnode::{format, Component, ToVNodeChild, VNode, VNodeChild};
use kelake_dom::render;
use kelake_macro::react;
// use serde_json::{json, Value};
use wasm_bindgen::prelude::*;
use js_sys::Function;

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
        return react!(<div>{self.props.age}</div>);
    }
}

impl ToVNodeChild for Select {
    fn to(self) -> VNodeChild {
        self.render()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn my_function()  {
    unsafe { 
        log("123123");
    }
    
}


pub fn start1() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");

    let document: web_sys::Document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
 
    let a = react!(<Select age={9999}></Select>);
    render(react!(
        <div>
            123<div>qwr{ "asdf" }{a}</div><a href="https://www.baidu.com/">baidu_link</a>
            <button onclick="my_function()">button</button>
        </div>
    ),body.into());
    Ok(())
}


pub fn start2() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document: web_sys::Document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    document.add_event_listener_with_callback("click", &Function::new_with_args("e"," console.log(e) "));

    render(react!(
        <div>
            <button onClick={||{
                
            }}>button</button>
        </div>
    ),body.into());
    Ok(())
}



#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    start2();
    Ok(())
}