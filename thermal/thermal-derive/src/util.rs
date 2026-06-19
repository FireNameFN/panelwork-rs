use std::sync::LazyLock;

use proc_macro_crate::FoundCrate;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, Ident, Type};

pub const THERMAL_CRATE: LazyLock<proc_macro2::TokenStream> = LazyLock::new(|| {
    let thermal_crate = proc_macro_crate::crate_name("thermal").unwrap();

    match &thermal_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(name, Span::call_site());

            quote!(::#ident)
        }
    }
});

pub fn get_handle_ty(data: &Data) -> &Type {
    match data {
        Data::Struct(data) => {
            &data
                .fields
                .iter()
                .find(|field| field.ident.as_ref().is_some_and(|ident| ident == "handle"))
                .unwrap()
                .ty
        }
        _ => panic!("expected struct"),
    }
}
