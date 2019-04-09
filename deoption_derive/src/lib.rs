extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Path, Type};

#[proc_macro_derive(Deoption)]
pub fn deoption_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let generics = &ast.generics;

    if let Data::Struct(ds) = &ast.data {
        if let Fields::Named(nf) = &ds.fields {
            for field in &nf.named {
                if let Type::Path(tp) = &field.ty {
                    if let Some(segment) = &tp.path.segments.last() {
                        let ty = segment.value().ident.to_string();
                        if ty == "Option" {
                            println!("this be an option!");
                        }
                    }
                }
            }
        }
    }

    let gen = quote! {
        impl Deoption for #name {
            fn deoption<T>(self) -> Result<T, DeoptionError> {
                Err(DeoptionError::new(vec!["ok".to_owned()]))
            }

        }
    };

    gen.into()
}
