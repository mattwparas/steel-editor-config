use std::error::Error;

use steel::{
    rvals::Custom,
    steel_vm::ffi::{FFIModule, RegisterFFIFn},
};

steel::declare_module!(build_module);

struct EditorConfig(ec4rs::Properties);

impl Custom for EditorConfig {}

impl EditorConfig {
    fn get_property(&self, property: &str) -> Option<String> {
        self.0
            .get_raw_for_key(property)
            .into_option()
            .map(|x| x.to_owned())
    }
}

fn editor_config_module() -> FFIModule {
    let mut module = FFIModule::new("steel/editorconfig");

    module
        .register_fn("get-property", EditorConfig::get_property)
        .register_fn("config-at-path", editor_config_at_path);

    module
}

#[derive(Debug)]
struct ConfigError(ec4rs::Error);
impl Custom for ConfigError {}
impl Error for ConfigError {}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn editor_config_at_path(path: &str) -> Result<EditorConfig, ConfigError> {
    ec4rs::properties_of(path)
        .map(EditorConfig)
        .map_err(ConfigError)
}

pub fn build_module() -> FFIModule {
    editor_config_module()
}
