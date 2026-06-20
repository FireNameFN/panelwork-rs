use proc_macro_crate::FoundCrate;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, Ident, Type};

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

pub fn get_thermal_crate() -> TokenStream {
    let thermal_crate = proc_macro_crate::crate_name("thermal").unwrap();

    match &thermal_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(name, Span::call_site());

            quote!(::#ident)
        }
    }
}
