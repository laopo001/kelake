use crate::vnode::VNodeChild;

pub fn render(c:VNodeChild,id:&str){
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let body = document.body().expect("document should have a body");
}