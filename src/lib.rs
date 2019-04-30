#![deny(clippy::pedantic)]
// #![warn(missing_docs)]

use set_error::ChangeError;

use std::{
    path::Path,
    rc::Rc,
    time::{Duration, SystemTime},
};

pub struct FileListBuilder {
    files: Vec<WatchedFile>,
    interval: Duration,
    max_retries: Option<u32>,
}

pub struct WatchedFile {
    path: String,
    time: SystemTime,
    functions_on_run: Vec<Rc<Fn(String) -> WatchingFuncResult>>,
}

pub enum WatchingFuncResult {
    Success,
    Retry,
}

impl FileListBuilder {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            interval: Duration::from_millis(1000),
            max_retries: None,
        }
    }
    fn add_file(mut self, file: WatchedFile) -> Self {
        self.files.push(file);
        self
    }
    fn with_interval(mut self, inter: Duration) -> Self {
        self.interval = inter;
        self
    }
    fn with_max_retries(mut self, re: u32) -> Self {
        self.max_retries = Some(re);
        self
    }
    fn launch(self) -> () {}
}

impl WatchedFile {
    fn new(path: String) -> Result<Self, String> {
        Ok(Self {
            path: path.clone(),
            time: Path::new(&path)
                .metadata()
                .set_error(&format!("failed to open file {} metadata", path))?
                .modified()
                .set_error(&format!("failed to find files date modified {}", path))?,
            functions_on_run: Vec::new(),
        })
    }
    fn add_func<F: 'static + Fn(String) -> WatchingFuncResult>(mut self, func: F) -> Self {
        self.functions_on_run.push(Rc::new(func));
        self
    }
}
