#![deny(clippy::pedantic)]
// #![warn(missing_docs)]

use std::{
    iter::Iterator,
    path::Path,
    rc::Rc,
    thread,
    time::{self, SystemTime},
};

struct FileListBuilder {
    files: Vec<WatchedFile>,
}

struct WatchedFile {
    path: String,
    time: SystemTime,
    function_on_run: Rc<Fn() -> ()>,
}

impl FileListBuilder {
    fn launch(self) -> () {}
}
