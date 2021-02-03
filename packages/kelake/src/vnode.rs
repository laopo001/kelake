use std::any::Any;
use std::collections::HashMap;
use std::convert::From;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
pub type Task = Box<(String,Box<dyn ComponentUpdate>)>;
// pub type Task = Rc<dyn FnMut()>;

#[derive()]
pub enum PropsValue {
    String(String),
    Task(Task),
}

impl std::fmt::Debug for PropsValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PropsValue")
            .field({
                match self {
                    PropsValue::String(string) => string,
                    PropsValue::Task(task) => &stringify!(task),
                }
            })
            .finish()
    }
}

#[derive(Debug)]
pub struct VNode {
    pub name: String,
    pub props: HashMap<String, PropsValue>,
    pub parent: *const VNode,
    pub children: Vec<VNodeChild>,
}
impl VNode {
    pub fn new<T: IntoIterator<Item = impl ToVNodeChild>>(
        name: String,
        props: HashMap<String, PropsValue>,
        children: T,
    ) -> Self {
        VNode {
            name,
            props,
            parent: std::ptr::null(),
            children: children.into_iter().map(|x| (x).to()).collect(),
        }
    }
    pub fn set_parent(&mut self, p: *const VNode) {
        self.parent = p;
    }
}

#[derive(Debug)]
pub enum VNodeChild {
    Text(String),
    Node(VNode),
    NodeList(Vec<VNodeChild>),
    Component(Box<dyn ComponentUpdate>),
}


pub trait Component: Sized + 'static {
    type Props;
    fn create(props: Self::Props, children: Vec<VNodeChild>) -> Self;
}

pub trait ComponentUpdate: std::fmt::Debug {
    fn update(&mut self, string: String);
    fn render(&self) -> VNodeChild;
}

// fn type_name<T>(_: T) -> String {
//     unsafe { std::intrinsics::type_name::<T>().to_string() }
// }

// impl std::fmt::Display for VNodeChild {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", "____kelake::vnode::VNodeChild")
//     }
// }

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
    fn to(self) -> VNodeChild;
}

macro_rules! tov {
    ( ($t:tt) ) => {
        impl ToVNodeChild for $t {
            fn to(self) -> VNodeChild {
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

impl ToVNodeChild for &str {
    fn to(self) -> VNodeChild {
        VNodeChild::Text(format!("{}", self))
    }
}

impl ToVNodeChild for VNodeChild {
    fn to(self) -> VNodeChild {
        unsafe { self }
    }
}

impl ToVNodeChild for Vec<VNodeChild> {
    fn to(self) -> VNodeChild {
        VNodeChild::NodeList(unsafe { self })
    }
}

pub fn format(s: impl ToVNodeChild) -> VNodeChild {
    s.to()
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
