use wgpu::{Device, Features, Instance, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct Setup<'a> {
   pub device: Device,
   pub surface: Surface<'a>,
   pub queue: Queue,
   pub config: SurfaceConfiguration,
   pub size: PhysicalSize<u32>,
   pub window: &'a Window,
}

impl<'a> Setup<'a> {
   pub async fn new(window: &'a Window) -> Self {
      let size = window.inner_size();

      let instance = Instance::new(wgpu::InstanceDescriptor {
         #[cfg(not(target_arch="wasm32"))]
         backends: wgpu::Backends::PRIMARY,

         #[cfg(target_arch="wasm32")]
         backends: wgpu::Backends::GL,
         ..Default::default()
      });

      let surface = instance.create_surface(window).unwrap();

      let adapter = instance
          .request_adapter(&wgpu::RequestAdapterOptions {
             power_preference: wgpu::PowerPreference::HighPerformance,
             compatible_surface: Some(&surface),
             force_fallback_adapter: false,
          })
          .await
          .unwrap();

      let (device, queue) = adapter
          .request_device(
             &wgpu::DeviceDescriptor {
                label: None,
                required_features: Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                required_limits: if cfg!(target_arch = "wasm32") {
                   wgpu::Limits {
                      max_texture_dimension_2d: 8192,
                      ..wgpu::Limits::downlevel_webgl2_defaults()
                   }
                   // ..wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                   wgpu::Limits::default()
                },
             },
             None,
          )
          .await
          .unwrap();


      let surface_caps = surface.get_capabilities(&adapter);

      let surface_format = surface_caps
          .formats
          .iter()
          .copied()
          .find(|f| f.is_srgb())
          .unwrap_or(surface_caps.formats[0]);

      // Query the maximum supported texture size
      // let max_texture_size = adapter.limits().max_texture_dimension_2d;


      let config = SurfaceConfiguration {
         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
         format: surface_format,
         width: size.width,
         height: size.height,
         present_mode: surface_caps.present_modes[0],
         alpha_mode: surface_caps.alpha_modes[0],
         desired_maximum_frame_latency: 2,
         view_formats: vec![],
      };

      Self {
         surface,
         queue,
         config,
         size,
         window,
         device,
      }
   }
}