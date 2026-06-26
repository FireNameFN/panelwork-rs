use ash::{VkResult, vk::Format};

use crate::{
    core::command::Command,
    thvk::{device_memory::ThDeviceMemory, image::ThImage},
};

pub struct Atlas {
    pixel_size: u32,

    width: u32,

    height: u32,

    buffer: Vec<u8>,

    skyline: Vec<(u32, u32)>,
}

impl Atlas {
    pub fn new(width: u32, height: u32, pixel_size: u32) -> Self {
        Self {
            pixel_size,
            width,
            height,
            buffer: vec![0; (width * height * pixel_size) as usize],
            skyline: vec![(0, 0)],
        }
    }

    pub fn pixel_size(&self) -> u32 {
        self.pixel_size
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn slice(&self) -> &[u8] {
        &self.buffer
    }

    pub fn create_texture(
        &self,
        command: Command,
        format: Format,
        mip_levels: u32,
    ) -> VkResult<ThImage<ThDeviceMemory>> {
        command.create_texture(
            format,
            mip_levels,
            &self.buffer,
            self.width,
            self.height,
            self.pixel_size,
        )
    }

    pub fn add(&mut self, slice: &[u8], width: u32, height: u32) -> (u32, u32) {
        self.add_stride(slice, width, height, width * 4)
    }

    pub fn add_stride(&mut self, slice: &[u8], width: u32, height: u32, stride: u32) -> (u32, u32) {
        let (y, index, last_index) = loop {
            match self.find_point(width, height) {
                None => self.grow(width, height),
                Some(ok) => break ok,
            };
        };

        let x = self.skyline[index].0;

        self.copy(slice, stride, height, x, y);

        let max_width = x + width;

        let max_height = y + height;

        let last_height = self.skyline[last_index].1;

        self.skyline.splice(
            index..=last_index,
            [(x, max_height), (max_width, last_height)],
        );

        (x, y)
    }

    pub fn find_point(&self, width: u32, height: u32) -> Option<(u32, usize, usize)> {
        if width > self.width || height > self.height {
            return None;
        }

        let max_width = self.width - width;

        let max_height = self.height - height;

        let mut y = u32::MAX;

        let mut result = None;

        for i in 0..self.skyline.len() {
            let (px, mut py) = self.skyline[i];

            if px > max_width {
                break;
            }

            if py >= y {
                continue;
            }

            let x2 = px + width;

            let mut last_index = i;

            for j in i + 1..self.skyline.len() {
                let (npx, npy) = self.skyline[j];

                if npx >= x2 {
                    break;
                }

                py = py.max(npy);

                last_index = j;
            }

            if py >= y {
                continue;
            }

            if py > max_height {
                continue;
            }

            y = py;

            result = Some((py, i, last_index));
        }

        result
    }

    fn grow(&mut self, width: u32, height: u32) {
        let mut next_width = self.width;
        let mut next_height = self.height;

        if next_width <= next_height {
            next_width *= 2;
        } else {
            next_height *= 2
        }

        if width > next_width {
            next_width = width.next_power_of_two();
        }

        if height > next_height {
            next_height = height.next_power_of_two();
        }

        self.resize(next_width, next_height);
    }

    fn resize(&mut self, width: u32, height: u32) {
        let stride = self.width * self.pixel_size;

        let old_buffer = std::mem::replace(
            &mut self.buffer,
            vec![0; (width * height * self.pixel_size) as usize],
        );

        self.width = width;

        self.copy(&old_buffer, stride, self.height, 0, 0);

        self.height = height;
    }

    fn copy(&mut self, slice: &[u8], stride: u32, height: u32, x: u32, y: u32) {
        let stride = stride as usize;

        let buffer_stride = (self.width * self.pixel_size) as usize;

        let mut offset = 0;

        let mut buffer_offset = buffer_stride * y as usize + (x * self.pixel_size) as usize;

        for _ in 0..height {
            self.buffer[buffer_offset..buffer_offset + stride]
                .copy_from_slice(&slice[offset..offset + stride]);

            offset += stride;
            buffer_offset += buffer_stride;
        }
    }
}
