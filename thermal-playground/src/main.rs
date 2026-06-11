use std::{ffi::CStr, slice};

use ash::vk::{self, Handle};
use sdl3_sys::{
    events::{SDL_Event, SDL_EventType},
    video::{SDL_WINDOW_RESIZABLE, SDL_WINDOW_VULKAN},
};
use thermal::{
    ext::physical_device::ThPhysicalDeviceIteratorExt,
    thvk::{device::QueueInfo, library::ThLibrary},
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

        let mut count = 0;

        let f = slice::from_raw_parts(
            sdl3_sys::vulkan::SDL_Vulkan_GetInstanceExtensions(&mut count),
            count as usize,
        );

        instance_extensions = f.iter().map(|ptr| CStr::from_ptr(*ptr)).collect::<Vec<_>>();
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
                priorities: &[0.0],
            }],
            &[],
        )
        .unwrap();

    //std::thread::sleep(Duration::from_secs(60));

    unsafe {
        let mut window = std::ptr::null_mut();

        let mut renderer = std::ptr::null_mut();

        sdl3_sys::render::SDL_CreateWindowAndRenderer(
            c"Thermal".as_ptr(),
            1280,
            720,
            SDL_WINDOW_RESIZABLE | SDL_WINDOW_VULKAN,
            &mut window,
            &mut renderer,
        );

        'outer: loop {
            sdl3_sys::events::SDL_WaitEvent(std::ptr::null_mut());

            let mut event = SDL_Event::default();

            while sdl3_sys::events::SDL_PollEvent(&mut event) {
                if event.event_type() == SDL_EventType::QUIT {
                    break 'outer;
                }
            }

            sdl3_sys::render::SDL_RenderClear(renderer);

            sdl3_sys::render::SDL_RenderPresent(renderer);
        }
    }

    println!("Done");
}
