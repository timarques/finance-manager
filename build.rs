#[cfg(target_os = "windows")]
fn compile_winres() {
    let icon_path = std::env::var("APP_ICON_ICO_PATH").unwrap();
    winres::WindowsResource::new()
        .set_icon(&icon_path)
        .compile()
        .unwrap();
}

fn main() {
    println!("cargo:rerun-if-env-changed=APP_G_RESOURCES_XML");

    if cfg!(target_os = "windows") {
        compile_winres();
    }

    glib_build_tools::compile_resources(
        &[std::env::var("APP_RESOURCES").unwrap()],
        &std::env::var("APP_G_RESOURCES_XML").unwrap(),
        "compiled.gresource",
    );

    println!("cargo:rustc-env=APP_ID={}", std::env::var("APP_ID").unwrap());
    println!("cargo:rustc-env=APP_TITLE={}", std::env::var("APP_TITLE").unwrap());
    println!("cargo:rustc-env=APP_G_RESOURCES_ID={}", std::env::var("APP_G_RESOURCES_ID").unwrap());
}