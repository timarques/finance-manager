mod metadata;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;

use std::error::Error;
use std::env;
use std::fs;

fn compile_resources(metadata: &metadata::Metadata) -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;

    let resources_template_file = "resources/resources.gresource.xml.in";
    let content = fs::read_to_string(resources_template_file)?
        .replace("@APP_RESOURCE_PATH@", &metadata.resource_path);

    let resources_output_file = format!("{}/resources.gresource.xml", out_dir);
    fs::write(&resources_output_file, content)?;

    glib_build_tools::compile_resources(
        &["resources"],
        &resources_output_file,
        "compiled.gresource",
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=resources");
    
    let metadata = metadata::Metadata::new();
    metadata.export();

    compile_resources(&metadata)?;

    if env::var("CARGO_INSTALL_ROOT").is_ok() {
        #[cfg(target_os = "linux")]
        linux::Linux::new(metadata)?.install()?;
    }

    #[cfg(target_os = "windows")]
    windows::Windows::new().install()?;

    Ok(())
}