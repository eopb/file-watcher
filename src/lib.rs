#![deny(clippy::pedantic)]
// #![warn(missing_docs)]

use std::{
    path::Path,
    rc::Rc,
    time::{Duration, SystemTime},
};

struct FileListBuilder {
    files: Vec<WatchedFile>,
    interval: Duration,
    max_retries: Option<u32>,
}

struct WatchedFile {
    path: String,
    time: SystemTime,
    functions_on_run: Vec<Rc<Fn(String) -> WatchingFuncResult>>,
}

enum WatchingFuncResult {
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
    fn launch(self) -> () {}
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
}

impl WatchedFile {
    fn new(path: String) -> Result<Self, String> {
        Ok(Self {
            path: path,
            time: Path::new(&path)
                .metadata()
                .set_error(&format!("failed to open file {} metadata", path.display()))?
                .modified()
                .set_error(&format!(
                    "failed to find files date modifide {}",
                    path.display()
                )),
            functions_on_run: Vec::new(),
        })
    }
}
