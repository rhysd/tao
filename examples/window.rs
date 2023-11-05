// Copyright 2014-2021 The winit contributors
// Copyright 2021-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use tao::{
  event::{ElementState, Event, KeyEvent, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  keyboard::KeyCode::KeyX,
  window::{Theme, WindowBuilder},
};

#[allow(clippy::single_match)]
fn main() {
  let event_loop = EventLoop::new();

  let mut theme = Theme::Light;
  let mut window = Some(
    WindowBuilder::new()
      .with_title("A fantastic window!")
      .with_inner_size(tao::dpi::LogicalSize::new(300.0, 300.0))
      .with_min_inner_size(tao::dpi::LogicalSize::new(200.0, 200.0))
      .with_theme(Some(theme))
      .build(&event_loop)
      .unwrap(),
  );

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        window_id: _,
        ..
      } => {
        // drop the window to fire the `Destroyed` event
        window = None;
      }
      Event::WindowEvent {
        event: WindowEvent::Destroyed,
        window_id: _,
        ..
      } => {
        *control_flow = ControlFlow::Exit;
      }
      Event::WindowEvent {
        event:
          WindowEvent::KeyboardInput {
            event:
              KeyEvent {
                physical_key: KeyX,
                state: ElementState::Pressed,
                ..
              },
            ..
          },
        ..
      } => {
        let next = if let Theme::Dark = theme {
          Theme::Light
        } else {
          Theme::Dark
        };
        if let Some(w) = &mut window {
          w.set_theme(next);
        }
        println!("Theme changed! {:?} => {:?}", theme, next);
        theme = next;
      }
      Event::MainEventsCleared => {
        if let Some(w) = &window {
          w.request_redraw();
        }
      }
      _ => (),
    }
  });
}
