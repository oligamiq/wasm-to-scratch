use anyhow::Result;
use futures::{io::Cursor, AsyncReadExt};
use sb_sbity::project::Project;
use futures_lite as futures;

pub struct ProjectZip {
    path: String,
    pub project: Project,
}

impl ProjectZip {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(path: String) -> Result<Self> {
        use std::io::Read as _;

        let project_data_zip = std::fs::File::open(&path)?;
        let zip_data = std::io::BufReader::new(project_data_zip);
        Self::new_from_data(path, zip_data.bytes().map(|i| i.unwrap()).collect::<Vec<u8>>())
    }

    pub fn new_from_data(path: String, bytes: Vec<u8>) -> Result<Self> {
        let path = path;
        let cursor = Cursor::new(bytes);

        let mut archive = futures::future::block_on(async {
            async_zip::base::read::seek::ZipFileReader::new(cursor).await
        })?;

        let index = match archive.file().entries().iter().position(|entry| {
            match entry.filename().as_str() {
                Ok("project.json") => true,
                _ => false,
            }
        }) {
            Some(index) => index,
            None => return Err(anyhow::anyhow!("project.json not found")),
        };

        let mut reader = futures::future::block_on(async {
            archive.reader_without_entry(index).await
        })?;

        let mut buffer = Vec::new();
        futures::future::block_on(async {
            reader.read_to_end(&mut buffer).await
        })?;

        let json = std::str::from_utf8(&buffer)?;

        let project = serde_json::from_str::<Project>(json).unwrap();
        Ok(Self { path, project })
    }

    // #[cfg(not(target_arch = "wasm32"))]
    // pub fn zip<P>(&self, out: P) -> Result<()>
    // where
    //     P: AsRef<std::path::Path>,
    // {
    //     let project_data_zip = std::fs::File::open(&self.path)?;
    //     let zip_data = std::io::BufReader::new(project_data_zip);
    //     let mut archive = zip::ZipArchive::new(zip_data)?;
    //     // archive.file_names()

    //     let mut out = std::fs::File::create(out)?;
    //     let mut zip = zip::ZipWriter::new(&mut out);
    //     let options =
    //         zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    //     zip.start_file("project.json", options)?;
    //     zip.write_all(serde_json::to_string(&self.project)?.as_bytes())?;
    //     for i in 0..archive.len() {
    //         let mut file = archive.by_index(i)?;
    //         let name = file.name();
    //         if name == "project.json" {
    //             continue;
    //         }
    //         let options = zip::write::FileOptions::default().compression_method(file.compression());
    //         zip.start_file(name, options)?;
    //         let _ = std::io::copy(&mut file, &mut zip)?;
    //     }
    //     zip.finish()?;
    //     Ok(())
    // }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}
