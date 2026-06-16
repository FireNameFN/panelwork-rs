use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        self, BlendFactor, BlendOp, ColorComponentFlags, DynamicState, GraphicsPipelineCreateInfo,
        Pipeline, PipelineCache, PipelineColorBlendAttachmentState,
        PipelineColorBlendStateCreateInfo, PipelineDynamicStateCreateInfo,
        PipelineInputAssemblyStateCreateInfo, PipelineMultisampleStateCreateInfo,
        PipelineRasterizationStateCreateInfo, PipelineShaderStageCreateInfo,
        PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateInfo, PrimitiveTopology,
        RenderPass, SampleCountFlags, ShaderStageFlags, VertexInputAttributeDescription,
        VertexInputBindingDescription,
    },
};

use crate::thvk::{pipeline_layout::ThPipelineLayout, shader_module::ThShaderModule};

pub struct ThPipeline {
    pub handle: Pipeline,

    pub layout: Arc<ThPipelineLayout>,

    pub shader_modules: [Arc<ThShaderModule>; 2],
}

pub struct GraphicsPipelineSettings<'a> {
    pub vertex_shader: Arc<ThShaderModule>,

    pub fragment_shader: Arc<ThShaderModule>,

    pub vertex_bindings: &'a [VertexInputBindingDescription],

    pub vertex_attributes: &'a [VertexInputAttributeDescription],

    pub samples: SampleCountFlags,

    pub sample_shading: Option<f32>,
}

impl ThPipelineLayout {
    pub fn create_graphics_pipeline(
        self: &Arc<ThPipelineLayout>,
        render_pass: RenderPass,
        settings: GraphicsPipelineSettings,
    ) -> VkResult<ThPipeline> {
        let stages_info = [
            PipelineShaderStageCreateInfo {
                stage: ShaderStageFlags::VERTEX,
                module: settings.vertex_shader.handle,
                p_name: c"main".as_ptr(),
                ..Default::default()
            },
            PipelineShaderStageCreateInfo {
                stage: ShaderStageFlags::FRAGMENT,
                module: settings.fragment_shader.handle,
                p_name: c"main".as_ptr(),
                ..Default::default()
            },
        ];

        let vertex_info = PipelineVertexInputStateCreateInfo {
            vertex_binding_description_count: settings.vertex_bindings.len() as u32,
            p_vertex_binding_descriptions: settings.vertex_bindings.as_ptr(),
            vertex_attribute_description_count: settings.vertex_attributes.len() as u32,
            p_vertex_attribute_descriptions: settings.vertex_attributes.as_ptr(),
            ..Default::default()
        };

        let assembly_info = PipelineInputAssemblyStateCreateInfo {
            topology: PrimitiveTopology::TRIANGLE_LIST,
            ..Default::default()
        };

        let viewport_info = PipelineViewportStateCreateInfo {
            viewport_count: 1,
            scissor_count: 1,
            ..Default::default()
        };

        let rasterization_info = PipelineRasterizationStateCreateInfo {
            line_width: 1.,
            ..Default::default()
        };

        let multisample_info = PipelineMultisampleStateCreateInfo {
            rasterization_samples: settings.samples,
            sample_shading_enable: if settings.sample_shading.is_some() {
                vk::TRUE
            } else {
                vk::FALSE
            },
            min_sample_shading: settings.sample_shading.unwrap_or_default(),
            ..Default::default()
        };

        let blend_attachment = PipelineColorBlendAttachmentState {
            blend_enable: vk::TRUE,
            src_color_blend_factor: BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: BlendOp::ADD,
            src_alpha_blend_factor: BlendFactor::ONE,
            dst_alpha_blend_factor: BlendFactor::ONE_MINUS_SRC_ALPHA,
            alpha_blend_op: BlendOp::ADD,
            color_write_mask: ColorComponentFlags::RGBA,
        };

        let blend_info = PipelineColorBlendStateCreateInfo {
            attachment_count: 1,
            p_attachments: &blend_attachment,
            ..Default::default()
        };

        let dynamic_states = [DynamicState::VIEWPORT, DynamicState::SCISSOR];

        let dynamic_info = PipelineDynamicStateCreateInfo {
            dynamic_state_count: dynamic_states.len() as u32,
            p_dynamic_states: dynamic_states.as_ptr(),
            ..Default::default()
        };

        let pipeline_info = GraphicsPipelineCreateInfo {
            stage_count: stages_info.len() as u32,
            p_stages: stages_info.as_ptr(),
            p_vertex_input_state: &vertex_info,
            p_input_assembly_state: &assembly_info,
            p_viewport_state: &viewport_info,
            p_rasterization_state: &rasterization_info,
            p_multisample_state: &multisample_info,
            p_color_blend_state: &blend_info,
            p_dynamic_state: &dynamic_info,
            layout: self.handle,
            render_pass: render_pass,
            ..Default::default()
        };

        let handles = unsafe {
            self.device.handle.create_graphics_pipelines(
                PipelineCache::null(),
                &[pipeline_info],
                None,
            )
        }
        .map_err(|e| e.1)?;

        Ok(ThPipeline {
            handle: handles[0],
            layout: self.clone(),
            shader_modules: [settings.vertex_shader, settings.fragment_shader],
        })
    }
}

impl Drop for ThPipeline {
    fn drop(&mut self) {
        unsafe {
            self.layout
                .device
                .handle
                .destroy_pipeline(self.handle, None)
        }
    }
}
