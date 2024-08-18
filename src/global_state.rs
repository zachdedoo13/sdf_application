use std::iter;
use egui::Context;
use egui_wgpu::ScreenDescriptor;
use log::error;
use wgpu::{CommandEncoder, TextureView};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;
use crate::inbuilt::gui_state::EguiRenderer;
use crate::inbuilt::setup::Setup;
use crate::packages::test_gui;
use crate::packages::test_render_pipeline::TestRenderPipeline;

pub struct GlobalState<'a> {
   pub setup: Setup<'a>,

   pub egui_renderer: EguiRenderer,

   test_render_pipeline: TestRenderPipeline,
}
impl<'a> GlobalState<'a> {
   pub fn new(window: &'a Window) -> GlobalState<'a> {
      let setup_future = async { Setup::new(window).await };
      let setup = pollster::block_on(setup_future);

      let egui_renderer = EguiRenderer::new(&setup.device, setup.config.format, None, 1, setup.window);


      let test_render_pipeline = TestRenderPipeline::new(&setup);

      Self {
         setup,
         egui_renderer,
         test_render_pipeline,
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

   pub fn update(&mut self) {}

   pub fn update_gui(&mut self, view: &TextureView, encoder: &mut CommandEncoder) {
      #[allow(unused_assignments)]
      let mut screen_descriptor = ScreenDescriptor { size_in_pixels: [1, 1], pixels_per_point: 0.0 };

      #[cfg(not(target_arch="wasm32"))] {
         screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.setup.config.width, self.setup.config.height],
            pixels_per_point: self.setup.window.scale_factor() as f32,
         };
      }

      #[cfg(target_arch = "wasm32")]
      {
         screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.setup.config.width, self.setup.config.height],
            // pixels_per_point: self.setup.window.scale_factor() as f32,
            pixels_per_point: 1.0, // todo incorrectly scaled, fuzzy in 4k
         };
      }

      let run_ui = |ui: &Context| {
         test_gui::splits(ui);
      };

      self.egui_renderer.draw(
         &self.setup.device,
         &self.setup.queue,
         encoder,
         &self.setup.window,
         &view,
         screen_descriptor,
         run_ui,
      );
   }

   pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
      let output = self.setup.surface.get_current_texture()?;
      let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
      let mut encoder = self.setup.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
         label: Some("Render Encoder"),
      });

      {
         self.test_render_pipeline.render_pass(&mut encoder, &view);
      }

      self.update_gui(&view, &mut encoder);

      self.setup.queue.submit(iter::once(encoder.finish()));
      output.present();

      Ok(())
   }


}