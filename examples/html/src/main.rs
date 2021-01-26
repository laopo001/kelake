#![allow(unused)]
use kelake::vnode::{format, Component, ToVNodeChild, VNode, VNodeChild};
use kelake_macro::react;
// use serde_json::{json, Value};

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

fn main() {
    // let a = Select {
    //     props: SelectProps { age: 1 },
    //     s: 1,
    // };
    // let s = a.render;
    // let a:Vec<impl ToVNodeChild> = vec![]
    // a.push("value");
    // a.push(1);
    // let mut b = json!({"a":"123","b":"234"}).as_object().unwrap();
    // let mut c = json!({"d":"123"}).as_object().unwrap();
    // b.extend(c.iter());
    // for (key, value) in b.as_object().unwrap() {
    //     dbg!(key, value);
    // }
    // println!("======================");
    // let a ="123";
    // dbg!(type_name(&react!(<div>{a}</div>)));
    // let a = vec![ react!(<Select age={9999}></Select>) ] ;
    let a = react!(<Select age={9999}></Select>);
    let b = "1";
    // dbg!(to_vnode_child_vec!(a));
    dbg!(react! {
        <div onClick="">123<div>qwr{ "asdf" }{a}</div>123</div>
    });

    // let a = "asdf";
    // dbg!(react! {
    //  <Ga style={"safdasdf"}> <div>{a}</div> </Ga>
    // });
    // dbg!( v
    //     a
    // });
}
