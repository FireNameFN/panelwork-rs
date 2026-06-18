use std::{ffi::CStr, io::Cursor};

use ash::vk::{
    self, AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
    ClearColorValue, ClearValue, CommandBufferLevel, CommandBufferUsageFlags,
    CommandPoolCreateFlags, DescriptorPoolSize, DescriptorType, Filter, Format, ImageLayout,
    ImageUsageFlags, PipelineBindPoint, PipelineStageFlags, SampleCountFlags, SamplerAddressMode,
    SubpassContents, SubpassDescription,
};
use png::{Decoder, Transformations};
use sdl3::event::{Event, WindowEvent};
use thermal::{
    core::{command::Command, presenter::Presenter, vertex_buffer::VertexBuffer},
    defaults,
    ext::{
        physical_device::ThPhysicalDeviceIteratorExt, result::SwapchainResultExt,
        sdl3_physical_device::ThPhysicalDeviceSdl3IteratorExt,
    },
    primitives, sdl3_util,
    thvk::{
        descriptor_set::Binding, device::QueueInfo, handle::ThHandle,
        image_view::ThImageViewSource, library::ThLibrary, pipeline::GraphicsPipelineSettings,
    },
};

#[allow(dead_code)]
const IMAGE: &[u8] = include_bytes!("../resources/OverGreen.png");

#[allow(dead_code)]
const IMAGE2: &[u8] = include_bytes!("../resources/dennis.png");

