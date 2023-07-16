rust
use crate::error::CkError;
use sxd_document::Package;
use sxd_document::dom::Document;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use fs_err as fs;

#[derive(Debug)]
pub struct Cache {
    root: PathBuf,
    files: HashMap<PathBuf, String>,
    values: HashMap<PathBuf, Package>,
    last_path: Option<PathBuf>,
}

impl Cache {
    /// Create a new cache, used to read files only once and otherwise store their contents.
    pub fn new(doc_dir: &str) -> Cache {
        Cache {
            root: Path::new(doc_dir).to_owned(),
            files: HashMap::new(),
            values: HashMap::new(),
            last_path: None,
        }
    }

    fn resolve_path(&mut self, path: &str) -> PathBuf {
        if path != "-" {
            let resolve = self.root.join(path);
            self.last_path = Some(resolve.clone());
            resolve
        } else {
            self.last_path
                .as_ref()
                // FIXME: Point to a line number
                .expect("No last path set. Make sure to specify a full path before using `-`")
                .clone()
        }
    }

    fn read_file(&mut self, path: &PathBuf) -> Result<String, io::Error> {
        if let Some(f) = self.files.get(path) {
            return Ok(f.clone());
        }
        let file = fs::read_to_string(path)?;

        self.files.insert(path.clone(), file.clone());

        Ok(file)
    }

    /// Get the text from a file. If called multiple times, the file will only be read once
    pub fn get_file(&mut self, path: &str) -> Result<String, io::Error> {
        let path = self.resolve_path(path);
        self.read_file(&path)
    }

    /// Parse the DOM from a file. If called multiple times, the file will only be read once.
    pub fn get_value<T, F: Fn(Document<'_>) -> T>(&mut self, path: &str, f: F) -> Result<T, CkError> {
        let rpath = self.resolve_path(path);

        if let Some(v) = self.values.get(&rpath) {
            return Ok(f(v.as_document()));
        }

        let content = self.read_file(&rpath)?;
        let val = sxd_document::parser::parse(&content)?;

        self.values.insert(rpath, val);

        self.get_value(path, f)
    }
}
