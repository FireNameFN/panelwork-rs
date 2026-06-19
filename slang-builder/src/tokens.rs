use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};

pub struct VertexBinding {
    pub binding: u32,

    pub stride: u32,

    pub input_rate: &'static str,
}

impl ToTokens for VertexBinding {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let binding = self.binding;
        let stride = self.stride;
        let input_rate = Ident::new(self.input_rate, Span::call_site());

        tokens.extend(quote! {
            VertexInputBindingDescription {
                binding: #binding,
                stride: #stride,
                input_rate: VertexInputRate::#input_rate,
            }
        });
    }
}

pub struct VertexAttribute {
    pub location: u32,

    pub binding: u32,

    pub format: String,

    pub offset: u32,
}

impl ToTokens for VertexAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let location = self.location;
        let binding = self.binding;
        let format = Ident::new(&self.format, Span::call_site());
        let offset = self.offset;

        tokens.extend(quote! {
            VertexInputAttributeDescription {
                location: #location,
                binding: #binding,
                format: Format::#format,
                offset: #offset,
            }
        });
    }
}

pub struct DescriptorBinding {
    pub binding: u32,

    pub descriptor_type: &'static str,

    pub descriptor_count: u32,

    pub stage_flags: &'static str,
}

impl ToTokens for DescriptorBinding {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let binding = self.binding;
        let descriptor_type = Ident::new(self.descriptor_type, Span::call_site());
        let descriptor_count = self.descriptor_count;
        let stage_flags = Ident::new(self.stage_flags, Span::call_site());

        tokens.extend(quote! {
            DescriptorSetLayoutBinding {
                binding: #binding,
                descriptor_type: DescriptorType::#descriptor_type,
                descriptor_count: #descriptor_count,
                stage_flags: ShaderStageFlags::#stage_flags,
                p_immutable_samplers: std::ptr::null(),
                _marker: PhantomData,
            }
        });
    }
}
