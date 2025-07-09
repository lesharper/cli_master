// util
#[derive(Debug, Clone)]
pub struct Utility {
    name: String,
    description: String,
    config: UtilityConfig,
}

#[derive(Debug, Clone, Default)]
pub struct UtilityConfig {
    // Добавьте параметры конфигурации для get_dir_tree
    // Например: pub path: Option<String>,
}

impl Utility {
    pub fn new(name: &str, description: &str, config: UtilityConfig) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            config,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}