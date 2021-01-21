// use serde_json::Value;
use std::convert::From;

#[derive(Debug,Clone)]
pub struct VNode {
    name:String,
    children:Vec<VNodeChild>,
}
impl VNode {
    pub fn new(name:String, children:Vec<VNodeChild>) -> Self {
        VNode{ name, children }
        // "".to_string()
    }
}

#[derive(Debug,Clone)]
pub enum VNodeChild {
    VText(String),
    VNode(VNode)
}
// impl From<f64> for VNodeChild {
//     fn from(value: f64) -> Self {
//         VNodeChild::Int(value)
//     }
// }

// impl From<&str> for VNodeChild {
//     fn from(value: &str) -> Self {
//         VNodeChild::VText(value.to_string())
//     }
// }


// impl From<bool> for VNodeChild {
//     fn from(value: bool) -> Self {
//         VNodeChild::Bool(value)
//     }
// }

// impl From<Vec<VNodeChild>> for VNodeChild {
//     fn from(value: Vec<VNodeChild>) -> Self {
//         VNodeChild::List(value)
//     }
// }



#[test]
fn it_works() {
    // VNodeChild::from("123");
    assert_eq!(2 + 2, 4);
}
