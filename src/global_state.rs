use wgpu::Extent3d;
use log::error;
use std::iter;
use winit::dpi::{PhysicalSize};
use winit::event::WindowEvent;
use winit::window::Window;
use crate::inbuilt::gui_state::EguiRenderer;
use crate::inbuilt::setup::Setup;
use crate::packages::test_render_pipeline::TestRenderPipeline;
use crate::packages::time_package::TimePackage;
use crate::ui::ui_state::UiState;
use crate::utility::structs::EguiTexturePackage;

pub struct GlobalState<'a> {
   pub setup: Setup<'a>,
   pub egui_renderer: EguiRenderer,

   ui_state: UiState,

   egui_texture_package: EguiTexturePackage,

   time_package: TimePackage,

   test_render_pipeline: TestRenderPipeline,
}
impl<'a> GlobalState<'a> {
   pub fn new(window: &'a Window) -> GlobalState<'a> {
      let setup = pollster::block_on(async { Setup::new(window).await });

      let mut egui_renderer = EguiRenderer::new(&setup.device, setup.config.format, None, 1, setup.window);

      let test_render_pipeline = TestRenderPipeline::new(&setup);

      let egui_texture_package = EguiTexturePackage::new(&setup, &mut egui_renderer, Extent3d {
         width: 250,
         height: 250,
         depth_or_array_layers: 1,
      });

      let ui_state = UiState::new();

      let time_package = TimePackage::new();

      Self {
         setup,
         egui_renderer,
         test_render_pipeline,
         egui_texture_package,
         ui_state,
         time_package,
      }
   }


   pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
      if new_size.width > 0 && new_size.height > 0 {
         self.setup.size = new_size;
         self.setup.config.width = new_size.width;
         self.setup.config.height = new_size.height;
         self.setup.surface.configure(&self.setup.device, &self.setup.config);

         error!("WINDOW SIZE -> {:?}", self.setup.size);
      }
   }

   pub fn update_input(&mut self, _event: &WindowEvent) -> bool { false }

   pub fn update(&mut self) {
      self.egui_texture_package.update(&self.setup, &mut self.egui_renderer);
      self.time_package.update();
   }

   pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
      let output = self.setup.surface.get_current_texture()?;
      let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
      let mut encoder = self.setup.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
         label: Some("Render Encoder"),
      });

      {
         self.test_render_pipeline.render_pass(&mut encoder, &self.egui_texture_package.view);
      }


      // self.update_gui(&view, &mut encoder);
      self.ui_state.render_and_update(&self.setup, &mut self.egui_renderer, &mut self.egui_texture_package, &view, &mut encoder, &self.time_package);

      self.setup.queue.submit(iter::once(encoder.finish()));
      output.present();

      Ok(())
   }
}