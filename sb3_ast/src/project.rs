use std::{
    io::{Read as _, Write as _},
    path::PathBuf,
};

use anyhow::Result;
use sb_sbity::project::Project as ProjectData;

use crate::{Sprite, Stage};

pub struct Project {
    path: PathBuf,
    sprite: Vec<Sprite>,
    stage: Stage,
    pub project: ProjectData,
}

impl Project {
    pub fn new<P>(path: String) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let project_data_zip = std::fs::File::open(&path)?;
        let zip_data = std::io::BufReader::new(project_data_zip);
        let mut archive = zip::ZipArchive::new(zip_data)?;
        let mut project_data = archive.by_name("project.json")?;
        let mut buffer = String::new();
        let _ = project_data.read_to_string(&mut buffer)?;

        let project = serde_json::from_str::<ProjectData>(&buffer)?;

        let mut sprites = Vec::new();
        let mut stage = None;
        for sprite_or_stage in &project.targets {
            match sprite_or_stage {
                sb_sbity::target::SpriteOrStage::Sprite(sprite) => {
                    sprites.push(Sprite::new(sprite.clone()));
                }
                sb_sbity::target::SpriteOrStage::Stage(stage_inner) => {
                    stage = Some(Stage::new(stage_inner.clone()));
                }
            }
        }

        Ok(Self {
            path: path.into(),
            sprite: sprites,
            stage: stage.unwrap(),
            project: project,
        })
    }

    pub fn flush(&mut self) {
        for sprite in &mut self.sprite {
            sprite.flush();
        }
        self.stage.flush();
        for sprite_or_stage in &mut self.project.targets {
            match sprite_or_stage {
                sb_sbity::target::SpriteOrStage::Sprite(sprite) => {
                    *sprite_or_stage = sb_sbity::target::SpriteOrStage::Sprite(sprite.clone());
                }
                sb_sbity::target::SpriteOrStage::Stage(stage) => {
                    *sprite_or_stage = sb_sbity::target::SpriteOrStage::Stage(stage.clone());
                }
            }
        }
    }

    pub fn zip<P>(&self, out: P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let project_data_zip = std::fs::File::open(&self.path)?;
        let zip_data = std::io::BufReader::new(project_data_zip);
        let mut archive = zip::ZipArchive::new(zip_data)?;
        // archive.file_names()

        let mut out = std::fs::File::create(out)?;
        let mut zip = zip::ZipWriter::new(&mut out);
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        zip.start_file("project.json", options)?;
        zip.write_all(serde_json::to_string(&self.project)?.as_bytes())?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name();
            if name == "project.json" {
                continue;
            }
            let options = zip::write::FileOptions::default().compression_method(file.compression());
            zip.start_file(name, options)?;
            let _ = std::io::copy(&mut file, &mut zip)?;
        }
        zip.finish()?;
        Ok(())
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}
