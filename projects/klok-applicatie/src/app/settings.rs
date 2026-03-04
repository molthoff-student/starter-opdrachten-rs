use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const SETTINGS_FILE: &str = "settings.json";

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub font_scale: f32,
    pub window_width: u32,
    pub window_height: u32,
    pub autosave: bool,
    pub exitsave: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            font_scale: 1.0,
            window_width: 1080,
            window_height: 768,
            autosave: true,
            exitsave: true,
        }
    }
}

impl AppSettings {
    pub fn path() -> Result<PathBuf, Box<dyn Error>> {
        let mut path = env::current_exe()?;
        path.pop();
        path.push(SETTINGS_FILE);

        if let Some(path_str) = path.to_str() {
            println!("Settings file resides at:\n{}", path_str);
        } else {
            println!("Couldn't create path buffer.");
        }

        Ok(path)
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = Self::path()?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let settings = serde_json::from_reader(reader)?;
        println!("Loaded settings file.");
        Ok(settings)
    }

    pub fn init() -> AppSettings {
        AppSettings::load().unwrap_or_else(|_| {
            let app_settings = AppSettings::default();
            println!("No settings file found. Creating and saving default...");
            app_settings.save().expect("Couldn't initialize settings.");
            app_settings
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::path()?;
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, self)?;
        println!("Saved settings file.");
        Ok(())
    }
}
