use std::any::Any;
use std::collections::HashMap;
use std::convert::From;
use std::rc::Rc;

type Task = Rc<dyn FnMut()>;
#[derive(Clone)]
pub struct VNode {
    pub name: String,
    pub props: Vec<(String, String)>,
    // pub listeners: HashMap<String, Task>,
    pub children: Vec<VNodeChild>,
}
impl VNode {
    pub fn new<T: IntoIterator<Item = impl ToVNodeChild>>(
        name: String,
        props: Vec<(String, String)>,
        children: T,
    ) -> Self {
        VNode {
            name,
            props,
            // listeners: HashMap::new(),
            children: children.into_iter().map(|x| (x).to()).collect(),
        }
    }
}
impl std::fmt::Debug for VNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VNode")
            .field("name", &self.name)
            .field("props", &self.props)
            // .field("listeners", &"listeners".to_string())
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum VNodeChild {
    Text(String),
    Node(VNode),
    NodeList(Vec<VNodeChild>),
}

pub trait Component<T> {
    fn create(props: T, children: Vec<VNodeChild>) -> Self;
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
// impl ToVNodeChild for i32 {}
// impl ToVNodeChild for i64 {}
// impl ToVNodeChild for f32 {}
// impl ToVNodeChild for f64 {}
// impl ToVNodeChild for String {}
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
