use std::iter;
use log::error;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;
use crate::inbuilt::setup::Setup;
use crate::packages::test_render_pipeline::TestRenderPipeline;

pub struct GlobalState<'a> {
   pub setup: Setup<'a>,

   test_render_pipeline: TestRenderPipeline,
}
impl<'a> GlobalState<'a> {
   pub fn new(window: &'a Window) -> GlobalState<'a> {
      let setup_future = async { Setup::new(window).await };
      let setup = pollster::block_on(setup_future);


      let test_render_pipeline = TestRenderPipeline::new(&setup);

      Self {
         setup,
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

   pub fn update_input(&mut self, event: &WindowEvent) -> bool { false }

   pub fn update(&mut self) {}

   pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
      let output = self.setup.surface.get_current_texture()?;
      let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
      let mut encoder = self.setup.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
         label: Some("Render Encoder"),
      });

      {
         self.test_render_pipeline.render_pass(&mut encoder, &view);
      }

      self.setup.queue.submit(iter::once(encoder.finish()));
      output.present();

      Ok(())
   }


}