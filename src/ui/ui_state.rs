use catppuccin_egui::Theme;
use egui::{CentralPanel, Context, menu, SidePanel, TopBottomPanel, Ui, Vec2, Visuals};
use egui::load::SizedTexture;
use egui::panel::{Side, TopBottomSide};
use egui_plot::{Line, Plot, PlotPoints};
use egui_wgpu::ScreenDescriptor;
use wgpu::{CommandEncoder, Extent3d, TextureView};
use crate::inbuilt::gui_state::EguiRenderer;
use crate::inbuilt::setup::Setup;
use crate::packages::time_package::TimePackage;
use crate::utility::functions::round_to_x_decimals;
use crate::utility::structs::EguiTexturePackage;

pub struct UiState {
   theme: Theme,
}
impl UiState {
   pub fn new() -> Self {

      Self {
         theme: catppuccin_egui::FRAPPE,
      }
   }

   fn ui(&mut self, context: &Context, egui_texture_package: &mut EguiTexturePackage, time_package: &TimePackage) {
      catppuccin_egui::set_theme(&context, self.theme);

      CentralPanel::default().show(context, |ui| {
         ui.group(|ui| {
            menu::bar(ui, |ui| {
               ui.menu_button("File", |ui| {
                  if ui.button("Open").clicked() {
                     // Handle open action
                  }
                  if ui.button("Save").clicked() {
                     // Handle save action
                  }
               });

               ui.menu_button("Edit", |ui| {
                  if ui.button("Undo").clicked() {
                     // Handle undo action
                  }
                  if ui.button("Redo").clicked() {
                     // Handle redo action
                  }
               });

               ui.menu_button("Theme", |ui| {

                  if ui.button(format!("Dark mode {:?}", if context.style().visuals.dark_mode {"Y"} else {"N"} )).clicked() {
                     context.set_visuals(match context.style().visuals.dark_mode {
                        true => {Visuals::light()}
                        false => {Visuals::dark()}
                     });
                  }

                  ui.menu_button("catppuccin", |ui| {
                     if ui.button("FRAPPE").clicked() { self.theme = catppuccin_egui::FRAPPE; };
                     if ui.button("LATTE").clicked() { self.theme = catppuccin_egui::LATTE; }
                     if ui.button("MACCHIATO").clicked() { self.theme = catppuccin_egui::MACCHIATO; }
                     if ui.button("MOCHA").clicked() { self.theme = catppuccin_egui::MOCHA; }
                  });

               });
            });
         });


         self.panels(ui, egui_texture_package, time_package);
      });

   }

   fn panels(&mut self, ui: &mut Ui, egui_texture_package: &mut EguiTexturePackage, time_package: &TimePackage) {
      SidePanel::new(Side::Left, "left")
          .resizable(true)
          .show_inside(ui, |ui| {
             ui.allocate_space(ui.available_size());
          });

      SidePanel::new(Side::Right, "right")
          .resizable(false)
          .max_width(0.0)
          .show_inside(ui, |_ui| {});

      CentralPanel::default() // right panel
          .show_inside(ui, |ui| {
             TopBottomPanel::new(TopBottomSide::Bottom, "bottom")
                 .resizable(true)
                 .show_inside(ui, |ui| {

                    self.bottom_right(ui, time_package);
                    // ui.allocate_space(ui.available_size());
                 });

             CentralPanel::default() // image panel
                 .show_inside(ui, |ui| {
                    ui.set_min_height(1.0);
                    let ms = to_extent(ui.available_size());
                    egui_texture_package.size = ms;

                    let st = SizedTexture::new(
                       egui_texture_package.texture_id,
                       to_v2(egui_texture_package.texture.size())
                    );
                    ui.add(egui::Image::new(st));
                 });
          });
   }

   fn bottom_right(&mut self, ui: &mut Ui, time_package: &TimePackage) {
      self.statistics(ui, time_package)
   }

   fn statistics(&mut self, ui: &mut Ui, time_package: &TimePackage) {
      egui::ScrollArea::vertical()
          .show(ui, |ui| {
             egui::containers::CollapsingHeader::new("Fps")
                 .show(ui, |ui| {
                    ui.group(|ui| {
                       let mut w = ui.available_width();

                       ui.horizontal(|ui| {
                          let tss = round_to_x_decimals(time_package.fps as f32, 1);
                          ui.label(format!("Current fps: {}", if tss % 1.0 == 0.0 {format!("{tss}.0")} else {format!("{tss}")}) );

                          let tss = round_to_x_decimals(time_package.start_time.elapsed().as_secs_f32(), 1);
                          ui.label(format!("Time since start: {}", if tss % 1.0 == 0.0 {format!("{tss}.0")} else {format!("{tss}")}) );

                          w = ui.min_size().x;
                       });

                       let points: PlotPoints = time_package.past_fps.iter().enumerate().map(|(i, &val)| {
                          [i as f64, val]
                       }).collect();


                       let line = Line::new(points);
                       Plot::new("my_plot")
                           .width(w)
                           .view_aspect(2.0)
                           .allow_drag(false)
                           .allow_scroll(false)
                           .allow_zoom(false)
                           .allow_boxed_zoom(false)
                           .show(ui, |plot_ui| plot_ui.line(line));

                    });
                 });
          });


   }

   pub fn render_and_update(&mut self,
    setup: &Setup,
    egui_renderer: &mut EguiRenderer,
    egui_texture_package: &mut EguiTexturePackage,
    view: &TextureView, encoder: &mut CommandEncoder,
    time_package: &TimePackage,
   ) {
      #[allow(unused_assignments)]
      let mut screen_descriptor = ScreenDescriptor { size_in_pixels: [1, 1], pixels_per_point: 0.0 };

      #[cfg(not(target_arch="wasm32"))] {
         screen_descriptor = ScreenDescriptor {
            size_in_pixels: [setup.config.width, setup.config.height],
            pixels_per_point: setup.window.scale_factor() as f32,
         };
      }

      #[cfg(target_arch = "wasm32")]
      {
         screen_descriptor = ScreenDescriptor {
            // size_in_pixels: [setup.config.width, setup.config.height],
            size_in_pixels: [setup.config.width, setup.config.height],
            pixels_per_point: 1.0,
         };
      }

      let run_ui = |context: &Context| {
         self.ui(&context, egui_texture_package, time_package);
      };

      egui_renderer.draw(
         &setup.device,
         &setup.queue,
         encoder,
         &setup.window,
         &view,
         screen_descriptor,
         run_ui,
      );
   }
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