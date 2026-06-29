use std::sync::Arc;
use std::{ffi::CStr, io::Cursor};

use png::{Decoder, Transformations};
use thermal::ash::vk::{
    self, AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
    ClearColorValue, ClearValue, CommandBufferLevel, CommandBufferUsageFlags,
    CommandPoolCreateFlags, DescriptorPoolSize, DescriptorSet, DescriptorType, Filter, Format,
    ImageLayout, ImageUsageFlags, PipelineBindPoint, PipelineStageFlags, SampleCountFlags,
    SamplerAddressMode, SubpassContents, SubpassDescription,
};
use thermal::core::draw_handle::DrawHandle;
use thermal::glam::{Affine2, Vec4};
use thermal::mesh::rect::Rect;
use thermal::primitives::{Vertex, viewport_matrix};
use thermal::thrwh::surface::ThRwhSurface;
use thermal::thrwh::util::rwh_instance_extensions;
use thermal::thvk::command_buffer::ThCommandBuffer;
use thermal::thvk::command_pool::ThCommandPool;
use thermal::thvk::fence::ThFence;
use thermal::thvk::framebuffer::ThFramebuffer;
use thermal::thvk::handle::ThDeviceHandle;
use thermal::thvk::image_view::ThImageView;
use thermal::thvk::pipeline::ThPipeline;
use thermal::thvk::pipeline_layout::ThPipelineLayout;
use thermal::thvk::queue::ThQueue;
use thermal::thvk::render_pass::ThRenderPass;
use thermal::thvk::swapchain::ThSwapchainImage;
use thermal::{
    core::{atlas::Atlas, command::Command, presenter::Presenter},
    defaults,
    ext::{
        handle::ThHandleDeviceExt, physical_device::ThPhysicalDeviceIteratorExt,
        result::SwapchainResultExt,
    },
    primitives::vk::{rect, viewport},
    thvk::{
        descriptor_set::Binding,
        descriptor_set_layout::ThDescriptorSetLayout,
        device::QueueInfo,
        framebuffer::ThRenderPassSource,
        handle::ThHandle,
        image_view::ThImageViewSource,
        library::ThLibrary,
        pipeline::{GraphicsPipelineSettings, ThPipelineSource},
    },
};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::wayland::WindowAttributesExtWayland;
use winit::raw_window_handle::HasDisplayHandle;
use winit::window::Window;

#[allow(dead_code)]
const IMAGE: &[u8] = include_bytes!("../resources/OverGreen.png");

#[allow(dead_code)]
const IMAGE2: &[u8] = include_bytes!("../resources/dennis.png");

#[allow(dead_code)]
const TEXTURES: &[&[u8]] = include!(concat!(env!("OUT_DIR"), "/textures.rs"));

