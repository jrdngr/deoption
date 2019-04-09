use crate::proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Deoption)]
pub fn deoption_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let name = &ast.ident;
    let generics = &ast.generics;

    let gen = quote! {
        impl Deoption for #name {
            fn deoption(self) -> Result<_>

        }
    };

    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
