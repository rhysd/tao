// Copyright 2014-2021 The winit contributors
// Copyright 2021-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

#![cfg(target_os = "windows")]

use windows::Win32::{
  Foundation::{HANDLE, HWND},
  UI::WindowsAndMessaging::HMENU,
};

pub(crate) use self::{
  event_loop::{
    EventLoop, EventLoopProxy, EventLoopWindowTarget, PlatformSpecificEventLoopAttributes,
  },
  icon::WinIcon,
  keycode::{keycode_from_scancode, keycode_to_scancode},
  monitor::{MonitorHandle, VideoMode},
  window::Window,
};

pub use self::window::hit_test;

pub use self::icon::WinIcon as PlatformIcon;

use crate::{event::DeviceId as RootDeviceId, icon::Icon, keyboard::Key, window::Theme};
mod keycode;

#[non_exhaustive]
#[derive(Clone)]
pub enum Parent {
  None,
  ChildOf(HWND),
  OwnedBy(HWND),
}

#[derive(Clone)]
pub struct PlatformSpecificWindowBuilderAttributes {
  pub parent: Parent,
  pub menu: Option<HMENU>,
  pub taskbar_icon: Option<Icon>,
  pub skip_taskbar: bool,
  pub window_classname: String,
  pub no_redirection_bitmap: bool,
  pub drag_and_drop: bool,
  pub preferred_theme: Option<Theme>,
  pub decoration_shadow: bool,
  pub rtl: bool,
}

impl Default for PlatformSpecificWindowBuilderAttributes {
  fn default() -> Self {
    Self {
      parent: Parent::None,
      menu: None,
      taskbar_icon: None,
      no_redirection_bitmap: false,
      drag_and_drop: true,
      preferred_theme: None,
      skip_taskbar: false,
      window_classname: "Window Class".to_string(),
      decoration_shadow: true,
      rtl: false,
    }
  }
}

unsafe impl Send for PlatformSpecificWindowBuilderAttributes {}
unsafe impl Sync for PlatformSpecificWindowBuilderAttributes {}

// Cursor name in UTF-16. Used to set cursor in `WM_SETCURSOR`.
#[derive(Debug, Clone, Copy)]
pub struct Cursor(pub *const u16);
unsafe impl Send for Cursor {}
unsafe impl Sync for Cursor {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(isize);

impl DeviceId {
  pub unsafe fn dummy() -> Self {
    DeviceId(0)
  }
}

impl DeviceId {
  pub fn persistent_identifier(&self) -> Option<String> {
    if self.0 != 0 {
      raw_input::get_raw_input_device_name(HANDLE(self.0))
    } else {
      None
    }
  }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum OsError {
  #[allow(unused)]
  CreationError(&'static str),
  IoError(std::io::Error),
}
impl std::error::Error for OsError {}

impl std::fmt::Display for OsError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      OsError::CreationError(e) => f.pad(e),
      OsError::IoError(e) => f.pad(&e.to_string()),
    }
  }
}

// Constant device ID, to be removed when this backend is updated to report real device IDs.
const DEVICE_ID: RootDeviceId = RootDeviceId(DeviceId(0));

fn wrap_device_id(id: isize) -> RootDeviceId {
  RootDeviceId(DeviceId(id))
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyEventExtra {
  pub text_with_all_modifiers: Option<&'static str>,
  pub key_without_modifiers: Key<'static>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(isize);
unsafe impl Send for WindowId {}
unsafe impl Sync for WindowId {}

impl WindowId {
  pub unsafe fn dummy() -> Self {
    WindowId(0)
  }
}

#[macro_use]
mod util;
mod dark_mode;
mod dpi;
mod drop_handler;
mod event_loop;
mod icon;
mod keyboard;
mod keyboard_layout;
mod minimal_ime;
mod monitor;
mod raw_input;
mod window;
mod window_state;
