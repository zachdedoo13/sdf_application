use egui::TextureId;
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, Buffer, BufferUsages, Extent3d, Queue, ShaderStages, StorageTextureAccess, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::inbuilt::gui_state::EguiRenderer;
use crate::inbuilt::setup::Setup;

/// to Ping Or Pong
enum POP {
   First,
   Second,
}
pub struct PingPongData<T> {
   first: T,
   second: T,
   current: POP,
}
impl<T> PingPongData<T> {
   pub fn new(first: T, second: T) -> Self {
      Self {
         first,
         second,
         current: POP::First,
      }
   }

   pub fn pull_current(&self) -> &T {
      // send first
      match self.current {
         POP::First => { &self.first }
         POP::Second => { &self.second }
      }
   }

   pub fn pull_other(&self) -> &T {
      // send not first
      match self.current {
         POP::First => { &self.second }
         POP::Second => { & self.first }
      }
   }

   pub fn ping_pong(&mut self) {
      // swap
      self.current = match self.current {
         POP::First => { POP::Second }
         POP::Second => { POP::First }
      }
   }
}



pub struct UniformPackageSingles<T> {
   pub bind_group: BindGroup,
   pub layout: BindGroupLayout,
   pub buffer: Buffer,
   pub data: T,
}
impl<T: bytemuck::Pod> UniformPackageSingles<T> {
   // pre setups
   pub fn create(setup: &Setup, shader_stages: ShaderStages, data: T) -> UniformPackageSingles<T> {

      let buffer = setup.device.create_buffer_init(&BufferInitDescriptor {
         label: Some("UniformPackageSingles"),
         contents: &*Vec::from(bytemuck::bytes_of(&data)),
         usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
      });

      let layout = setup.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
         label: Some("UniformPackageSingles"),
         entries: &[
            wgpu::BindGroupLayoutEntry {
               binding: 0,
               visibility: shader_stages,
               ty: wgpu::BindingType::Buffer {
                  ty: wgpu::BufferBindingType::Uniform,
                  has_dynamic_offset: false,
                  min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<T>() as u64),
               },
               count: None,
            },
         ],
      });

      let bind_group = setup.device.create_bind_group(&BindGroupDescriptor {
         label: None,
         layout: &layout,
         entries: &[BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding()
         }],
      });

      UniformPackageSingles {
         bind_group,
         layout,
         buffer,
         data,
      }
   }

   // functions
   pub fn update_with_data(&self, queue: &Queue) {
      queue.write_buffer(
         &self.buffer,
         0,
         bytemuck::bytes_of(&self.data)
      );
   }

}



pub struct StorageTexturePackage {
   pub size: Extent3d,
   pub texture: Texture,
   pub view: TextureView,
   pub bind_group_layout: BindGroupLayout,
   pub bind_group: BindGroup,
}
impl StorageTexturePackage {
   pub fn new(setup: &Setup, size: (f32, f32)) -> Self {
      let size = Extent3d {
         width: size.0 as u32,
         height: size.1 as u32,
         // width: 128,
         // height: 128,
         depth_or_array_layers: 1,
      };

      let texture_desc = TextureDescriptor {
         label: Some("test"),
         size,
         mip_level_count: 1,
         sample_count: 1,
         dimension: TextureDimension::D2,
         format: TextureFormat::Rgba32Float,
         usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
         view_formats: &[],
      };

      let texture = setup.device.create_texture(&texture_desc);
      let view = texture.create_view(&TextureViewDescriptor::default());

      let bind_group_layout =
          setup.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
             entries: &[
                wgpu::BindGroupLayoutEntry {
                   binding: 0,
                   visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                   ty: wgpu::BindingType::StorageTexture {
                      access: StorageTextureAccess::ReadWrite,
                      format: TextureFormat::Rgba32Float,
                      view_dimension: TextureViewDimension::D2,
                   },
                   count: None,
                },
             ],
             label: Some("texture_bind_group_layout"),
          });

      let bind_group = setup.device.create_bind_group(&BindGroupDescriptor {
         layout: &bind_group_layout,
         entries: &[
            BindGroupEntry {
               binding: 0,
               resource: wgpu::BindingResource::TextureView(&view),
            },
         ],
         label: Some("diffuse_bind_group"),
      });


      Self {
         size,
         texture,
         view,
         bind_group_layout,
         bind_group,
      }
   }

   pub fn remake(&mut self, setup: &Setup, size: (f32, f32)) {
      *self = Self::new(setup, size);
   }
}


pub struct EguiTexturePackage {
   pub texture: Texture,
   pub view: TextureView,
   pub texture_id: TextureId,
   pub size: Extent3d,
}
impl EguiTexturePackage {
   pub fn new(setup: &Setup, egui_renderer: &mut EguiRenderer, in_size: Extent3d) -> Self {
      let size = Extent3d {
         width: if in_size.width > 0 { in_size.width } else { 1 },
         height: if in_size.height > 0 { in_size.height } else { 1 },
         depth_or_array_layers: 1,
      };

      let texture = setup.device.create_texture(&TextureDescriptor {
         label: Some("Egui Texture"),
         size: size.clone(),
         mip_level_count: 1,
         sample_count: 1,
         dimension: TextureDimension::D2,
         format: TextureFormat::Rgba8UnormSrgb,
         usage: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
         view_formats: &[],
      });

      let view = texture.create_view(&TextureViewDescriptor::default());

      let texture_id = egui_renderer.renderer.register_native_texture(
         &setup.device,
         &view,
         wgpu::FilterMode::Linear
      );

      Self {
         texture,
         view,
         texture_id,
         size,
      }
   }

   pub fn update(&mut self, setup: &Setup, egui_renderer: &mut EguiRenderer) {
      if self.texture.size() != self.size {
         let size = self.size;
         *self = Self::new(&setup, egui_renderer, size)
      }
   }
}