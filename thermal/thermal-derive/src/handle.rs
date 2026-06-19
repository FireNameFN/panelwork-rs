use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::util;

pub fn impl_th_handle(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let ty = util::get_handle_ty(&ast.data);

    let thermal = &*util::THERMAL_CRATE;

    let (generics_impl, generics_ty, generics_where) = ast.generics.split_for_impl();

    let generated = quote! {
        impl #generics_impl #thermal::thvk::handle::ThHandle<#ty> for #name #generics_ty #generics_where {
            fn handle(&self) -> #ty {
                self.handle
            }
        }
    };

    generated.into()
}
