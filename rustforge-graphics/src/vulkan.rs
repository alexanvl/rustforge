//! Vulkan initialization and device management

use std::sync::Arc;
use vulkano::{
    VulkanLibrary,
    instance::{Instance, InstanceCreateInfo},
    device::{
        Device, Queue,
        physical::PhysicalDevice,
    },
    swapchain::Surface,
    Version,
};
use winit::window::Window;
use rustforge_core::prelude::*;

/// Vulkan context holding device and queues
pub struct VulkanContext {
    pub instance: Arc<Instance>,
    pub surface: Arc<Surface>,
    pub physical_device: Arc<PhysicalDevice>,
    pub device: Arc<Device>,
    pub graphics_queue: Arc<Queue>,
    pub present_queue: Arc<Queue>,
}

impl VulkanContext {
    /// Initialize Vulkan with window surface
    pub fn new(_window: Arc<Window>) -> Result<Self> {
        // Create Vulkan library
        let library = VulkanLibrary::new()
            .map_err(|e| Error::Init(format!("Failed to load Vulkan library: {}", e)))?;

        // Create Vulkan instance
        let _instance = Instance::new(
            library,
            InstanceCreateInfo {
                application_name: Some("RustForge".into()),
                application_version: Version::V1_0,
                engine_name: Some("RustForge Engine".into()),
                engine_version: Version::V1_0,
                ..Default::default()
            },
        )
        .map_err(|e| Error::Init(format!("Failed to create Vulkan instance: {}", e)))?;

        // Create surface (TODO: Fix raw window handle version compatibility)
        // For now, create a dummy surface - this needs proper implementation
        // let surface = Surface::from_window(instance.clone(), window)
        //     .map_err(|e| Error::Init(format!("Failed to create surface: {}", e)))?;

        // TEMPORARY: Skip surface creation for now
        Err(Error::Init("Surface creation temporarily disabled due to raw-window-handle version conflict".into()))

        /* TODO: Re-enable once surface creation works
        // Select physical device
        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .map_err(|e| Error::Init(format!("Failed to enumerate devices: {}", e)))?
            .filter(|p| p.supported_extensions().contains(&DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::empty()
            }))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS) && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .ok_or_else(|| Error::Init("No suitable physical device found".into()))?;

        // Create logical device
        let (device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: DeviceExtensions {
                    khr_swapchain: true,
                    ..DeviceExtensions::empty()
                },
                ..Default::default()
            },
        )
        .map_err(|e| Error::Init(format!("Failed to create device: {}", e)))?;

        let queue = queues.next().unwrap();

        Ok(Self {
            instance,
            surface,
            physical_device,
            device,
            graphics_queue: queue.clone(),
            present_queue: queue,
        })
        */
    }
}
