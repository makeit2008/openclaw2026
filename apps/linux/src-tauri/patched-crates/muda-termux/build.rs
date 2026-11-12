fn main() {
    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_GTK3");
    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_GTK4");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let termux_gtk = target_os == "android" && std::env::var_os("TERMUX_GTK").is_some();
    if termux_gtk {
        println!("cargo:rustc-cfg=termux_gtk");
    }
    let gtk_platform = matches!(
        target_os.as_str(),
        "linux" | "dragonfly" | "freebsd" | "netbsd" | "openbsd"
    ) || termux_gtk;

    if gtk_platform
        && std::env::var_os("CARGO_FEATURE_GTK3").is_some()
        && std::env::var_os("CARGO_FEATURE_GTK4").is_some()
    {
        println!(
            "cargo::warning=features `gtk3` and `gtk4` are enabled together; Muda will use the `gtk4` backend"
        );
    }
}
