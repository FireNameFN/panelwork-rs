use std::ffi::CStr;

use ash::vk::{
    self, AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
    ClearColorValue, ClearValue, CommandBufferLevel, CommandBufferUsageFlags,
    CommandPoolCreateFlags, Format, ImageLayout, ImageUsageFlags, PipelineBindPoint,
    PipelineStageFlags, Rect2D, SampleCountFlags, SubpassContents, SubpassDescription,
};
use sdl3::event::Event;
use thermal::{
    core::presenter::Presenter,
    defaults,
    ext::{
        physical_device::ThPhysicalDeviceIteratorExt,
        sdl3_physical_device::ThPhysicalDeviceSdl3IteratorExt,
    },
    sdl3_util,
    thvk::{device::QueueInfo, library::ThLibrary},
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

    'outer: loop {
        let mut event = event_pump.wait_event();

        loop {
            match event {
                Event::Quit { .. } => {
                    break 'outer;
                }
                _ => (),
            }

            event = match event_pump.poll_event() {
                None => break,
                Some(event) => event,
            };
        }

        let (index, _) = match presenter.acquire_next_image(u64::MAX) {
            Err(result) => {
                println!("{}", result);

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
                            .create_framebuffer(
                                &[image_view.handle],
                                presenter.width,
                                presenter.height,
                            )
                            .unwrap()
                    })
                    .collect::<Vec<_>>();

                continue;
            }
            Ok(ok) => ok,
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
                    width: 1280,
                    height: 720,
                },
            },
            &[ClearValue {
                color: ClearColorValue {
                    float32: [0., 1., 0., 1.],
                },
            }],
            SubpassContents::INLINE,
        );

        command_buffer.cmd_end_render_pass();

        command_buffer.end().unwrap();

        queue
            .submit(
                fence.handle,
                &[presenter.semaphore.handle],
                &[PipelineStageFlags::BOTTOM_OF_PIPE],
                &[command_buffer.handle],
                &[presenter.present_semaphores[index as usize].handle],
            )
            .unwrap();

        _ = presenter.present(index);

        fence.wait(u64::MAX).unwrap();

        fence.reset().unwrap();

        command_pool.reset().unwrap();
    }

    println!("Done");
}
