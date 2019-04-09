extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Fields, Path, Type};

#[proc_macro_derive(Deoption)]
pub fn deoption_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let generics = &ast.generics;

    let mut blocks = Vec::new();
    blocks.push(quote!{});

    if let Data::Struct(ds) = &ast.data {
        if let Fields::Named(nf) = &ds.fields {
            for field in &nf.named {
                if let Type::Path(tp) = &field.ty {
                    if let Some(segment) = &tp.path.segments.last() {
                        let ty = segment.value().ident.to_string();
                        if ty == "Option" {
                            if let Some(id) = &field.ident {
                                let ident_name = format!("self.{}", id.to_string());
                                let ident = Ident::new(&ident_name, id.span());
                                blocks.push(quote! {
                                    if #ident.is_none() {
                                        missing.push(#ident.to_string());
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    let gen = quote! {
        impl Deoption for #name {
            fn deoption<T>(self) -> Result<T, DeoptionError> {
                let mut missing = Vec::new();

                #(#blocks);*

                Err(DeoptionError::new(missing))
            }

        }
    };

    gen.into()
}
