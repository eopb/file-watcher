#![deny(clippy::pedantic)]
// #![warn(missing_docs)]

use std::{
    rc::Rc,
    time::{Duration, SystemTime},
};

struct FileListBuilder {
    files: Vec<WatchedFile>,
    interval: Duration,
}

struct WatchedFile {
    path: String,
    time: SystemTime,
    function_on_run: Rc<Fn() -> ()>,
}

impl FileListBuilder {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            interval: Duration::from_millis(1000),
        }
    }
    fn launch(self) -> () {}
}
