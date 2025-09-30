extern crate proc_macro;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
#[allow(unused_imports)]
use syn::{DeriveInput, Ident, parse_macro_input};

#[proc_macro_derive(EbiInputEnum)]
pub fn derive2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;

    let output = quote! {
        impl FromEbiTraitObject for #ident {
            fn from_trait_object(object: EbiInput) -> Result<Box<Self>> {
                match object {
                    EbiInput::String(value, _) => Ok(Box::new(value.parse()?)),
                    _ => Err(anyhow!(
                        "cannot read {} {} as an enum",
                        object.get_type().get_article(),
                        object.get_type()
                    )),
                }
            }
        }
    };
    output.into()
}

#[proc_macro_derive(ActivityKey)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;

    let output = quote! {
        impl crate::activity_key::has_activity_key::HasActivityKey for #ident {
            fn activity_key(&self) -> &ActivityKey {
                &self.activity_key
            }

            fn activity_key_mut(&mut self) -> &mut ActivityKey {
                &mut self.activity_key
            }
        }
    };
    output.into()
}
