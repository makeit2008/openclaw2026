// Copyright 2014-2021 The winit contributors
// Copyright 2021-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;
#[cfg(any(
  any(target_os = "linux", termux_gtk),
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]
#[path = "linux/mod.rs"]
mod platform;
#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod platform;
#[cfg(all(target_os = "android", not(termux_gtk)))]
#[path = "android/mod.rs"]
mod platform;
#[cfg(target_os = "ios")]
#[path = "ios/mod.rs"]
mod platform;

pub use platform::*;

#[cfg(all(
  not(target_os = "ios"),
  not(target_os = "windows"),
  not(any(target_os = "linux", termux_gtk)),
  not(target_os = "macos"),
  not(all(target_os = "android", not(termux_gtk))),
  not(target_os = "dragonfly"),
  not(target_os = "freebsd"),
  not(target_os = "netbsd"),
  not(target_os = "openbsd"),
))]
compile_error!("The platform you're compiling for is not supported by tao");
