use super::config::AppSettings;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct SettingsManager {
    settings: Arc<RwLock<AppSettings>>,
    config_path: PathBuf,
}

impl SettingsManager {
    pub fn new() -> Self {
        let config_path = Self::get_config_path();
        let settings = Self::load_settings(&config_path);

        Self {
            settings: Arc::new(RwLock::new(settings)),
            config_path,
        }
    }

    fn get_config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("kubespark");

        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }

        config_dir.join("settings.json")
    }

    fn load_settings(path: &PathBuf) -> AppSettings {
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(settings) => settings,
                    Err(e) => {
                        eprintln!("Failed to parse settings: {}, using defaults", e);
                        AppSettings::default()
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read settings: {}, using defaults", e);
                    AppSettings::default()
                }
            }
        } else {
            AppSettings::default()
        }
    }

    pub fn save_settings(&self) -> Result<(), String> {
        let settings = self
            .settings
            .read()
            .map_err(|e| format!("Failed to read settings: {}", e))?;

        let json = serde_json::to_string_pretty(&*settings)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write(&self.config_path, json)
            .map_err(|e| format!("Failed to write settings: {}", e))?;

        Ok(())
    }

    pub fn get_settings(&self) -> AppSettings {
        self.settings.read().unwrap().clone()
    }

    pub fn update_settings<F>(&self, update_fn: F) -> Result<(), String>
    where
        F: FnOnce(&mut AppSettings),
    {
        {
            let mut settings = self
                .settings
                .write()
                .map_err(|e| format!("Failed to write settings: {}", e))?;
            update_fn(&mut settings);
        }

        self.save_settings()
    }

    pub fn reset_to_defaults(&self) -> Result<(), String> {
        {
            let mut settings = self
                .settings
                .write()
                .map_err(|e| format!("Failed to write settings: {}", e))?;
            *settings = AppSettings::default();
        }

        self.save_settings()
    }
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self::new()
    }
}
