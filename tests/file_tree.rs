use std::env::temp_dir;
use std::fs::File;
use std::path::PathBuf;

use file_tree::FileTree;

#[test]
fn basic() {
    let mut ft = FileTree::new(false).unwrap();
    let path = ft.get_new_file().unwrap();
    assert_eq!(path.file_name().unwrap(), "000000000000");
    assert_eq!(
        ft.get_new_file().unwrap().file_name().unwrap(),
        "000000000001"
    );

    let tail = PathBuf::from("000/000/000/000000000000");
    assert!(path.ends_with(tail));
}

#[test]
fn new_dir() {
    let mut ft = FileTree::new(false).unwrap();
    let mut p = PathBuf::new();
    for _ in 0..1001 {
        p = ft.get_new_file().unwrap();
    }
    assert!(p.ends_with(PathBuf::from("000/000/001/000000001000")));
}

#[test]
fn keep_persistent() {
    let mut ft = FileTree::new(true).unwrap();
    let file = ft.get_new_file().unwrap();
    drop(ft);
    assert!(file.parent().unwrap().exists());

    let mut ft2 = FileTree::new_in(temp_dir(), true).unwrap();
    let file2 = ft2.get_new_file().unwrap();
    drop(ft2);
    assert!(file2.parent().unwrap().exists());
}

#[test]
fn delete_nonpersistent() {
    let mut ft = FileTree::new(false).unwrap();
    let file = ft.get_new_file().unwrap();
    drop(ft);
    assert!(!file.parent().unwrap().exists());

    let mut ft2 = FileTree::new_in(temp_dir(), false).unwrap();
    let file2 = ft2.get_new_file().unwrap();
    drop(ft2);
    assert!(!file2.parent().unwrap().exists());
}

#[test]
fn from_existing() {
    let mut ft = FileTree::new(true).unwrap();
    let file_path = ft.get_new_file().unwrap();
    File::create(file_path.clone()).unwrap();
    let base = ft.get_root();
    drop(ft);

    let mut ft2 = FileTree::from_existing(base);
    let file2 = ft2.get_new_file().unwrap();
    assert_eq!(file_path.file_name().unwrap(), "000000000000");
    assert_eq!(file2.file_name().unwrap(), "000000000001");
}
