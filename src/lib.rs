#![deny(clippy::pedantic)]
// #![warn(missing_docs)]

use set_error::ChangeError;

use std::{
    path::Path,
    rc::Rc,
    thread,
    time::{Duration, SystemTime},
};

pub struct FileListBuilder<T> {
    files: Vec<WatchedFile<T>>,
    interval: Duration,
    max_retries: Option<u32>,
    open_file_func: Rc<Fn(&str) -> WatchingFuncResult<T>>,
}

pub struct WatchedFile<T> {
    path: String,
    time: SystemTime,
    functions_on_run: Vec<Rc<Fn(T) -> WatchingFuncResult<T>>>,
}

pub enum WatchingFuncResult<T> {
    Success(T),
    Retry,
    Fail(String),
}
use WatchingFuncResult::*;

impl<T> FileListBuilder<T> {
    fn new<F: 'static + Fn(&str) -> WatchingFuncResult<T>>(open_func: F) -> Self {
        Self {
            files: Vec::new(),
            interval: Duration::from_millis(1000),
            max_retries: None,
            open_file_func: Rc::new(open_func),
        }
    }
    fn add_file(mut self, file: WatchedFile<T>) -> Self {
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
    fn launch(self) -> Result<(), String> {
        let on_first_run = true;
        for file in self.files {
            let mut retries = self.max_retries;
            let file_data = loop {
                match (self.open_file_func)(&file.path) {
                    Success(t) => break t,
                    Fail(s) => return Err(s),
                    Retry => {
                        retries = retries.map(|x| x - 1);
                        match retries {
                            Some(n) if n == 0 => return Err(String::from("no more retries")),
                            _ => {
                                thread::sleep(self.interval);
                                continue;
                            }
                        }
                    }
                }
            };
        }
        Ok(())
    }
}

impl<T> WatchedFile<T> {
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
    fn add_func<F: 'static + Fn(T) -> WatchingFuncResult<T>>(mut self, func: F) -> Self {
        self.functions_on_run.push(Rc::new(func));
        self
    }
}
