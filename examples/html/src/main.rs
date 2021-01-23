#![allow(unused)]
use kelake_macro::react;
use serde_json::{json, Value};
fn  format() -> String {
    "".to_string()
}

fn main() {
    // let a = react!(<div>asf</div>);
    // dbg!(react!(<div>123<div>qwr{ "asdf" }{{a}}</div></div>));

    let a = "asdf";
    dbg!(react! {
      <a src={a}> {a}</a>
     } );
    // dbg!( v
    //     a
    // });
}
