// util.rs
use color_eyre::Result;
use std::fs::{self};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Utility {
    name: String,
    description: String,
    pub config: UtilityConfig,
}

#[derive(Debug, Clone, Default)]
pub struct UtilityConfig {
    pub path: Option<String>,
    pub max_depth: Option<usize>,
    pub output: Option<String>, // Для хранения результата get_dir_tree
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

    pub fn config(&self) -> &UtilityConfig {
        &self.config
    }

    pub fn run_get_dir_tree(&mut self) -> Result<()> {
        let path = self
            .config
            .path
            .as_ref()
            .ok_or_else(|| color_eyre::eyre::eyre!("Directory path is not specified"))?;
        let output = Self::build_dir_tree(Path::new(path), 0)?;
        self.config.output = Some(output);
        Ok(())
    }

    fn build_dir_tree(path: &Path, depth: usize) -> Result<String> {
        let mut output = String::new();
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().into_owned();
            let indent = "-".repeat(depth * 2 + 1); // Два тире на уровень вложенности
            output.push_str(&format!("{} {}\n", indent, file_name));

            if entry.file_type()?.is_dir() {
                let sub_path = entry.path();
                output.push_str(&Self::build_dir_tree(&sub_path, depth + 1)?);
            }
        }
        Ok(output)
    }

    pub fn copy_output_to_clipboard(&self) -> Result<()> {
        let output = self
            .config
            .output
            .as_ref()
            .ok_or_else(|| color_eyre::eyre::eyre!("No output to copy"))?;
        let mut clipboard = arboard::Clipboard::new()?;
        clipboard.set_text(output)?;
        Ok(())
    }
}
