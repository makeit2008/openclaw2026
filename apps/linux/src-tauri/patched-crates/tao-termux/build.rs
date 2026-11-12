fn main() {
    println!("cargo:rerun-if-env-changed=TERMUX_GTK");
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("android") && std::env::var_os("TERMUX_GTK").is_some() {
        println!("cargo:rustc-cfg=termux_gtk");
    }
}
