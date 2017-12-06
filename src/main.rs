extern crate emval;

use emval::*;

fn main() {
    let window = JSObj::global("window");
    // JSObj::global("confirm").call(args!("What is your name?")) でも可
    let name: String = window.call_prop("prompt", args!("What is your name?"));
    window.call_prop::<()>("alert", args!(format!("Hi, {}!", name)));
}
