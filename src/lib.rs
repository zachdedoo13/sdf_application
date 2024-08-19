pub mod global_state;
pub mod inbuilt {
   pub mod setup;
   pub mod event_loop;
   pub mod vertex_package;
   pub mod vertex_library;
   pub mod gui_state;
}
pub mod packages {
   pub mod test_render_pipeline;
   pub mod test_gui;
   // pub mod time_package;
}

pub mod utility {
   pub mod functions;
   pub mod macros;
   pub mod structs;
}

pub mod ui {
   pub mod ui_state;
}