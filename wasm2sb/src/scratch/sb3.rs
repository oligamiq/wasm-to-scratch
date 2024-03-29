use std::{collections::HashMap, sync::Arc};

use async_zip::ZipEntryBuilder;
use futures::{io::Cursor, AsyncReadExt};
use futures_lite as futures;
use miette::Result;
use parking_lot::RwLock;
use sb_itchy::{
    build_context::{GlobalVarListContext, TargetContext},
    target::SpriteBuilder,
    uid::Uid,
};
use sb_sbity::{comment::Comment, project::Project, target::SpriteOrStage};

use crate::{error::Wasm2SbError, util::get_preview_rect_from_block};

use super::rewrite_dependency::rewrite_list;

pub type CommentMap = HashMap<Uid, Comment>;

#[derive(Debug)]
pub struct ProjectZip {
    path: String,
    pub project: Arc<RwLock<Project>>,
    buff: Vec<u8>,
    x: i32,
    y: i32,
    target_context: TargetContextWrapper,
    comment_buff: CommentMap,
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

        use crate::error::Wasm2SbError;

        let project_data_zip = match std::fs::File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                    bad_bit: (53, 66).into(),
                }
                .into())
            }
        };
        let zip_data = std::io::BufReader::new(project_data_zip);
        Self::new_from_data(
            path,
            zip_data.bytes().map(|i| i.unwrap()).collect::<Vec<u8>>(),
        )
    }

    pub fn new_from_data(path: String, bytes: Vec<u8>) -> Result<Self> {
        let path = path;
        let cursor = Cursor::new(bytes.clone());

        let mut archive = match futures::future::block_on(async {
            async_zip::base::read::seek::ZipFileReader::new(cursor).await
        }) {
            Ok(archive) => archive,
            Err(_) => {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                    bad_bit: (53, 66).into(),
                }
                .into())
            }
        };

        let index = match archive.file().entries().iter().position(|entry| {
            match entry.filename().as_str() {
                Ok("project.json") => true,
                _ => false,
            }
        }) {
            Some(index) => index,
            None => {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                    bad_bit: (53, 66).into(),
                }
                .into())
            }
        };

        let mut reader =
            match futures::future::block_on(async { archive.reader_without_entry(index).await }) {
                Ok(reader) => reader,
                Err(_) => {
                    return Err(Wasm2SbError {
                        src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                        bad_bit: (53, 66).into(),
                    }
                    .into())
                }
            };

        let mut buffer = Vec::new();
        match futures::future::block_on(async { reader.read_to_end(&mut buffer).await }) {
            Ok(_) => (),
            Err(e) => {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", e.to_string()),
                    bad_bit: (53, 66).into(),
                }
                .into())
            }
        };

        let json = match std::str::from_utf8(&buffer) {
            Ok(json) => json,
            Err(_) => {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                    bad_bit: (53, 66).into(),
                }
                .into())
            }
        };

        let mut project = serde_json::from_str::<Project>(json).unwrap();

        let mut sprite = None;

        for target in project.targets.iter_mut() {
            match target {
                SpriteOrStage::Sprite(sprite_impl) => {
                    sprite = Some(sprite_impl);
                    break;
                }
                SpriteOrStage::Stage(stage_impl) => {
                    rewrite_list(&mut stage_impl.target.lists);
                }
            }
        }

        if sprite.is_none() {
            let sprite_builder = SpriteBuilder::default();
            let new_sprite = sprite_builder.build(
                &mut Vec::default(),
                &GlobalVarListContext {
                    vars: Default::default(),
                    lists: Default::default(),
                },
                &HashMap::default(),
            );
            project.targets.push(SpriteOrStage::Sprite(new_sprite));
            sprite = project.targets.last_mut().and_then(|i| match i {
                SpriteOrStage::Sprite(sprite) => Some(sprite),
                _ => None,
            });
            if sprite.is_none() {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                    bad_bit: (53, 66).into(),
                }
                .into());
            }
        }

        let (left_x, _right_x, top_y, _bottom_y) =
            get_preview_rect_from_block(&sprite.unwrap().target.blocks);

        Ok(Self {
            path,
            target_context: TargetContextWrapper::new_from_sb(&project),
            project: Arc::new(RwLock::new(project)),
            buff: bytes,
            y: top_y as i32,
            x: (left_x - 2000) as i32,
            comment_buff: HashMap::new(),
        })
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn update_y(&mut self, y: i32) {
        self.y += y;
    }

    pub fn zip(&self) -> Result<Vec<u8>> {
        let json = match serde_json::to_string(&*self.project.read()) {
            Ok(json) => json,
            Err(_) => {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                    bad_bit: (53, 66).into(),
                }
                .into())
            }
        };

        let cursor = Cursor::new(self.buff.clone());

        let mut archive = match futures::future::block_on(async {
            async_zip::base::read::seek::ZipFileReader::new(cursor).await
        }) {
            Ok(archive) => archive,
            Err(_) => {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                    bad_bit: (53, 66).into(),
                }
                .into())
            }
        };

        let mut buff = Vec::new();
        let mut writer = async_zip::base::write::ZipFileWriter::new(&mut buff);

        for i in 0..archive.file().entries().len() {
            let filename = match archive.file().entries().get(i).unwrap().filename().as_str() {
                Ok(filename) => filename,
                Err(_) => {
                    return Err(Wasm2SbError {
                        src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                        bad_bit: (53, 66).into(),
                    }
                    .into())
                }
            }
            .to_string();

            let buffer = if filename == "project.json" {
                json.as_bytes().to_vec()
            } else {
                let mut reader =
                    match futures::future::block_on(async { archive.reader_without_entry(i).await }) {
                        Ok(reader) => reader,
                        Err(_) => {
                            return Err(Wasm2SbError {
                                src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                                bad_bit: (53, 66).into(),
                            }
                            .into())
                        }
                    };

                let mut buffer = Vec::new();
                match futures::future::block_on(async { reader.read_to_end(&mut buffer).await }) {
                    Ok(_) => (),
                    Err(_) => {
                        return Err(Wasm2SbError {
                            src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                            bad_bit: (53, 66).into(),
                        }
                        .into())
                    }
                };
                buffer
            };

            let entry = archive.file().entries().get(i).unwrap();

            let builder = ZipEntryBuilder::new(entry.filename().to_owned(), entry.compression());
            match futures::future::block_on(async {
                writer.write_entry_whole(builder, buffer.as_slice()).await
            }) {
                Ok(_) => (),
                Err(_) => {
                    return Err(Wasm2SbError {
                        src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                        bad_bit: (53, 66).into(),
                    }
                    .into())
                }
            };
        }

        match futures::future::block_on(async { writer.close().await }) {
            Ok(_) => (),
            Err(_) => {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                    bad_bit: (53, 66).into(),
                }
                .into())
            }
        };

        Ok(buff)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn zip_file<P>(&self, out: P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        use std::io::Write as _;

        let out = out.as_ref();

        if out.parent().is_some() {
            if out.parent().unwrap().parent().is_some_and(|i| i.exists()) {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                    bad_bit: (53, 66).into(),
                }.into());
            }
            if out.parent().is_some() && !out.parent().unwrap().exists() {
                match std::fs::create_dir_all(out.parent().unwrap()) {
                    Ok(_) => (),
                    Err(_) => {
                        return Err(Wasm2SbError {
                            src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                            bad_bit: (53, 66).into(),
                        }
                        .into())
                    }
                };
            }
        }

        if out.exists() {
            match std::fs::remove_file(&out) {
                Ok(_) => (),
                Err(_) => {
                    return Err(Wasm2SbError {
                        src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                        bad_bit: (53, 66).into(),
                    }
                    .into())
                }
            };
        }

        let zipped = self.zip()?;

        let mut out = match std::fs::File::create(out) {
            Ok(file) => file,
            Err(_) => {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                    bad_bit: (53, 66).into(),
                }
                .into())
            }
        };

        match out.write_all(&zipped) {
            Ok(_) => (),
            Err(_) => {
                return Err(Wasm2SbError {
                    src: miette::NamedSource::new("sb3.rs", "source\n  text\n    here".into()),
                    bad_bit: (53, 66).into(),
                }
                .into())
            }
        };

        Ok(())
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}

