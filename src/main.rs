extern crate emval;

use emval::*;

fn main() {
    /*let window = JSObj::global("window");
    println!("{:?}", window);
    let alert = window.get_prop("alert");
    println!("{:?}", alert);
    alert.call(args!("Hello, world!"));
    */
    let window = JSObj::global("window");
    window.call_prop::<()>("alert", args!("Hello, world!"));

    println!("Hello, world!");
}
