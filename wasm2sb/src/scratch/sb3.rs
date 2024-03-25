use anyhow::Result;
use async_zip::ZipEntryBuilder;
use futures::{io::Cursor, AsyncReadExt};
use futures_lite as futures;
use sb_sbity::project::Project;

#[derive(Debug)]
pub struct ProjectZip {
    path: String,
    pub project: Project,
    buff: Vec<u8>,
}

// impl Clone for ProjectZip {
//     fn clone(&self) -> Self {
//         let project_str = serde_json::to_string(&self.project).unwrap();
//         let project = serde_json::from_str(&project_str).unwrap();

//         Self {
//             path: self.path.clone(),
//             project,
//             buff: self.buff.clone(),
//         }
//     }
// }

impl ProjectZip {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(path: String) -> Result<Self> {
        use std::io::Read as _;

        let project_data_zip = std::fs::File::open(&path)?;
        let zip_data = std::io::BufReader::new(project_data_zip);
        Self::new_from_data(
            path,
            zip_data.bytes().map(|i| i.unwrap()).collect::<Vec<u8>>(),
        )
    }

    pub fn new_from_data(path: String, bytes: Vec<u8>) -> Result<Self> {
        let path = path;
        let cursor = Cursor::new(bytes.clone());

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

        let mut reader =
            futures::future::block_on(async { archive.reader_without_entry(index).await })?;

        let mut buffer = Vec::new();
        futures::future::block_on(async { reader.read_to_end(&mut buffer).await })?;

        let json = std::str::from_utf8(&buffer)?;

        let project = serde_json::from_str::<Project>(json).unwrap();
        Ok(Self {
            path,
            project,
            buff: bytes,
        })
    }

    pub fn zip(&self) -> Result<Vec<u8>> {
        let json = serde_json::to_string(&self.project)?;

        let cursor = Cursor::new(self.buff.clone());

        let mut archive = futures::future::block_on(async {
            async_zip::base::read::seek::ZipFileReader::new(cursor).await
        })?;

        let mut buff = Vec::new();
        let mut writer = async_zip::base::write::ZipFileWriter::new(&mut buff);

        for i in 0..archive.file().entries().len() {
            let filename = archive
                .file()
                .entries()
                .get(i)
                .unwrap()
                .filename()
                .as_str()?
                .to_string();

            let buffer = if filename == "project.json" {
                json.as_bytes().to_vec()
            } else {
                let mut reader =
                    futures::future::block_on(async { archive.reader_without_entry(i).await })?;

                let mut buffer = Vec::new();
                futures::future::block_on(async { reader.read_to_end(&mut buffer).await })?;
                buffer
            };

            let entry = archive.file().entries().get(i).unwrap();

            let builder = ZipEntryBuilder::new(entry.filename().to_owned(), entry.compression());
            futures::future::block_on(async {
                writer.write_entry_whole(builder, buffer.as_slice()).await
            })?;
        }

        futures::future::block_on(async { writer.close().await })?;

        Ok(buff)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn zip_file<P>(&self, out: P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        use std::io::Write as _;

        let zipped = self.zip()?;

        let mut out = std::fs::File::create(out)?;

        out.write_all(&zipped)?;

        Ok(())
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}
