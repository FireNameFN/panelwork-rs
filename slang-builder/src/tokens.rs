use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::ToTokens;

pub struct VertexAttribute {
    pub location: u32,

    pub binding: u32,

    pub format: String,

    pub offset: u32,
}

pub struct VertexBinding {
    pub binding: u32,

    pub stride: u32,

    pub input_rate: &'static str,
}

impl ToTokens for VertexAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let location = self.location;
        let binding = self.binding;
        let format = TokenStream::from_str(&self.format).unwrap();
        let offset = self.offset;

        tokens.extend(quote::quote! {
            VertexInputAttributeDescription {
                location: #location,
                binding: #binding,
                format: Format::#format,
                offset: #offset
            }
        });
    }
}

impl ToTokens for VertexBinding {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let binding = self.binding;
        let stride = self.stride;
        let input_rate = TokenStream::from_str(self.input_rate).unwrap();

        tokens.extend(quote::quote! {
            VertexInputBindingDescription {
                binding: #binding,
                stride: #stride,
                input_rate: VertexInputRate::#input_rate,
            }
        });
    }
}
