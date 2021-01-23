use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::convert::From;
#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct VNode {
    name: String,
    props: Value,
    children: Vec<VNodeChild>,
}
impl VNode {
    pub fn new(name: String, props: Value, children: Vec<VNodeChild>) -> Self {
        VNode {
            name,
            props,
            children,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VNodeChild {
    Text(String),
    Node(VNode),
}

impl std::fmt::Display for VNodeChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "___VNodeChild___")
    }
}

pub fn format<T: std::fmt::Display>(child: T) -> VNodeChild {
    let f = format!("{}", child);
    if f == "___VNodeChild___" {
        unsafe {
            let q = std::mem::transmute::<&T, *const VNodeChild>(&child);
            (*q).clone()
        }
    } else {
        VNodeChild::Text(f.to_string())
    }
}
