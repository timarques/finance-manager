#[cfg(target_os = "windows")]
fn compile_winres(icon_path: &str) {
    winres::WindowsResource::new()
        .set_icon(&icon_path)
        .compile()
        .unwrap();
}

fn compile_gresources(resources_directory: &str, gresources_xml: &str) {
    glib_build_tools::compile_resources(
        &[resources_directory],
        &gresources_xml,
        "compiled.gresources",
    );
}

fn export_variables(app_id: &str, app_title: &str, app_icon: &str, app_g_resources_id: &str) {
    println!("cargo:rustc-env=APP_ID={}", app_id);
    println!("cargo:rustc-env=APP_TITLE={}", app_title);
    println!("cargo:rustc-env=APP_G_RESOURCES_ID={}", app_g_resources_id);
    println!("cargo:rustc-env=APP_ICON={}", app_icon);
}

fn main() {
    println!("cargo:rerun-if-env-changed=APP_G_RESOURCES_XML");
    println!("cargo:rerun-if-env-changed=resources");

    if let (
        Ok(app_resources),
        Ok(app_g_resources_xml),
        Ok(app_g_resources_id),
        Ok(app_id),
        Ok(app_icon),
        Ok(app_title),
    ) = (
        std::env::var("APP_RESOURCES"),
        std::env::var("APP_G_RESOURCES_XML"),
        std::env::var("APP_G_RESOURCES_ID"),
        std::env::var("APP_ID"),
        std::env::var("APP_ICON_NAME"),
        std::env::var("APP_TITLE"),
    ) {
        #[cfg(target_os = "windows")]
        compile_winres(&std::env::var("APP_ICON_ICO_PATH").unwrap());

        compile_gresources(&app_resources, &app_g_resources_xml);
        export_variables(&app_id, &app_title, &app_icon, &app_g_resources_id);
    } else if cfg!(debug_assertions) {

        let g_resources_template = include_str!("resources/templates/gresources.xml.template")
            .replace("@APP_G_RESOURCES_ID@", "com/app/debug")
            .replace("@APP_ICON@", "icon.svg");

        let g_resources_file_path = format!("{}/gresources.xml", std::env::var("OUT_DIR").unwrap());
        std::fs::write(&g_resources_file_path, g_resources_template).unwrap();

        compile_gresources("resources", &g_resources_file_path);
        export_variables("com.app.debug", "Debug", "icon", "com/app/debug");
    } else {
        panic!("Missing environment variables");
    }

}