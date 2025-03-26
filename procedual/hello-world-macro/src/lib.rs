use quote::quote;
use proc_macro::{TokenStream, TokenTree};
use syn::{Data::Struct, Fields::Named, parse_macro_input, DataStruct, DeriveInput, FieldsNamed};

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

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        Struct(
            DataStruct {
                fields: Named(
                    FieldsNamed {
                        ref named, ..
                    }
                ), ..
            }
        ) => named,
        _ => unimplemented!("Only works for structs with named fields")
    };

    let builder_fields = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {
            pub #name: #ty
        }
    });

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields),*
        }
    };

    public_version.into()
}