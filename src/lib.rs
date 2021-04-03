use hello::hello_h::Hello;

pub mod hello;

pub fn run() {
    let hello = Hello::new("world");
    println!("{}", hello.speak());
}