pub struct TargetContextWrapper {
    global_vars: HashMap<String, Uid>,
    global_lists: HashMap<String, Uid>,
    this_sprite_vars: HashMap<String, Uid>,
    this_sprite_lists: HashMap<String, Uid>,
    all_broadcasts: HashMap<String, Uid>,
    pub target_context: Vec<Box<TargetContext<'static>>>,
}

impl TargetContextWrapper {
    pub fn new(
        global_vars: HashMap<String, Uid>,
        global_lists: HashMap<String, Uid>,
        this_sprite_vars: HashMap<String, Uid>,
        this_sprite_lists: HashMap<String, Uid>,
        all_broadcasts: HashMap<String, Uid>,
    ) -> Self {
        Self {
            global_vars,
            global_lists,
            this_sprite_vars,
            this_sprite_lists,
            all_broadcasts,
            target_context: Vec::new(),
        }
    }

    pub fn get_target_context(&mut self) -> &'static mut TargetContext<'static> {
        let target_context: &'static mut TargetContext<'static> =
            Box::leak(Box::new(TargetContext {
                global_vars: Box::leak(Box::new(self.global_vars.clone())),
                global_lists: Box::leak(Box::new(self.global_lists.clone())),
                this_sprite_vars: Box::leak(Box::new(self.this_sprite_vars.clone())),
                this_sprite_lists: Box::leak(Box::new(self.this_sprite_lists.clone())),
                all_broadcasts: Box::leak(Box::new(self.all_broadcasts.clone())),
            }));

        let box_target_ctx = unsafe { Box::from_raw(target_context) };

        self.target_context.push(box_target_ctx);

        return target_context;
    }

