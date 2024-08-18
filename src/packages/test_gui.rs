use egui::{Context};
use egui::panel::{Side, TopBottomSide};

pub fn splits(context: &Context) {
   egui::containers::SidePanel::new(Side::Left, "left")
       .resizable(true)
       .show(context, |ui| {
          ui.label("test");
          ui.allocate_space(ui.available_size());
       });

   egui::containers::TopBottomPanel::new(TopBottomSide::Bottom, "bottom")
       .resizable(true)
       .show(context, |ui| {
          ui.label("Shader render settings");


          egui::ScrollArea::vertical().show(ui, |ui| {
             for i in 0..20 {
                ui.add(egui::Slider::new(&mut 4, 0..=(i * 19)));
             }
          });
       });
}