fn main() {
    println!("Hello, world!");

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let instance_extensions =
        rwh_instance_extensions(event_loop.display_handle().unwrap().as_raw()).unwrap();

    let library = ThLibrary::load().unwrap();

    let instance = library
        .create_instance(
            vk::API_VERSION_1_2,
            &[c"VK_LAYER_KHRONOS_validation"],
            &instance_extensions,
        )
        .unwrap();

    for physical_device in instance.physical_devices().unwrap().into_iter() {
        let str = unsafe {
            CStr::from_ptr(
                instance
                    .handle
                    .get_physical_device_properties(physical_device.handle)
                    .device_name
                    .as_ptr(),
            )
        };

        println!("{}", str.to_str().unwrap());
    }

    let (physical_device, family) = instance
        .physical_devices()
        .unwrap()
        .sort_by_type()
        .into_iter()
        .find_with_graphics_queue_family()
        .unwrap();

    let device = physical_device
        .create_device(
            &[QueueInfo {
                index: family,
                priorities: &[0.],
            }],
            &[c"VK_KHR_swapchain"],
        )
        .unwrap();

    let queue = device.get_queue(family, 0);

    let fence = device.create_fence().unwrap();

    let command_pool = queue
        .create_command_pool(CommandPoolCreateFlags::empty())
        .unwrap();

    let command_buffer = command_pool
        .allocate_command_buffer(CommandBufferLevel::PRIMARY)
        .unwrap();

    let render_pass = device
        .create_render_pass(
            &[AttachmentDescription {
                format: Format::B8G8R8A8_SRGB,
                samples: SampleCountFlags::TYPE_1,
                load_op: AttachmentLoadOp::CLEAR,
                store_op: AttachmentStoreOp::STORE,
                stencil_load_op: AttachmentLoadOp::DONT_CARE,
                stencil_store_op: AttachmentStoreOp::DONT_CARE,
                initial_layout: ImageLayout::UNDEFINED,
                final_layout: ImageLayout::PRESENT_SRC_KHR,
                ..Default::default()
            }],
            &[SubpassDescription {
                pipeline_bind_point: PipelineBindPoint::GRAPHICS,
                color_attachment_count: 1,
                p_color_attachments: &AttachmentReference {
                    attachment: 0,
                    layout: ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                },
                ..Default::default()
            }],
        )
        .unwrap()
        .arc();

    let descriptor_pool = device
        .create_descriptor_pool(
            10,
            &[DescriptorPoolSize {
                ty: DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 10,
            }],
        )
        .unwrap();

    let vertex_shader = device
        .create_compiled_shader(&thermal::slang::VERTEX)
        .unwrap();

    let solid_shader = device
        .create_compiled_shader(&thermal::slang::SOLID)
        .unwrap();

    let texture_shader = device
        .create_compiled_shader(&thermal::slang::TEXTURE)
        .unwrap();

    let descriptor_set_layouts = thermal::slang::TEXTURE
        .set_layouts
        .iter()
        .map(|set| device.create_descriptor_set_layout(set).unwrap())
        .collect::<Vec<_>>();

    let solid_pipeline_layout = device
        .create_pipeline_layout::<ThDescriptorSetLayout>(vec![], &[])
        .unwrap();

    let texture_pipeline_layout = device
        .create_pipeline_layout(descriptor_set_layouts, &[])
        .unwrap();

    #[allow(unused_variables)]
    let solid_pipeline = solid_pipeline_layout
        .create_graphics_pipeline(
            render_pass.handle(),
            GraphicsPipelineSettings {
                vertex_shader: vertex_shader.handle(),

                fragment_shader: solid_shader.handle(),

                vertex_bindings: thermal::slang::VERTEX.vertex_bindings,

                vertex_attributes: thermal::slang::VERTEX.vertex_attributes,

                samples: SampleCountFlags::TYPE_1,

                sample_shading: Option::None,
            },
        )
        .unwrap();

    let texture_pipeline = texture_pipeline_layout
        .create_graphics_pipeline(
            render_pass.handle(),
            GraphicsPipelineSettings {
                vertex_shader: vertex_shader.handle(),

                fragment_shader: texture_shader.handle(),

                vertex_bindings: thermal::slang::VERTEX.vertex_bindings,

                vertex_attributes: thermal::slang::VERTEX.vertex_attributes,

                samples: SampleCountFlags::TYPE_1,

                sample_shading: Option::None,
            },
        )
        .unwrap();

    let descriptor_sets = descriptor_pool
        .allocate_descriptor_set(
            &texture_pipeline
                .layout()
                .set_layouts()
                .iter()
                .map(|set_layout| set_layout.handle())
                .collect::<Vec<_>>(),
        )
        .unwrap();

    let descriptor_sets2 = descriptor_pool
        .allocate_descriptor_set(
            &texture_pipeline
                .layout()
                .set_layouts()
                .iter()
                .map(|set_layout| set_layout.handle())
                .collect::<Vec<_>>(),
        )
        .unwrap();

    let sampler = device
        .create_sampler(Filter::NEAREST, SamplerAddressMode::CLAMP_TO_BORDER)
        .unwrap();

    let mut decoder = Decoder::new(Cursor::new(IMAGE2));

    decoder.set_transformations(Transformations::ALPHA);

    let mut reader = decoder.read_info().unwrap();

    let mut decoder_buffer = vec![0; reader.output_buffer_size().unwrap()];

    let frame = reader.next_frame(&mut decoder_buffer).unwrap();

    let image_data = &decoder_buffer[..frame.buffer_size()];

    let command = Command::new(queue.clone()).unwrap();

    let image = command
        .create_texture(
            Format::R8G8B8A8_SRGB,
            1,
            image_data,
            frame.width,
            frame.height,
            4,
        )
        .unwrap();

    let image_view = image
        .create_image_view(
            Format::R8G8B8A8_SRGB,
            defaults::MAPPING_RGBA,
            defaults::SUBRESOURCE_COLOR,
        )
        .unwrap();

    let mut atlas = Atlas::new(512, 512, 4);

    for texture in TEXTURES {
        let mut decoder = Decoder::new(Cursor::new(texture));

        decoder.set_transformations(Transformations::ALPHA);

        let mut reader = decoder.read_info().unwrap();

        let mut decoder_buffer = vec![0; reader.output_buffer_size().unwrap()];

        let frame = reader.next_frame(&mut decoder_buffer).unwrap();

        let image_data = &decoder_buffer[..frame.buffer_size()];

        atlas.add(image_data, frame.width, frame.height);
    }

    let atlas_texture = atlas
        .create_texture(command, Format::R8G8B8A8_SRGB, 1)
        .unwrap();

    let atlas_view = atlas_texture
        .create_image_view(
            Format::R8G8B8A8_SRGB,
            defaults::MAPPING_RGBA,
            defaults::SUBRESOURCE_COLOR,
        )
        .unwrap();

    device.update_descriptor_sets(
        &[descriptor_sets.clone(), descriptor_sets2.clone()].concat(),
        &[
            &[Binding::CombinedImageSampler(
                sampler.handle(),
                image_view.handle(),
                ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            )],
            &[Binding::CombinedImageSampler(
                sampler.handle(),
                atlas_view.handle(),
                ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            )],
        ],
    );

    let viewport_matrix = viewport_matrix(0., 0., 1280., 720.);

    let draw_handle = DrawHandle::<Vertex, Affine2>::new(device);

    let mut app = App {
        queue,
        render_pass,
        command_buffer,
        draw_handle,
        fence,
        solid_pipeline,
        texture_pipeline,
        texture: descriptor_sets2,
        viewport_matrix,
        frame: 0,
        window: None,
        presenter: None,
        image_views: vec![],
        framebuffers: vec![],
    };

    event_loop.run_app(&mut app).unwrap();

    println!("Done");
}

