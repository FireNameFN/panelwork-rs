use std::ffi::CStr;

use ash::vk::{
    self, AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
    ClearColorValue, ClearValue, CommandBufferLevel, CommandBufferUsageFlags,
    CommandPoolCreateFlags, Extent2D, Format, ImageLayout, ImageUsageFlags, Offset2D,
    PipelineBindPoint, PipelineStageFlags, Rect2D, SampleCountFlags, SubpassContents,
    SubpassDescription, Viewport,
};
use sdl3::event::{Event, WindowEvent};
use thermal::{
    core::{presenter::Presenter, vertex_buffer::VertexBuffer},
    defaults,
    ext::{
        physical_device::ThPhysicalDeviceIteratorExt, result::PresentResultExt,
        sdl3_physical_device::ThPhysicalDeviceSdl3IteratorExt,
    },
    sdl3_util,
    thvk::{device::QueueInfo, library::ThLibrary, pipeline::GraphicsPipelineSettings},
};

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
        .filter_discrete()
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

    let command_pool = device
        .create_command_pool(family, CommandPoolCreateFlags::empty())
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

    let vertex_shader = device
        .create_shader_module(thermal::shaders::VERTEX.code())
        .unwrap();

    let solid_shader = device
        .create_shader_module(thermal::shaders::SOLID.code())
        .unwrap();

    let solid_pipeline_layout = device.create_pipeline_layout(&[], &[]).unwrap();

    let solid_pipeline = solid_pipeline_layout
        .create_graphics_pipeline(
            render_pass.handle,
            GraphicsPipelineSettings {
                vertex_shader: vertex_shader,

                fragment_shader: solid_shader,

                vertex_bindings: thermal::shaders::VERTEX.bindings,

                vertex_attributes: thermal::shaders::VERTEX.attributes,

                samples: SampleCountFlags::TYPE_1,

                sample_shading: Option::None,
            },
        )
        .unwrap();

    let mut vertex_buffer = VertexBuffer::<(f32, f32)>::new(physical_device.clone(), device, 32);

    vertex_buffer.add(&[
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

    let mut presenter = Presenter::new(&physical_device, &queue, surface.clone()).unwrap();

    presenter.usage = ImageUsageFlags::COLOR_ATTACHMENT;

    presenter.present_mode = physical_device
        .surface_present_modes(surface.handle)
        .unwrap()
        .into_iter()
        .min()
        .unwrap();

    presenter.set_size(1280, 720).unwrap();

    let mut image_views = presenter
        .images
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
                .images
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
                        .create_framebuffer(&[image_view.handle], presenter.width, presenter.height)
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
            Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D {
                    width: presenter.width,
                    height: presenter.height,
                },
            },
            &[ClearValue {
                color: ClearColorValue {
                    float32: [0., 1., 0., 1.],
                },
            }],
            SubpassContents::INLINE,
        );

        command_buffer.cmd_set_viewport(Viewport {
            x: 0.,
            y: 0.,
            width: presenter.width as f32,
            height: presenter.height as f32,
            ..Default::default()
        });

        command_buffer.cmd_set_scissor(Rect2D {
            offset: Offset2D::default(),
            extent: Extent2D {
                width: presenter.width,
                height: presenter.height,
            },
        });

        command_buffer.cmd_bind_vertex_buffers(
            0,
            &[vertex_buffer.last_buffer.buffer().handle],
            &[0],
        );

        command_buffer.cmd_bind_pipeline(solid_pipeline.handle);

        command_buffer.cmd_draw(6, 1, 0, 0);

        command_buffer.cmd_end_render_pass();

        command_buffer.end().unwrap();

        vertex_buffer.flush();

        queue
            .submit(
                fence.handle,
                &[presenter.semaphore.handle],
                &[PipelineStageFlags::BOTTOM_OF_PIPE],
                &[command_buffer.handle],
                &[presenter.present_semaphores[index as usize].handle],
            )
            .unwrap();

        presenter.present(index).unwrap_out_of_date();

        fence.wait(u64::MAX).unwrap();

        fence.reset().unwrap();

        command_pool.reset().unwrap();
    }

    println!("Done");
}
