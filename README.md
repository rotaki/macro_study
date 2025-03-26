# Notes

## Tools

* `cargo-expand` shows the expanded code of a macro
* `#![feature(trace_macros)]` to trace macro expansion. Only works on nightly
* `#![feature(log_syntax)]` to log syntax tree of a macro. Only works on nightly
* proc_macro2 for cleaner separation of parsing and tokenizing
* `syn = {version = "2.0", features = ["extra-traits"]}` to print a debug representation of the syntax tree


## Macros case studies
### [No panic](https://github.com/dtolnay/no-panic)

* If there is a possibility of `panic`, a drop function of a object allocated before the panic will be called. If there is no possibility of `panic`s, the drop function will be removed by the compiler optimization when `core::mem::forget` is declared. So, by injecting compile-time errors into the drop function, we can see whether a function potentially panics or not at compile time.