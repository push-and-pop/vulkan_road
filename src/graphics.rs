use std::sync::Arc;

use vulkano::device::QueueFlags;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::swapchain::Surface;
use vulkano::VulkanLibrary;
use winit::event_loop::EventLoop;

use crate::window::Window;

pub struct Graphics {
    vulkan_instance: Arc<Instance>,
    window: Window,
}
impl Graphics {
    pub fn new() -> Self {
        let window = Window::new();
        let library: Arc<VulkanLibrary> =
            VulkanLibrary::new().expect("no local Vulkan library/DLL");
        let required_extensions = Surface::required_extensions(&window.event_loop);
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions, // 实例扩展
                ..Default::default()
            },
        )
        .expect("failed to create instance");
        Graphics {
            vulkan_instance: instance,
            window: window,
        }
    }

    pub fn set_device(self) -> Self {
        let physical_device = self
            .vulkan_instance
            .enumerate_physical_devices()
            .expect("could not enumerate devices")
            .next()
            .expect("no devices available");

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            })
            .expect("couldn't find a graphical queue family")
            as u32;

        self
    }
}
