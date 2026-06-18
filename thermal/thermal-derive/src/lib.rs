use proc_macro::TokenStream;
use proc_macro_crate::FoundCrate;
use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(ThDeviceHandle)]
pub fn th_device_handle_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_th_device_handle(&ast)
}

fn impl_th_device_handle(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let ty = match &ast.data {
        Data::Struct(data) => {
            &data
                .fields
                .iter()
                .find(|field| field.ident.as_ref().is_some_and(|ident| ident == "handle"))
                .unwrap()
                .ty
        }
        _ => panic!("expected struct"),
    };

    let thermal_crate = proc_macro_crate::crate_name("thermal").unwrap();

    let thermal = match &thermal_crate {
        FoundCrate::Itself => quote! {crate},
        FoundCrate::Name(name) => quote! {::#name},
    };

    let generated = quote! {
        impl #thermal::thvk::handle::ThHandle<#ty> for #name {
            fn handle(&self) -> #ty {
                self.handle
            }
        }

        impl #thermal::thvk::handle::ThDeviceHandle<#ty> for #name {
            fn device(&self) -> &::std::sync::Arc<#thermal::thvk::device::ThDevice> {
                &self.device
            }
        }
    };

    generated.into()
}
