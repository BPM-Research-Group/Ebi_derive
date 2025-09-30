extern crate proc_macro;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
#[allow(unused_imports)]
use syn::{DeriveInput, Ident, parse_macro_input};
use syn::{ItemStruct, parse_quote};

//credit to Yandros: https://users.rust-lang.org/t/derive-macro-aggregating-several-derive-macros/80222/11
#[doc(hidden)]
#[proc_macro_attribute]
pub fn __annihilate(_: TokenStream, _: TokenStream) -> TokenStream {
    <_>::default()
}

#[proc_macro_derive(
    EbiInputEnum,
    // attributes(sql_type), /* in case you wanted to allow extra `#[sql_typ>e]` annotations under your derive */
)]
pub fn aggregate_macro(item: TokenStream) -> TokenStream {
    let mut item: ItemStruct = parse_macro_input!(item);
    // ensure the generated item definition does not end up actually emitted.
    item.attrs.push(parse_quote!(
        #[::ebi_derive::__annihilate]
    ));
    // "re"-emit the item with the desired derives
    quote!(
        #[::core::prelude::v1::derive(
            ::ebi_derive::EbiInputEnumFromEbiTraitObject,
            ::strum_macros::VariantNames,
            ::strum_macros::EnumString,
        )]
        #[strum(serialize_all = "lowercase")]
        #item
    )
    .into()
}

#[proc_macro_derive(EbiInputEnumFromEbiTraitObject)]
pub fn derive_from_ebi_trait_object(input: TokenStream) -> TokenStream {
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
