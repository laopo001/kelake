#![allow(unused)]
#![feature(core_intrinsics)]
use kelake::vnode::{format, Component, ToVNodeChild, VNode, VNodeChild};
use kelake_macro::react;
use serde_json::{json, Value};
// trait TypeName {
//     fn process(&self) -> String;
// }

// impl TypeName for f32 {
//     fn process(&self) -> String { "f32".to_string() }
// }

// impl TypeName for i64 {
//     fn process(&self) -> String { "i64".to_string() }
// }

// macro_rules! get_type {
//     ($e:expr) => { TypeName::process(&$e) };
// }
fn type_name<T>(_: T) -> String {
    unsafe { std::intrinsics::type_name::<T>().to_string() }
}

struct Select {
    s: i32,
    props: SelectProps,
}
struct SelectProps {
    age: i32,
}

impl Component<SelectProps> for Select {
    fn create(props: SelectProps) -> Self {
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

fn main() {
    // let mut b = json!({"a":"123","b":"234"}).as_object().unwrap();
    // let mut c = json!({"d":"123"}).as_object().unwrap();
    // b.extend(c.iter());
    // for (key, value) in b.as_object().unwrap() {
    //     dbg!(key, value);
    // }
    // println!("======================");
    // let a ="123";
    // dbg!(type_name(&react!(<div>{a}</div>)));
    let a = react!(<Select age={9999}></Select>);
    dbg!(react!(<div>123<div>qwr{ "asdf" }{{a}}</div>123</div>));

    // let a = "asdf";
    // dbg!(react! {
    //  <Ga style={"safdasdf"}> <div>{a}</div> </Ga>
    // });
    // dbg!( v
    //     a
    // });
}
