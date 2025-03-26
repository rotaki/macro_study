#![feature(trace_macros)]
#![feature(log_syntax)]

pub fn base_greeting_fn(name: &str, greeting: &str) -> String {
    format!("{}, {}!", greeting, name)
}

macro_rules! greetings {
    () => {
        base_greeting_fn("world", "Hello")
    };
    ($name:literal) => {
        base_greeting_fn($name, "Hello")
    };
    ($name:literal, $greeting:literal) => {
        base_greeting_fn($name, $greeting)
    };
    (test, $name:literal) => {{
        log_syntax!("The name passed to test is ", $name);
        println!("Returning default greeting");
        base_greeting_fn($name, "Hello")
    }};
}

fn main() {
    trace_macros!(true);
    // let greetings = greetings!();
    // let greetings = greetings!("foo");
    // let greetings = greetings!("bar", "Hi");
    let greetings = greetings!(test, "baz");
    trace_macros!(false);
}
