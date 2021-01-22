#![allow(unused)]
use kelake_macro::react;


fn main() {
    let a = react!(<div>asf</div>);
    dbg!(react!(<div>123<div>qwr{ "asdf" }{{a}}</div></div>));
    // dbg!(react! (
    //   <div> a{{b}} </div>
    // ));
    // dbg!( v
    //     a
    // });
}
