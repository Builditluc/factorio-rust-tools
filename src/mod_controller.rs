use std::{
    fs::{self, File},
    io::BufWriter,
    path::PathBuf,
};

use derive_builder::Builder;
use serde_derive::Serialize;
use tracing::debug;

use crate::errors::Result;

const MOD_MANIFEST_NAME: &str = "info.json";

pub struct ModController {
    mods_dir: PathBuf,
}

impl ModController {
    pub fn new(mods_dir: PathBuf) -> ModController {
        ModController { mods_dir }
    }

    pub fn create_mod(&self, manifest: ModManifest) -> Result<Mod> {
        let dir = self
            .mods_dir
            .join(format!("{}_{}", manifest.name, manifest.version));

        debug!("creating mod: {:?}", &dir);
        fs::create_dir_all(&dir)?;

        let json = serde_json::to_string(&manifest)?;
        debug!("Writing mod manifest: {}", &json);
        fs::write(dir.join(MOD_MANIFEST_NAME), &json)?;

        serde_json::to_writer_pretty(
            BufWriter::new(File::create(dir.join(MOD_MANIFEST_NAME))?),
            &manifest,
        )?;

        Ok(Mod { dir })
    }
}

pub struct Mod {
    dir: PathBuf,
}

impl Mod {
    pub fn add_file(self, file_name: &str, contents: &str) -> Result<Self> {
        fs::write(&self.dir.join(file_name), contents.as_bytes())?;
        Ok(self)
    }
}

/// The contents of an `info.json` file in a mod. Described [on the
/// Wiki](https://wiki.factorio.com/Tutorial:Mod_structure#info.json).
#[derive(Builder, Serialize)]
#[builder(setter(into, strip_option))]

pub struct ModManifest {
    pub name: String,
    pub version: String,
    pub title: String,
    pub author: String,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[builder(default = r#""1.1".into()"#)]
    pub factorio_version: String,

    #[builder(default)]
    pub dependencies: Vec<String>,
}
