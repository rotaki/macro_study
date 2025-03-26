# Notes

## Tools

* `cargo-expand` shows the expanded code of a macro
* `#![feature(trace_macros)]` to trace macro expansion. Only works on nightly
* `#![feature(log_syntax)]` to log syntax tree of a macro. Only works on nightly
* proc_macro2 for cleaner separation of parsing and tokenizing


## [No panic](https://github.com/dtolnay/no-panic)

* If there is a possibility of failure, a drop will be called. If there is no possibility of failure, a drop will be removed by the optimization of the compiler when used with `core::mem::forget`. So, by compile-time errors into the drop function, we can see whether a function potentially panics or not at compile time.