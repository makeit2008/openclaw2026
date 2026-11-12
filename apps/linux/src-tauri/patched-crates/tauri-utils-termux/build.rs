// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// `tauri::generate_context!` writes its output to `OUT_DIR`, which Cargo only sets for the doc tests
// of packages that have a build script.
fn main() {
    println!("cargo:rerun-if-env-changed=TERMUX_GTK");
    let __target = std::env::var("TARGET").unwrap_or_default();
    if __target.contains("android") && std::env::var_os("TERMUX_GTK").is_some() {
        println!("cargo:rustc-cfg=termux_gtk");
    }
}