struct App {
    queue: ThQueue,

    render_pass: Arc<ThRenderPass>,

    command_buffer: ThCommandBuffer<ThCommandPool>,

    draw_handle: DrawHandle<Vertex, Affine2>,

    fence: ThFence,

    #[allow(dead_code)]
    solid_pipeline: ThPipeline<ThPipelineLayout<ThDescriptorSetLayout>>,

    texture_pipeline: ThPipeline<ThPipelineLayout<ThDescriptorSetLayout>>,

    texture: Vec<DescriptorSet>,

    viewport_matrix: Affine2,

    frame: u64,

    window: Option<Arc<Window>>,

    presenter: Option<Presenter<ThRwhSurface<Arc<Window>, Arc<Window>>>>,

    image_views: Vec<ThImageView<ThSwapchainImage<Arc<ThRwhSurface<Arc<Window>, Arc<Window>>>>>>,

    framebuffers: Vec<ThFramebuffer<Arc<ThRenderPass>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Thermal")
                        .with_name("thermal", ""),
                )
                .unwrap(),
        );

        let surface_factory = self
            .queue
            .device()
            .physical_device
            .instance
            .create_rwh_surface_factory(window.clone())
            .unwrap();

        let surface = surface_factory.create_rwh_surface(window.clone()).unwrap();

        let present_mode = *self
            .queue
            .device()
            .physical_device
            .surface_present_modes(surface.handle())
            .unwrap()
            .iter()
            .min()
            .unwrap();

        let mut presenter = Presenter::new(self.queue.clone(), surface).unwrap();

        presenter.usage = ImageUsageFlags::COLOR_ATTACHMENT;

        presenter.present_mode = present_mode;

        self.window = Some(window);
        self.presenter = Some(presenter);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_) => self.resize(),
            WindowEvent::RedrawRequested => self.draw(),
            WindowEvent::CursorMoved { .. } => self.window().request_redraw(),
            _ => (),
        }
    }
}

