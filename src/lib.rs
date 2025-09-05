extern crate proc_macro;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
#[allow(unused_imports)]
use syn::{parse_macro_input, DeriveInput, Ident};

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