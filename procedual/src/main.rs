// #![feature(trace_macros)]

#[macro_use]
extern crate hello_world_macro;

#[derive(Hello, UpperCase)]
struct Example;

#[derive(Hello, UpperCase)]
enum Pet {
    Dog,
}

fn main() {
    //trace_macros!(true);
    let e = Example;
    // trace_macros!(false);
    e.hello_world();
    println!("{}", e.uppercase());

    let d = Pet::Dog;
    d.hello_world();
    println!("{}", d.uppercase());
}