impl App {
    fn window(&self) -> &Window {
        self.window.as_ref().unwrap()
    }

    fn presenter(&self) -> &Presenter<ThRwhSurface<Arc<Window>, Arc<Window>>> {
        self.presenter.as_ref().unwrap()
    }

    fn resize(&mut self) {
        let size = self.window().inner_size();

        let presenter = self.presenter.as_mut().unwrap();

        presenter.set_size(size.width, size.height).unwrap();

        self.image_views = presenter
            .images()
            .iter()
            .map(|image| {
                image
                    .clone()
                    .create_image_view(
                        Format::B8G8R8A8_SRGB,
                        defaults::MAPPING_RGBA,
                        defaults::SUBRESOURCE_COLOR,
                    )
                    .unwrap()
            })
            .collect();

        self.framebuffers = self
            .image_views
            .iter()
            .map(|image_view| {
                self.render_pass
                    .clone()
                    .create_framebuffer(
                        &[image_view.handle()],
                        presenter.width(),
                        presenter.height(),
                    )
                    .unwrap()
            })
            .collect();
    }

    fn draw(&mut self) {
        let presenter = self.presenter();

        let Some((index, _)) = presenter.acquire_next_image(u64::MAX).unwrap_out_of_date() else {
            println!("out of date");

            self.resize();

            self.window().request_redraw();

            return;
        };

        self.command_buffer
            .begin(CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .unwrap();

        self.command_buffer.cmd_begin_render_pass(
            self.render_pass.handle(),
            self.framebuffers[index as usize].handle(),
            rect(0, 0, presenter.width(), presenter.height()),
            &[ClearValue {
                color: ClearColorValue {
                    float32: [0., 1., 0., 1.],
                },
            }],
            SubpassContents::INLINE,
        );

        self.command_buffer.cmd_set_viewport(viewport(
            0.,
            0.,
            presenter.width() as f32,
            presenter.height() as f32,
        ));

        self.command_buffer
            .cmd_set_scissor(rect(0, 0, presenter.width(), presenter.height()));

        self.command_buffer
            .cmd_bind_pipeline(self.texture_pipeline.handle());

        self.command_buffer.cmd_bind_descriptor_sets(
            PipelineBindPoint::GRAPHICS,
            self.texture_pipeline.layout().handle(),
            0,
            &self.texture,
        );

        let color_white = Vec4::ONE;

        let mesh_rect = Rect::new(
            100. + self.frame as f32 / 100.,
            100.,
            700.,
            700.,
            color_white,
        );

        self.draw_handle.add(&mesh_rect);

        self.draw_handle.set_instance(&[self.viewport_matrix]);

        self.draw_handle.draw(&self.command_buffer);

        self.command_buffer.cmd_end_render_pass();

        self.command_buffer.end().unwrap();

        self.draw_handle.flush();

        let presenter = self.presenter();

        self.queue
            .submit(
                self.fence.handle(),
                &[presenter.semaphore().handle()],
                &[PipelineStageFlags::BOTTOM_OF_PIPE],
                &[self.command_buffer.handle],
                &[presenter.present_semaphores()[index as usize].handle()],
            )
            .unwrap();

        presenter.present(index).unwrap_out_of_date();

        self.fence.wait(u64::MAX).unwrap();

        self.fence.reset().unwrap();

        self.command_buffer.command_pool.reset().unwrap();

        self.frame += 1;
    }
}
