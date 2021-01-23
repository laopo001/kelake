use impls::impls;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;
use std::convert::From;
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    NodeList(Vec<VNodeChild>),
}

fn type_name<T>(_: T) -> String {
    unsafe { std::intrinsics::type_name::<T>().to_string() }
}

impl std::fmt::Display for VNodeChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "____kelake::vnode::VNodeChild")
    }
}

// pub fn format2<T: std::fmt::Display >(child: T) -> VNodeChild {
//     // println!( "{}", type_name(child.clone()));
//     let f = format!("{}", child);
//     if f == "____kelake::vnode::VNodeChild" {
//         unsafe {
//             let q = std::mem::transmute::<&T, *const VNodeChild>(&child);
//             (*q).clone()
//             // std::mem::transmute::<T, Any>(child);
//             // VNodeChild::Text(f.to_string())
//         }
//     } else {
//         VNodeChild::Text(f.to_string())
//     }
// }

pub trait ToVNodeChild {
    fn to(&self) -> VNodeChild;
}

macro_rules! tov {
    ( ($t:tt) ) => {
        impl ToVNodeChild for $t {
            fn to(&self) -> VNodeChild {
                VNodeChild::Text(format!("{}", self))
            }
        }
    };
}


tov!((i32));
tov!((i64));
tov!((f32));
tov!((f64));
tov!((String));
tov!((bool));
// impl ToVNodeChild for i32 {}
// impl ToVNodeChild for i64 {}
// impl ToVNodeChild for f32 {}
// impl ToVNodeChild for f64 {}
// impl ToVNodeChild for String {}
impl ToVNodeChild for &str {
    fn to(&self) -> VNodeChild {
        VNodeChild::Text(format!("{}", self))
    }
}

impl ToVNodeChild for dyn std::fmt::Display {
    fn to(&self) -> VNodeChild {
        VNodeChild::Text(format!("{}", self))
    }
}

impl ToVNodeChild for VNodeChild {
    fn to(&self) -> VNodeChild {
        unsafe { self.clone() }
    }
}

impl ToVNodeChild for Vec<VNodeChild> {
    fn to(&self) -> VNodeChild {
        VNodeChild::NodeList(unsafe { self.clone() })
    }
}

pub fn format(s: &dyn ToVNodeChild) -> VNodeChild {
    s.to()
    // if let Some(t) = s.downcast_ref::<String>() {
    //     VNodeChild::Text(t.to_string())
    // } else if let Some(v) = s.downcast_ref::<Vec<VNodeChild>>() {
    //     VNodeChild::NodeList(v.clone())
    // } else {
    //     ToVNodeChild::to(s);
    // }
}

macro_rules! attribute {
    ( $e:expr ) => {
        if let Some(f) = (&$e as &Any).downcast_ref::<f32>() {
            println!("`{}` is f32.", f);
        } else if let Some(f) = (&$e as &Any).downcast_ref::<f64>() {
            println!("`{}` is f64.", f);
        } else {
            println!("I dunno what is `{:?}` :(", $e);
        }
    };
}
