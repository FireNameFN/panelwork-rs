use proc_macro::TokenStream;

mod device_handle;
mod handle;
mod util;

#[proc_macro_derive(ThHandle)]
pub fn th_handle_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    handle::impl_th_handle(&ast)
}

#[proc_macro_derive(ThDeviceHandle)]
pub fn th_device_handle_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    device_handle::impl_th_device_handle(&ast)
}
