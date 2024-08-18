use egui::{Context, Vec2, SidePanel, Ui, CentralPanel, TopBottomPanel};
use egui::load::SizedTexture;
use egui::panel::{Side, TopBottomSide};
use wgpu::Extent3d;
use crate::utility::structs::EguiTexturePackage;

pub fn splits(context: &Context, texture_package: &mut EguiTexturePackage) {
   catppuccin_egui::set_theme(&context, catppuccin_egui::FRAPPE);


   SidePanel::new(Side::Left, "left")
       .resizable(true)
       .show(context, |ui| {
          test_contents(ui);
          ui.allocate_space(ui.available_size());
       });

   SidePanel::new(Side::Right, "right")
       .resizable(false)
       .max_width(0.0)
       .show(context, |_ui| {});

   CentralPanel::default() // right panel
       .show(context, |ui| {
          TopBottomPanel::new(TopBottomSide::Bottom, "bottom")
              .resizable(true)
              .show_inside(ui, |ui| {
                 test_contents(ui);
              });

          CentralPanel::default() // image panel
              .show_inside(ui, |ui| {
                 ui.set_min_height(1.0);
                 let ms = to_extent(ui.available_size());
                 texture_package.size = ms;

                 let st = SizedTexture::new(
                    texture_package.texture_id,
                    to_v2(texture_package.texture.size())
                 );
                 ui.add(egui::Image::new(st));
              });
       });
}

fn to_v2(extent: Extent3d) -> Vec2 {
   Vec2::new(extent.width as f32, extent.height as f32)
}

fn to_extent(vec2: Vec2) -> Extent3d {
   Extent3d {
      width: vec2.x as u32,
      height: vec2.y as u32,
      depth_or_array_layers: 1,
   }
}

fn test_contents(ui: &mut Ui) {
   ui.vertical(|ui| {
      ui.label("Scroll Area Content");

      // Add sliders
      ui.add(egui::Slider::new(&mut 0.0, 0.0..=100.0).text("Slider 1"));
      ui.add(egui::Slider::new(&mut 50.0, 0.0..=100.0).text("Slider 2"));

      // Add horizontal UI elements
      ui.horizontal(|ui| {
         ui.label("Horizontal UI:");
         let _ = ui.button("Button 1");
         let _ = ui.button("Button 2");
      });

      // Add more content if needed
      ui.label("More content...");
   });
}