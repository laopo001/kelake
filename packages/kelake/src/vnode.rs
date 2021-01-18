use serde_json::Value;
use std::convert::From;

#[derive(Clone)]
pub enum VNode {
    VText(String),
    Int(f64),
    List(Vec<VNode>),
}
impl From<f64> for VNode {
    fn from(value: f64) -> Self {
        VNode::Int(value)
    }
}

impl From<&str> for VNode {
    fn from(value: &str) -> Self {
        VNode::VText(value.to_string())
    }
}

// trait ToF64 {
//     fn to(&self) -> f64;
// }

pub fn create_base_element(base_name: &str, props: Value, children: VNode) {}

#[test]
fn it_works() {
    VNode::from("123");
    assert_eq!(2 + 2, 4);
}
