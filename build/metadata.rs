#[derive(Debug, Clone)]
pub struct Metadata {
    pub id: &'static str,
    pub name: &'static str,
    pub resource_path: &'static str,
    pub icon_name: &'static str,
    pub title: &'static str,
}

impl Metadata {

    pub fn new() -> Self {
        let name = env!("CARGO_PKG_NAME");
        let icon_name = "money-svgrepo-com";
        let id = Self::create_id(name);
        let title = Self::create_title(name);
        let resource_path = Self::create_resource_path(id);

        Self {
            name,
            id,
            title,
            resource_path,
            icon_name,
        }
    }

    fn create_id(name: &str) -> &'static str {
        let id_string = format!("pt.tiago_marques.{}", name.replace('-', "_"));
        id_string.leak()
    }

    fn create_title(name: &str) -> &'static str {
        let title_string = name.split('-')
            .filter_map(|word| {
                let mut chars = word.chars();
                chars.next().map(|first| {
                    format!("{}{}", first.to_uppercase(), chars.collect::<String>())
                })
            })
            .collect::<Vec<String>>()
            .join(" ");

        title_string.leak()
    }

    fn create_resource_path(id: &str) -> &'static str {
        let resource_path = format!("/{}", id.replace('.', "/"));
        resource_path.leak()
    }

    pub fn export(&self) {
        println!("cargo:rustc-env=APP_ID={}", self.id);
        println!("cargo:rustc-env=APP_NAME={}", self.name);
        println!("cargo:rustc-env=APP_ICON_NAME={}", self.icon_name);
        println!("cargo:rustc-env=APP_TITLE={}", self.title);
        println!("cargo:rustc-env=APP_RESOURCE_PATH={}", self.resource_path);
    }

}