fn main() {
    println!("Hello, world!");

    sdl3::hint::set(sdl3::hint::names::VIDEO_DRIVER, "wayland,x11,cocoa,windows");

    let sdl = sdl3::init().unwrap();

    let video = sdl.video().unwrap();

    video.vulkan_load_library_default().unwrap();

    let instance_extensions = sdl3_util::sdl_instance_extensions();

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
        .find_with_sdl_presentation_support()
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
        .unwrap();

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

    let solid_pipeline_layout = device.create_pipeline_layout(vec![], &[]).unwrap();

    let texture_pipeline_layout = device
        .create_pipeline_layout(descriptor_set_layouts.clone(), &[])
        .unwrap();

    let solid_pipeline = solid_pipeline_layout
        .create_graphics_pipeline(
            render_pass.handle,
            GraphicsPipelineSettings {
                vertex_shader: vertex_shader.clone(),

                fragment_shader: solid_shader,

                vertex_bindings: thermal::slang::VERTEX.vertex_bindings,

                vertex_attributes: thermal::slang::VERTEX.vertex_attributes,

                samples: SampleCountFlags::TYPE_1,

                sample_shading: Option::None,
            },
        )
        .unwrap();

    let texture_pipeline = texture_pipeline_layout
        .create_graphics_pipeline(
            render_pass.handle,
            GraphicsPipelineSettings {
                vertex_shader: vertex_shader,

                fragment_shader: texture_shader,

                vertex_bindings: thermal::slang::VERTEX.vertex_bindings,

                vertex_attributes: thermal::slang::VERTEX.vertex_attributes,

                samples: SampleCountFlags::TYPE_1,

                sample_shading: Option::None,
            },
        )
        .unwrap();

    let descriptor_sets = descriptor_pool
        .allocate_descriptor_set(
            &descriptor_set_layouts
                .iter()
                .map(|set_layout| set_layout.handle)
                .collect::<Vec<_>>(),
        )
        .unwrap();

    let sampler = device
        .create_sampler(Filter::NEAREST, SamplerAddressMode::CLAMP_TO_BORDER)
        .unwrap();

    let reader = Cursor::new(IMAGE2);

    let mut decoder = Decoder::new(reader);

    decoder.set_transformations(Transformations::ALPHA);

    let mut reader = decoder.read_info().unwrap();

    let mut buf = vec![0; reader.output_buffer_size().unwrap()];

    let frame = reader.next_frame(&mut buf).unwrap();

    let bytes = &buf[..frame.buffer_size()];

    let command = Command::new(queue.clone()).unwrap();

    let image = command
        .create_texture(&physical_device, Format::R8G8B8A8_SRGB, 1, bytes, 48, 48, 4)
        .unwrap();

    let image_view = image
        .create_image_view(
            Format::R8G8B8A8_SRGB,
            defaults::MAPPING_RGBA,
            defaults::SUBRESOURCE_COLOR,
        )
        .unwrap();

    device.update_descriptor_sets(
        &descriptor_sets,
        &[&[Binding::CombinedImageSampler(
            sampler.handle,
            image_view.handle,
            ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )]],
    );

    let mut vertex_buffer = VertexBuffer::<(f32, f32)>::new(physical_device.clone(), device, 32);

    let (buffer, _) = vertex_buffer.add(&[
        (-0.5, -0.5),
        (0.5, -0.5),
        (-0.5, 0.5),
        (0.5, -0.5),
        (-0.5, 0.5),
        (0.5, 0.5),
    ]);

    let window = video
        .window("Thermal", 1280, 720)
        .resizable()
        .vulkan()
        .build()
        .unwrap();

    let surface = instance.create_sdl3_surface(window.raw()).unwrap();

    let mut presenter =
        Presenter::new(physical_device.clone(), queue.clone(), surface.clone()).unwrap();

    presenter.usage = ImageUsageFlags::COLOR_ATTACHMENT;

    presenter.present_mode = physical_device
        .surface_present_modes(surface.handle())
        .unwrap()
        .into_iter()
        .min()
        .unwrap();

    presenter.set_size(1280, 720).unwrap();

    let mut image_views = presenter
        .images()
        .iter()
        .map(|image| {
            image
                .create_image_view(
                    Format::B8G8R8A8_SRGB,
                    defaults::MAPPING_RGBA,
                    defaults::SUBRESOURCE_COLOR,
                )
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut framebuffers = image_views
        .iter()
        .map(|image_view| {
            render_pass
                .create_framebuffer(&[image_view.handle], 1280, 720)
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut event_pump = sdl.event_pump().unwrap();

    let mut resize = false;

    'outer: loop {
        let mut event = event_pump.wait_event();

        loop {
            match event {
                Event::Quit { .. } => {
                    break 'outer;
                }
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::PixelSizeChanged(_, _) => resize = true,
                    _ => (),
                },
                _ => (),
            }

            event = match event_pump.poll_event() {
                None => break,
                Some(event) => event,
            };
        }

        if resize {
            resize = false;

            let (width, height) = window.size_in_pixels();

            presenter.set_size(width, height).unwrap();

            image_views = presenter
                .images()
                .iter()
                .map(|image| {
                    image
                        .create_image_view(
                            Format::B8G8R8A8_SRGB,
                            defaults::MAPPING_RGBA,
                            defaults::SUBRESOURCE_COLOR,
                        )
                        .unwrap()
                })
                .collect::<Vec<_>>();

            framebuffers = image_views
                .iter()
                .map(|image_view| {
                    render_pass
                        .create_framebuffer(
                            &[image_view.handle],
                            presenter.width(),
                            presenter.height(),
                        )
                        .unwrap()
                })
                .collect::<Vec<_>>();
        }

        let (index, _) = match presenter.acquire_next_image(u64::MAX).unwrap_out_of_date() {
            None => {
                println!("out of date");

                resize = true;

                continue;
            }
            Some(ok) => ok,
        };

        command_buffer
            .begin(CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .unwrap();

        command_buffer.cmd_begin_render_pass(
            render_pass.handle,
            framebuffers[index as usize].handle,
            primitives::rect(0, 0, presenter.width(), presenter.height()),
            &[ClearValue {
                color: ClearColorValue {
                    float32: [0., 1., 0., 1.],
                },
            }],
            SubpassContents::INLINE,
        );

        command_buffer.cmd_set_viewport(primitives::viewport(
            0.,
            0.,
            presenter.width() as f32,
            presenter.height() as f32,
        ));

        command_buffer.cmd_set_scissor(primitives::rect(
            0,
            0,
            presenter.width(),
            presenter.height(),
        ));

        command_buffer.cmd_bind_vertex_buffers(0, &[buffer], &[0]);

        command_buffer.cmd_bind_pipeline(texture_pipeline.handle);

        command_buffer.cmd_bind_descriptor_sets(
            PipelineBindPoint::GRAPHICS,
            texture_pipeline_layout.handle,
            0,
            &descriptor_sets,
        );

        command_buffer.cmd_draw(6, 1, 0, 0);

        command_buffer.cmd_end_render_pass();

        command_buffer.end().unwrap();

        vertex_buffer.flush();

        queue
            .submit(
                fence.handle,
                &[presenter.semaphore().handle],
                &[PipelineStageFlags::BOTTOM_OF_PIPE],
                &[command_buffer.handle],
                &[presenter.present_semaphores()[index as usize].handle],
            )
            .unwrap();

        presenter.present(index).unwrap_out_of_date();

        fence.wait(u64::MAX).unwrap();

        fence.reset().unwrap();

        command_pool.reset().unwrap();
    }

    println!("Done");
}
