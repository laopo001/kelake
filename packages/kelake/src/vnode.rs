use serde_json::Value;
use std::convert::From;

#[derive(Clone)]
pub enum VNode {
    VText(String),
    Int(f64),
    Bool(bool),
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


impl From<bool> for VNode {
    fn from(value: bool) -> Self {
        VNode::Bool(value)
    }
}

impl From<Vec<VNode>> for VNode {
    fn from(value: Vec<VNode>) -> Self {
        VNode::List(value)
    }
}


pub fn create_base_element(base_name: &str, props: Value, children: VNode) {}

#[test]
fn it_works() {
    VNode::from("123");
    assert_eq!(2 + 2, 4);
}
