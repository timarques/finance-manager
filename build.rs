use std::env::VarError;
use std::path::PathBuf;
use std::error::Error;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct AppValues {
    id: String,
    resource_path: String,
    icon_name: String,
    title: String,
}

fn get_data_dir(is_root: bool) -> Result<PathBuf, Box<dyn Error>> {
    Ok(if let Ok(data_dir) = env::var("XDG_DATA_HOME") {
        PathBuf::from(data_dir)
    } else if is_root {
        PathBuf::from("/usr/share")
    } else {
        PathBuf::from(env::var("HOME")?).join(".local").join("share")
    })
}

fn install_icon_file(
    app_icon_name: &str,
    app_id: &str,
    data_dir: &PathBuf
) -> Result<PathBuf, Box<dyn Error>> {
    let icons_dir = data_dir.join("icons/hicolor/scalable/apps");
    fs::create_dir_all(&icons_dir)?;
    
    let source_icon = PathBuf::from("resources/icons")
        .join(format!("{}.svg", app_icon_name));
    
    if !source_icon.exists() {
        return Err("Icon file not found".into());
    }

    let target_icon = icons_dir.join(format!("{}.svg", app_id));
    fs::copy(source_icon, &target_icon)?;
    Ok(target_icon)
}

fn install_desktop_file(
    app_id: &str,
    icon_path: &PathBuf,
    exec_path: &str,
    app_title: &str,
    data_dir: &PathBuf
) -> Result<(), Box<dyn Error>> {
    let template_path = PathBuf::from("resources/app.desktop.in");
    if !template_path.exists() {
        return Err("Desktop template file not found".into());
    }

    let content = fs::read_to_string(&template_path)?
        .replace("@APP_ICON@", icon_path.to_str().unwrap())
        .replace("@APP_TITLE@", app_title)
        .replace("@APP_EXEC@", exec_path)
        .replace("@APP_VERSION@", env!("CARGO_PKG_VERSION"))
        .replace("@APP_NAME@", env!("CARGO_PKG_NAME"))
        .replace("@APP_DESCRIPTION@", env!("CARGO_PKG_DESCRIPTION"));

    let apps_dir = data_dir.join("applications");
    fs::create_dir_all(&apps_dir)?;
    
    let desktop_path = apps_dir.join(format!("{}.desktop", app_id));
    fs::write(&desktop_path, content)?;
    Ok(())
}

fn get_exec_path() -> Result<String, VarError> {
    let install_root = env::var("CARGO_INSTALL_ROOT");
    let bin_file_name = env::var("CARGO_BIN_FILE").unwrap_or(env::var("CARGO_PKG_NAME")?);
    
    Ok(if let Ok(install_root) = install_root {
        PathBuf::from(install_root)
            .join("bin")
            .join(&bin_file_name)
            .to_str()
            .unwrap_or(&bin_file_name)
            .to_string()
    } else {
        bin_file_name
    })
}

fn create_app_title() -> Result<String, VarError> {
    let pkg_name = env::var("CARGO_PKG_NAME")?;
    Ok(pkg_name.split('-')
        .filter_map(|word| {
            let mut chars = word.chars();
            chars.next().map(|first| {
                format!("{}{}", first.to_uppercase(), chars.collect::<String>())
            })
        })
        .collect::<Vec<String>>()
        .join(" "))
}

fn create_app_id() -> Result<String, VarError> {
    Ok(format!("pt.tiago_marques.{}", env::var("CARGO_PKG_NAME")?.replace('-', "_")))
}

fn create_app_resource_path(app_id: &str) -> Result<String, VarError> {
    Ok(format!("/{}", app_id.replace('.', "/")))
}

fn install_linux(values: &AppValues) -> Result<(), Box<dyn Error>> {
    let is_root = unsafe { libc::geteuid() == 0 };
    let data_dir = get_data_dir(is_root)?;
    let icon_path = install_icon_file(&values.icon_name, &values.id, &data_dir)?;
    let exec_path = get_exec_path()?;

    install_desktop_file(
        &values.id,
        &icon_path,
        &exec_path,
        &values.title,
        &data_dir
    )?;
    Ok(())
}


fn compile_resources(values: &AppValues) -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;

    let resources_template_file = "resources/resources.gresource.xml.in";
    let content = fs::read_to_string(resources_template_file)?
        .replace("@APP_RESOURCE_PATH@", &values.resource_path);

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
    let app_id = create_app_id()?;
    let app_values = AppValues {
        resource_path: create_app_resource_path(&app_id)?,
        id: app_id,
        icon_name: "pie-chart-svgrepo-com".into(),
        title: create_app_title()?,
    };

    println!("cargo:rerun-if-changed=resources");
    println!("cargo:rustc-env=APP_ID={}", app_values.id);
    println!("cargo:rustc-env=APP_ICON_NAME={}", app_values.icon_name);
    println!("cargo:rustc-env=APP_TITLE={}", app_values.title);
    println!("cargo:rustc-env=APP_RESOURCE_PATH={}", app_values.resource_path);

    compile_resources(&app_values)?;

    if env::var("CARGO_INSTALL_ROOT").is_ok() && env::var("CARGO_CFG_TARGET_OS")? == "linux" {
        install_linux(&app_values)?;
    }

    Ok(())
}