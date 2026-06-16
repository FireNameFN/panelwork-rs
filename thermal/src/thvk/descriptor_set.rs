use std::sync::Arc;

use ash::vk::{
    DescriptorImageInfo, DescriptorSet, DescriptorType, ImageLayout, ImageView, Sampler,
    WriteDescriptorSet,
};

use crate::thvk::device::ThDevice;

pub enum Binding {
    CombinedImageSampler(Sampler, ImageView, ImageLayout),
}

impl ThDevice {
    pub fn update_descriptor_sets(
        self: &Arc<ThDevice>,
        descriptor_sets: &[DescriptorSet],
        bindings: &[&[Binding]],
    ) {
        let descriptor_count = bindings.iter().map(|bindings| bindings.len()).sum();

        let mut descriptor_images: Vec<DescriptorImageInfo> = Vec::with_capacity(descriptor_count);

        let mut descriptor_writes = Vec::with_capacity(descriptor_count);

        for (descriptor_set, bindings) in descriptor_sets.iter().zip(bindings) {
            for (i, binding) in bindings.iter().enumerate() {
                let descriptor_type = match *binding {
                    Binding::CombinedImageSampler(sampler, image_view, image_layout) => {
                        descriptor_images.push(DescriptorImageInfo {
                            sampler,
                            image_view,
                            image_layout,
                        });

                        DescriptorType::COMBINED_IMAGE_SAMPLER
                    }
                };

                descriptor_writes.push(WriteDescriptorSet {
                    dst_set: *descriptor_set,
                    dst_binding: i as u32,
                    descriptor_count: 1,
                    descriptor_type: descriptor_type,
                    p_image_info: descriptor_images.last().unwrap(),
                    ..Default::default()
                });
            }
        }

        unsafe { self.handle.update_descriptor_sets(&descriptor_writes, &[]) }
    }
}
