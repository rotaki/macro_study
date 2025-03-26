use quote::quote;
use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    println!("{}", item);
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    println!("{}", name);
    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello, World");
            }
        }
    };
    add_hello_world.into()
}

#[proc_macro_derive(UpperCase)]
pub fn uppercase(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let add_uppercase = quote! {
        impl #name {
            fn uppercase(&self) -> String {
                stringify!(#name).to_uppercase()
            }
        }
    };
    add_uppercase.into()
}