use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::util;

pub fn impl_th_device_handle(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let ty = util::get_handle_ty(&ast.data);

    let thermal = &*util::THERMAL_CRATE;

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
