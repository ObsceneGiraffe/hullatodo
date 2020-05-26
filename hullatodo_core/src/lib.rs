#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate hullatodo_txt;
#[macro_use]
extern crate lazy_static;

use std::{
    fs::File,
    io::Result,
    io::prelude::*,
    path::Path,
    str,
    sync::Mutex
};

struct FileNode {
    path: String,
    text: String
}

trait FilePool {
    fn to_file_node<P: AsRef<Path>>(&mut self, path: P) -> Option<&mut FileNode>;
}

struct VecFilePool {
    files: Vec<FileNode>
}

impl FilePool for VecFilePool {
    fn to_file_node<P: AsRef<Path>>(&mut self, path: P) -> Option<&mut FileNode> {
        let path_str = path.as_ref().to_str()?.to_string();

        if let Some(i) = (0..self.files.len()).find(|&i| self.files[i].path == path_str) {
            Some(&mut self.files[i])
        } else {
            let node = FileNode { path: path_str, text: String::new() };
            self.files.push(node);
            Some(self.files.last_mut().unwrap())
        }
    }
}

lazy_static! {
    static ref FILES: Mutex<VecFilePool> = Mutex::new(VecFilePool { files: vec![] });
}

pub fn parse_todo_file<'a, P: AsRef<Path>>(path: P) -> Result<hullatodo_txt::TodoLines<'a>> {

    let mut file = File::open(path.as_ref())?;
    let file_len = file.metadata()?.len();

    let file_node = FILES.lock().unwrap().to_file_node(path.as_ref()).unwrap();

    let mut bytes = Vec::with_capacity(file_len as usize + 1);
    file.read_to_end(&mut bytes)?;
    file_node.text = String::from_utf8(bytes).unwrap();

    Ok(parse_todo_str(file_node.text.as_str()))
}

pub fn parse_todo_str(text: &'_ str) -> hullatodo_txt::TodoLines<'_> {
    hullatodo_txt::parse(text)
}

