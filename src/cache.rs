#![allow(dead_code)]
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use dirs::cache_dir;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedRates {
    pub base: String,
    pub rates: HashMap<String, f64>,
    pub date: String,
    pub fetched_at: DateTime<Utc>,
}

pub struct Cache {
    root: PathBuf,
}

impl Cache {
    pub fn new() -> Result<Self> {
        let root = cache_dir()
            .context("Cannot find user cache directory")?
            .join("currency-cli");
        create_dir_all(&root)?;
        Ok(Self { root })
    }

    fn file_for_base(&self, base: &str) -> PathBuf {
        self.root.join(format!("rates_{}.json", base.to_uppercase()))
    }

    pub fn save(&self, data: &CachedRates) -> Result<()> {
        let path = self.file_for_base(&data.base);
        let json = serde_json::to_string_pretty(&data)?;
        write(path, json)?;
        Ok(())
    }

    pub fn load(&self, base: &str) -> Result<CachedRates> {
        let path = self.file_for_base(base);
        let content = read_to_string(path)?;
        let parsed: CachedRates = serde_json::from_str(&content)?;
        Ok(parsed)
    }

    pub fn is_fresh(&self, cached: &CachedRates, max_age_minutes: i64) -> bool {
        let age = (Utc::now() - cached.fetched_at).num_minutes();
        age <= max_age_minutes
    }
}