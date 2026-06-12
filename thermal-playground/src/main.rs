use std::ffi::CStr;

use ash::vk::{
    self, AccessFlags, CommandBufferBeginInfo, CommandBufferLevel, CommandBufferUsageFlags,
    CommandPoolCreateFlags, Handle, ImageLayout, ImageUsageFlags, PipelineStageFlags, SurfaceKHR,
};
use sdl3_sys::{
    events::{SDL_Event, SDL_EventType},
    video::{SDL_WINDOW_RESIZABLE, SDL_WINDOW_VULKAN},
};
use thermal::{
    core::presenter::Presenter,
    ext::physical_device::ThPhysicalDeviceIteratorExt,
    thvk::{device::QueueInfo, library::ThLibrary},
    util,
};

fn main() {
    println!("Hello, world!");

    let instance_extensions;

    unsafe {
        sdl3_sys::hints::SDL_SetHint(
            sdl3_sys::hints::SDL_HINT_VIDEO_DRIVER,
            c"wayland,x11,cocoa,windows".as_ptr(),
        );

        sdl3_sys::init::SDL_Init(sdl3_sys::init::SDL_InitFlags::VIDEO);

        sdl3_sys::vulkan::SDL_Vulkan_LoadLibrary(std::ptr::null());

        instance_extensions = util::string_array_from_fn(|size| {
            sdl3_sys::vulkan::SDL_Vulkan_GetInstanceExtensions(size)
        });
    }

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
        .find_with_queue_family(|device, family, _| unsafe {
            sdl3_sys::vulkan::SDL_Vulkan_GetPresentationSupport(
                device.instance.handle.handle().as_raw() as *mut _,
                device.handle.as_raw() as *mut _,
                family,
            )
        })
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

    let mut surface = std::ptr::null_mut();

    unsafe {
        let window = sdl3_sys::video::SDL_CreateWindow(
            c"Thermal".as_ptr(),
            1280,
            720,
            SDL_WINDOW_RESIZABLE | SDL_WINDOW_VULKAN,
        );

        sdl3_sys::vulkan::SDL_Vulkan_CreateSurface(
            window,
            instance.handle.handle().as_raw() as *mut _,
            std::ptr::null(),
            &mut surface,
        );

        let mut presenter = Presenter::new(
            &physical_device,
            &queue,
            SurfaceKHR::from_raw(surface as u64),
        )
        .unwrap();

        presenter.usage = ImageUsageFlags::COLOR_ATTACHMENT;

        presenter.present_mode = physical_device
            .surface_present_modes(SurfaceKHR::from_raw(surface as u64))
            .unwrap()
            .into_iter()
            .min()
            .unwrap();

        presenter.set_size(1280, 720).unwrap();

        'outer: loop {
            sdl3_sys::events::SDL_WaitEvent(std::ptr::null_mut());

            let mut event = SDL_Event::default();

            while sdl3_sys::events::SDL_PollEvent(&mut event) {
                if event.event_type() == SDL_EventType::QUIT {
                    break 'outer;
                }
            }

            let (index, _) = match presenter.acquire_next_image(u64::MAX) {
                Err(result) => {
                    println!("{}", result);

                    presenter.set_size(1280, 720).unwrap();

                    continue;
                }
                Ok(ok) => ok,
            };

            device
                .handle
                .begin_command_buffer(
                    command_buffer.handle,
                    &CommandBufferBeginInfo {
                        flags: CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                        ..Default::default()
                    },
                )
                .unwrap();

            command_buffer.image_barrier(
                presenter.images[index as usize].handle,
                AccessFlags::NONE,
                AccessFlags::NONE,
                ImageLayout::UNDEFINED,
                ImageLayout::PRESENT_SRC_KHR,
                PipelineStageFlags::TOP_OF_PIPE,
                PipelineStageFlags::BOTTOM_OF_PIPE,
            );

            device
                .handle
                .end_command_buffer(command_buffer.handle)
                .unwrap();

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
    }

    unsafe {
        sdl3_sys::vulkan::SDL_Vulkan_DestroySurface(
            instance.handle.handle().as_raw() as *mut _,
            surface,
            std::ptr::null(),
        )
    }

    println!("Done");
}