    pub fn drop_target_context_all(&mut self) {
        self.target_context.clear();
    }

    pub fn get_mut_global_vars(&mut self) -> &mut HashMap<String, Uid> {
        self.drop_target_context_all();
        &mut self.global_vars
    }

    pub fn get_mut_global_lists(&mut self) -> &mut HashMap<String, Uid> {
        self.drop_target_context_all();
        &mut self.global_lists
    }

    pub fn get_mut_this_sprite_vars(&mut self) -> &mut HashMap<String, Uid> {
        self.drop_target_context_all();
        &mut self.this_sprite_vars
    }

    pub fn get_mut_this_sprite_lists(&mut self) -> &mut HashMap<String, Uid> {
        self.drop_target_context_all();
        &mut self.this_sprite_lists
    }

    pub fn get_mut_all_broadcasts(&mut self) -> &mut HashMap<String, Uid> {
        self.drop_target_context_all();
        &mut self.all_broadcasts
    }

    pub fn new_from_sb(project: &Project) -> Self {
        let mut global_vars = HashMap::new();
        let mut global_lists = HashMap::new();
        let mut this_sprite_vars = HashMap::new();
        let mut this_sprite_lists = HashMap::new();
        let mut all_broadcasts = HashMap::new();

        for target in project.targets.iter() {
            match target {
                SpriteOrStage::Sprite(sprite) => {
                    for (uid, var) in &sprite.target.variables.0 {
                        this_sprite_vars.insert(var.name.clone(), Uid::new(uid.clone()));
                    }
                    for (uid, list) in &sprite.target.lists.0 {
                        this_sprite_lists.insert(list.name.clone(), Uid::new(uid.clone()));
                    }
                    for (uid, broadcast) in &sprite.target.broadcasts.0 {
                        all_broadcasts.insert(broadcast.name.clone(), Uid::new(uid.clone()));
                    }
                }
                SpriteOrStage::Stage(stage) => {
                    for (uid, var) in &stage.target.variables.0 {
                        global_vars.insert(var.name.clone(), Uid::new(uid.clone()));
                    }
                    for (uid, list) in &stage.target.lists.0 {
                        global_lists.insert(list.name.clone(), Uid::new(uid.clone()));
                    }
                    for (uid, broadcast) in &stage.target.broadcasts.0 {
                        all_broadcasts.insert(broadcast.name.clone(), Uid::new(uid.clone()));
                    }
                }
            }
        }

        Self::new(
            global_vars,
            global_lists,
            this_sprite_vars,
            this_sprite_lists,
            all_broadcasts,
        )
    }
}

impl std::fmt::Debug for TargetContextWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TargetContextWrapper")
            .field("global_vars", &self.global_vars)
            .field("global_lists", &self.global_lists)
            .field("this_sprite_vars", &self.this_sprite_vars)
            .field("this_sprite_lists", &self.this_sprite_lists)
            .field("all_broadcasts", &self.all_broadcasts)
            .finish()
    }
}
