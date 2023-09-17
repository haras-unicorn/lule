use crate::scheme::*;
use colored::*;

pub fn concatinate(scheme: &mut Scheme) {
    let project_dirs = directories::ProjectDirs::from("com", "Lule", "Lule")
        .unwrap_or_else(|| panic!("{} {}", "error:".red().bold(), "Failed to get project dirs"));

    let cache_dir = project_dirs.cache_dir().to_path_buf();
    let config_dir = project_dirs.config_dir().to_path_buf();
    let config_path = {
        let mut config_path = config_dir.clone();
        config_path.push("lule");
        config_path
    };

    scheme.set_theme(Some("dark".to_string()));
    scheme.set_config(Some(config_path.into_os_string().into_string().unwrap()));
    scheme.set_cache(Some(cache_dir.into_os_string().into_string().unwrap()));
    scheme.set_palette(Some("pigment".to_string()));
}
