use std::path::PathBuf;

pub struct BookConfig {
    dest: PathBuf,
    src: PathBuf,
    multilingual: bool,
}


impl BookConfig {
    pub fn new() -> Self {
        BookConfig {
            dest: PathBuf::from("book"),
            src: PathBuf::from("src"),
            multilingual: false,
        }
    }

    pub fn dest(&self) -> PathBuf {
        self.dest.clone()
    }

    pub fn set_dest(mut self, dest: PathBuf) -> Self {
        self.dest = dest;
        self
    }

    pub fn src(&self) -> PathBuf {
        self.src.clone()
    }

    pub fn set_src(mut self, src: PathBuf) -> Self {
        self.src = src;
        self
    }
}