// #![deny(clippy::pedantic)]
// #![warn(missing_docs)]

use set_error::ChangeError;

use std::{
    path::Path,
    rc::Rc,
    thread,
    time::{Duration, SystemTime},
};

pub struct FileListBuilder<T: Clone> {
    files: Vec<WatchedFile<T>>,
    interval: Duration,
    max_retries: Option<u32>,
    open_file_func: Rc<Fn(&str) -> WatchingFuncResult<T>>,
}

pub struct WatchedFile<T> {
    path: String,
    date_modified: SystemTime,
    functions_on_run: Vec<Rc<Fn(T) -> WatchingFuncResult<T>>>,
    function_on_end: Rc<Fn(T) -> Result<(), String>>,
}

pub enum WatchingFuncResult<T> {
    Success(T),
    Retry(String),
    Fail(String),
}
use WatchingFuncResult::*;

impl<T: Clone> FileListBuilder<T> {
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
        let mut on_first_run = self.files.len();
        for file in self.files {
            thread::sleep(self.interval);
            if on_first_run != 0 {
                on_first_run -= 1
            }
            if (on_first_run != 0) || (file.date_modified != date_modified(&file.path)?) {
                let mut file_data = {
                    let mut retries = self.max_retries;
                    loop {
                        match (self.open_file_func)(&file.path) {
                            Success(t) => break t,
                            Fail(s) => return Err(s),
                            Retry(s) => {
                                retries = retries.map(|x| x - 1);
                                match retries {
                                    Some(n) if n == 0 => {
                                        return Err(String::from("no more retries"))
                                    }
                                    _ => {
                                        println!("{}", s);
                                        thread::sleep(self.interval);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                };
                for function_to_run in file.functions_on_run {
                    file_data = {
                        let mut retries = self.max_retries;
                        loop {
                            match function_to_run(file_data.clone()) {
                                Success(t) => break t,
                                Fail(s) => return Err(s),
                                Retry(s) => {
                                    retries = retries.map(|x| x - 1);
                                    match retries {
                                        Some(n) if n == 0 => {
                                            return Err(String::from("no more retries"))
                                        }
                                        _ => {
                                            println!("{}", s);
                                            thread::sleep(self.interval);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                (file.function_on_end)(file_data)?
            }
        }
        Ok(())
    }
}

impl<T> WatchedFile<T> {
    fn new<G: 'static + Fn(T) -> Result<(), String>>(
        path: String,
        end_func: G,
    ) -> Result<Self, String> {
        Ok(Self {
            path: path.clone(),
            date_modified: date_modified(&path)?,
            functions_on_run: Vec::new(),
            function_on_end: Rc::new(end_func),
        })
    }
    fn add_func<F: 'static + Fn(T) -> WatchingFuncResult<T>>(mut self, func: F) -> Self {
        self.functions_on_run.push(Rc::new(func));
        self
    }
}

fn date_modified(path: &str) -> Result<SystemTime, String> {
    Ok(Path::new(path)
        .metadata()
        .set_error(&format!("failed to open file {} metadata", path))?
        .modified()
        .set_error(&format!("failed to find files date modified {}", path))?)
}
