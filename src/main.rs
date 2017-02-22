extern crate emval;

use emval::*;

fn main() {
    let alert = JSObj::global("alert");
    alert.call(args!("こんにちは、世界！"));
    println!("Hello, world!");
}
