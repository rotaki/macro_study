// #![feature(trace_macros)]

#[macro_use]
extern crate hello_world_macro;

#[derive(Hello, UpperCase)]
struct Example;

#[derive(Hello, UpperCase)]
enum Pet {
    Dog,
}

#[public]
struct Foo {
    bar: i32,
    pub baz: i32,
}

#[public2]
struct Foo2 {
    bar: i32,
    pub baz: i32,
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

    let f = Foo { bar: 1, baz: 2 };
    println!("{:?}", f.bar);

    let f2 = Foo2 { bar: 1, baz: 2 };
    println!("{:?}", f2.bar);
}
