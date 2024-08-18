use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;
use crate::global_state::GlobalState;


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn resize_to_canvas() {
   use wasm_bindgen::JsCast;
   let window = web_sys::window().unwrap();
   let document = window.document().unwrap();
   let canvas = document.get_element_by_id("wgpu-canvas").unwrap();
   let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();



   // Adjust the canvas size by dividing the width and height by the device pixel ratio
   canvas.set_width((window.inner_width().unwrap().as_f64().unwrap()) as u32);
   canvas.set_height((window.inner_height().unwrap().as_f64().unwrap()) as u32);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
   cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
   }

   let event_loop = EventLoop::new().unwrap();
   let window = WindowBuilder::new().build(&event_loop).unwrap();

   #[cfg(target_arch = "wasm32")]
   {
      // Winit prevents sizing with CSS, so we have to set
      // the size manually when on web.
      use winit::dpi::PhysicalSize;
      let _ = window.request_inner_size(PhysicalSize::new(50, 50));

      use winit::platform::web::WindowExtWebSys;
      web_sys::window()
          .and_then(|win| win.document())
          .and_then(|doc| {
             let dst = doc.get_element_by_id("wasm-example")?;
             let canvas = web_sys::Element::from(window.canvas()?);
             canvas.set_id("wgpu-canvas");

             dst.append_child(&canvas).ok()?;
             Some(())
          })
          .expect("Couldn't append canvas to document body.");
   }

   let mut state = GlobalState::new(&window);
   let mut surface_configured = false;

   event_loop.run(move |event, control_flow| {
      match event {
         Event::WindowEvent {
            ref event,
            window_id,
         } if window_id == state.setup.window.id() => {
            if !state.update_input(event) {
               // UPDATED!
               match event {
                  WindowEvent::CloseRequested
                  | WindowEvent::KeyboardInput {
                     event:
                     KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                     },
                     ..
                  } => control_flow.exit(),

                  WindowEvent::Resized(physical_size) => {

                     #[cfg(not(target_arch="wasm32"))] {
                        log::info!("physical_size: {physical_size:?}");
                        surface_configured = true;
                        state.resize(*physical_size);
                     }


                     #[cfg(target_arch = "wasm32")]
                     {
                        let window = web_sys::window().unwrap();
                        // Get the device pixel ratio
                        let dpr = window.device_pixel_ratio();

                        let nps = &mut physical_size.clone();
                        nps.width = (nps.width as f64 / dpr) as u32;
                        nps.height = (nps.height as f64 / dpr) as u32;

                        log::info!("physical_size: {nps:?}");
                        surface_configured = true;
                        state.resize(*nps);


                        resize_to_canvas();
                     }
                  }

                  WindowEvent::RedrawRequested => {
                     // This tells winit that we want another frame after this one
                     state.setup.window.request_redraw();

                     if !surface_configured {
                        return;
                     }

                     state.update();
                     match state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(
                           wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                        ) => state.resize(state.setup.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                           log::error!("OutOfMemory");
                           control_flow.exit();
                        }

                        // This happens when the frame takes too long to present
                        Err(wgpu::SurfaceError::Timeout) => {
                           log::warn!("Surface timeout")
                        }
                     }
                  }
                  _ => {}
               }
               state.egui_renderer.handle_input(&mut state.setup.window, &event);
            }
         }
         _ => {}
      }
   }).unwrap();
}