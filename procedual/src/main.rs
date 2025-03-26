// #![feature(trace_macros)]

#[macro_use]
extern crate hello_world_macro;

#[derive(Hello)]
struct Example;

#[derive(Hello)]
enum Pet {
    Dog,
    Cat,
}

fn main() {
    //trace_macros!(true);
    let e = Example;
    // trace_macros!(false);
    e.hello_world();

    let d = Pet::Dog;
    d.hello_world();
}
