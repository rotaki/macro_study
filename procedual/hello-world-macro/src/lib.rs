use quote::{quote, ToTokens};
use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, token::Colon, Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed, Ident, Type, Visibility};

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

struct StructField {
    name: Ident,
    // ty: Type,
    ty: Ident,
}

// impl StructField {
//     pub fn new(field: &Field) -> Self {
//         Self {
//             name: field.ident.as_ref().unwrap().clone(),
//             ty: field.ty.clone(),
//         }
//     }
// }

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens);
    }
}

/*
impl Parse for StructField {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _vis :Result<Visibility, _> = input.parse();
        let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();

        Ok(
            StructField { 
                name: list.first().unwrap().clone(),
                ty: list.last().unwrap().clone()
            }
        )
    }
}
*/

impl Parse for StructField {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let first = input.cursor().ident().unwrap();
        let res = if first.0.to_string().contains("pub") {
            let second = first.1.ident().unwrap();
            let third = second.1.punct().unwrap().1.ident().unwrap();
            StructField {
                name: second.0,
                ty: third.0,
            }
        } else {
            let second = first.1.punct().unwrap().1.ident().unwrap();
            StructField {
                name: first.0,
                ty: second.0,
            }
        };
        let _: Result<proc_macro2::TokenStream, _> = input.parse();
        Ok(res)
    }
}

#[proc_macro_attribute]
pub fn public2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    eprintln!("{:#?}", ast);
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

    eprintln!("{:#?}", fields);

    // let builder_fields =
    //     fields.iter().map(StructField::new);
    let builder_fields = 
        fields.iter().map(|f| syn::parse2::<StructField>(f.to_token_stream()).unwrap());

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields),*
        }
    };

    public_version.into()
}
