#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
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
    fn to_file_node(&mut self, path: &Path) -> Result<&mut FileNode>;
}

struct VecFilePool {
    file_nodes: Vec<FileNode>
}

fn canonicalize(path: &Path) -> Result<String> {
    Ok(
        path.canonicalize()?
            .to_str().expect("invalid UTF-8")
            .to_string()
    )
}

impl FilePool for VecFilePool {
    fn to_file_node(&mut self, path: &Path) -> Result<&mut FileNode> {
        let path_str = canonicalize(path)?;

        if let Some(i) = (0..self.file_nodes.len()).find(|&i| self.file_nodes[i].path == path_str) {
            Ok(&mut self.file_nodes[i])
        } else {
            self.file_nodes.push(FileNode { path: path_str, text: String::new() });
            Ok(self.file_nodes.last_mut().unwrap())
        }
    }
}

lazy_static! {
    static ref FILES: Mutex<VecFilePool> = Mutex::new(VecFilePool { file_nodes: vec![] });
}

fn update_file_node<F>(path: &Path, f: F) -> Result<usize> 
    where F: FnOnce(&mut FileNode) -> Result<usize> {
    let mut pool = FILES
        .lock()
        .unwrap();

    let file_node = pool.to_file_node(path)?;
    f(file_node)
}

pub fn parse_todo_file<'a>(path: &Path) -> Result<hullatodo_txt::TodoLines<'a>> {

    let mut file = File::open(path)?;
    let file_len = file.metadata()?.len();
    // NOTE: the + 1 is to avoid reallocation when the string reaches capacity
    let mut buffer = String::with_capacity(file_len as usize + 1);
    file.read_to_string(&mut buffer)?;

    update_file_node(path, |file_node: &mut FileNode|{
        file_node.text = buffer;
        Ok(0)
    })?;

    // let slice = file_node.text.as_str();
    // Ok(parse_todo_str(slice))
    Ok(vec![])
}

pub fn parse_todo_str<'a>(text: &'a str) -> hullatodo_txt::TodoLines {
    hullatodo_txt::parse(text)
}

