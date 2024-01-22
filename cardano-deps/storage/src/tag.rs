use cardano::util::{hex, try_from_slice::TryFromSlice};
use std::io::{Read, Write};
use std::{fs, path::PathBuf};

use cardano::block;

pub const OLDEST_BLOCK: &str = "OLDEST_BLOCK";
pub const HEAD: &str = "HEAD";

pub fn get_epoch_tag(epoch: block::EpochId) -> String {
    format!("EPOCH_{}", epoch)
}

pub fn write<S: AsRef<str>>(storage: &super::Storage, name: &S, content: &[u8]) {
    let mut tmp_file = super::tmpfile_create_type(storage, super::StorageFileType::Tag);
    tmp_file.write_all(hex::encode(content).as_bytes()).unwrap();

    let path = storage.config.get_tag_filepath(name);
    let dir = PathBuf::from(path);

    match dir.parent() {
        None => {}
        Some(parent) => {
            if parent != storage.config.get_filetype_dir(super::StorageFileType::Tag) {
                fs::create_dir_all(parent).unwrap()
            }
        }
    };

    tmp_file
        .render_permanent(&storage.config.get_tag_filepath(name))
        .unwrap();
}

pub fn write_hash<S: AsRef<str>>(storage: &super::Storage, name: &S, content: &block::HeaderHash) {
    write(storage, name, content.as_ref())
}

pub fn read<S: AsRef<str>>(storage: &super::Storage, name: &S) -> Option<Vec<u8>> {
    if !exist(storage, name) {
        return None;
    }
    let mut content = Vec::new();
    let path = storage.config.get_tag_filepath(name);
    let mut file = fs::File::open(path).unwrap();
    file.read_to_end(&mut content).unwrap();
    String::from_utf8(content.clone())
        .ok()
        .and_then(|r| hex::decode(&r).ok())
        .or(Some(content))
}

pub fn read_hash<S: AsRef<str>>(storage: &super::Storage, name: &S) -> Option<block::HeaderHash> {
    read(storage, name).and_then(|v| block::HeaderHash::try_from_slice(&v[..]).ok())
}

pub fn exist<S: AsRef<str>>(storage: &super::Storage, name: &S) -> bool {
    let p = storage.config.get_tag_filepath(name);
    p.as_path().exists()
}

pub fn remove_tag<S: AsRef<str>>(storage: &super::Storage, name: &S) {
    let p = storage.config.get_tag_filepath(name);
    fs::remove_file(p).unwrap()
}
