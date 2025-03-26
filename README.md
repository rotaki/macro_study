# Notes

## Tools

* `cargo-expand` shows the expanded code of a macro
* `#![feature(trace_macros)]` to trace macro expansion. Only works on nightly
* `#![feature(log_syntax)]` to log syntax tree of a macro. Only works on nightly
* proc_macro2 for cleaner separation of parsing and tokenizing