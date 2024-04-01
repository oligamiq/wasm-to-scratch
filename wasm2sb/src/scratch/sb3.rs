use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc},
};

use async_zip::ZipEntryBuilder;
use futures::{io::Cursor, AsyncReadExt};
use futures_lite as futures;
use log::warn;
use parking_lot::{Mutex, RwLock};
use sb_itchy::{
    build_context::{CustomFuncTy, GlobalVarListContext, TargetContext},
    target::SpriteBuilder,
    uid::Uid,
};
use sb_sbity::{comment::Comment, project::Project, target::SpriteOrStage};

use crate::util::get_preview_rect_from_block;

use super::rewrite_dependency::rewrite_list;

use eyre::{eyre, Context, Result};

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

        let project_data_zip =
            std::fs::File::open(&path).wrap_err(format!("failed to open file: {:?}", path))?;

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
        })
        .wrap_err("failed to read zip file")?;

        let index = match archive.file().entries().iter().position(|entry| {
            match entry.filename().as_str() {
                Ok("project.json") => true,
                _ => false,
            }
        }) {
            Some(index) => index,
            None => return Err(eyre!("failed to find project.json in zip file")),
        };

        let mut reader =
            futures::future::block_on(async { archive.reader_without_entry(index).await })
                .wrap_err("failed to read project.json from zip file")?;

        let mut buffer = Vec::new();
        futures::future::block_on(async { reader.read_to_end(&mut buffer).await })
            .wrap_err("failed to read to end project.json from zip file")?;

        let json = std::str::from_utf8(&buffer).wrap_err("failed to parse to utf8 project.json")?;

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
            warn!("failed to find sprite in project, creating new sprite");

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
                return Err(eyre!("failed to create sprite"));
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

    pub fn get_target_context(&self) -> TargetContextGuard {
        self.target_context.get_target_context()
    }

    pub fn target_context_mut(&mut self) -> &mut TargetContextWrapper {
        &mut self.target_context
    }

    pub fn zip(&self) -> Result<Vec<u8>> {
        let json = serde_json::to_string(&*self.project.read()).wrap_err(
            "failed to serialize project to json, this is a bug, please report it to the developers",
        )?;

        let cursor = Cursor::new(self.buff.clone());

        let mut archive = futures::future::block_on(async {
            async_zip::base::read::seek::ZipFileReader::new(cursor).await
        })
        .wrap_err("failed to read broken zip file")?;

        let mut buff = Vec::new();
        let mut writer = async_zip::base::write::ZipFileWriter::new(&mut buff);

        for i in 0..archive.file().entries().len() {
            let filename = archive.file().entries().get(i).unwrap().filename().as_str().wrap_err(
                "failed to get filename from zip file, this is a bug, please report it to the developers",
            )?.to_string();

            let buffer = if filename == "project.json" {
                json.as_bytes().to_vec()
            } else {
                let mut reader =
                    futures::future::block_on(async { archive.reader_without_entry(i).await })
                        .wrap_err("failed to read entry from zip file")?;

                let mut buffer = Vec::new();
                futures::future::block_on(async { reader.read_to_end(&mut buffer).await })
                    .wrap_err("failed to read to end entry from zip file")?;
                buffer
            };

            let entry = archive.file().entries().get(i).unwrap();

            let builder = ZipEntryBuilder::new(entry.filename().to_owned(), entry.compression());
            futures::future::block_on(async {
                writer.write_entry_whole(builder, buffer.as_slice()).await
            })
            .wrap_err("failed to write entry to zip file")?;
        }

        futures::future::block_on(async { writer.close().await })
            .wrap_err("failed to close zip file")?;

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
                return Err(eyre!(
                    "ancestor directory does not exist: {:?}",
                    out.parent().unwrap().parent()
                ));
            }
            if out.parent().is_some() && !out.parent().unwrap().exists() {
                std::fs::create_dir_all(out.parent().unwrap()).wrap_err(format!(
                    "failed to create directory: {:?}",
                    out.parent().unwrap()
                ))?;
            }
        }

        if out.exists() {
            warn!("file already exists, removing: {:?}", out);
            std::fs::remove_file(&out).wrap_err(format!("failed to remove file: {:?}", out))?;
        }

        let zipped = self.zip()?;

        let mut out =
            std::fs::File::create(out).wrap_err(format!("failed to create file: {:?}", out))?;

        out.write_all(&zipped)
            .wrap_err(format!("failed to write to file: {:?}", out))?;

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
    atomic_counter: Arc<AtomicUsize>,
    custom_funcs: Arc<Mutex<HashMap<String, CustomFuncTy>>>,
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
            atomic_counter: Arc::new(AtomicUsize::new(0)),
            custom_funcs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_target_context(&self) -> TargetContextGuard {
        let target_context: &'static mut TargetContext<'static> =
            Box::leak(Box::new(TargetContext {
                global_vars: Box::leak(Box::new(self.global_vars.clone())),
                global_lists: Box::leak(Box::new(self.global_lists.clone())),
                this_sprite_vars: Box::leak(Box::new(self.this_sprite_vars.clone())),
                this_sprite_lists: Box::leak(Box::new(self.this_sprite_lists.clone())),
                all_broadcasts: Box::leak(Box::new(self.all_broadcasts.clone())),
                custom_funcs: self.custom_funcs.clone(),
            }));

        self.atomic_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        return TargetContextGuard {
            target_context,
            atomic_counter: self.atomic_counter.clone(),
        };
    }

    pub fn get_mut_global_vars(&mut self) -> &mut HashMap<String, Uid> {
        if self
            .atomic_counter
            .load(std::sync::atomic::Ordering::SeqCst)
            > 0
        {
            panic!("cannot get mutable reference to global_vars while target_context is in use");
        }
        &mut self.global_vars
    }

    pub fn get_mut_global_lists(&mut self) -> &mut HashMap<String, Uid> {
        if self
            .atomic_counter
            .load(std::sync::atomic::Ordering::SeqCst)
            > 0
        {
            panic!("cannot get mutable reference to global_lists while target_context is in use");
        }
        &mut self.global_lists
    }

    pub fn get_mut_this_sprite_vars(&mut self) -> &mut HashMap<String, Uid> {
        if self
            .atomic_counter
            .load(std::sync::atomic::Ordering::SeqCst)
            > 0
        {
            panic!(
                "cannot get mutable reference to this_sprite_vars while target_context is in use"
            );
        }
        &mut self.this_sprite_vars
    }

    pub fn get_mut_this_sprite_lists(&mut self) -> &mut HashMap<String, Uid> {
        if self
            .atomic_counter
            .load(std::sync::atomic::Ordering::SeqCst)
            > 0
        {
            panic!(
                "cannot get mutable reference to this_sprite_lists while target_context is in use"
            );
        }
        &mut self.this_sprite_lists
    }

    pub fn get_mut_all_broadcasts(&mut self) -> &mut HashMap<String, Uid> {
        if self
            .atomic_counter
            .load(std::sync::atomic::Ordering::SeqCst)
            > 0
        {
            panic!("cannot get mutable reference to all_broadcasts while target_context is in use");
        }
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

pub struct TargetContextGuard {
    target_context: &'static mut TargetContext<'static>,
    atomic_counter: Arc<AtomicUsize>,
}

impl std::ops::Deref for TargetContextGuard {
    type Target = TargetContext<'static>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.target_context
    }
}

impl TargetContextGuard {
    pub fn deref_mut(&mut self) -> &'static mut TargetContext {
        self.target_context
    }
}

impl std::ops::Drop for TargetContextGuard {
    fn drop(&mut self) {
        unsafe {
            std::mem::drop(Box::from_raw(self.target_context));
            self.atomic_counter
                .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
