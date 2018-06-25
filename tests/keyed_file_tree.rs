extern crate file_tree;

use std::env::temp_dir;
use std::fs::File;

use file_tree::KeyedFileTree;

#[test]
fn basic() {
    let mut ft = KeyedFileTree::new(false).unwrap();
    let path = ft.get(String::from("key1")).unwrap();
    assert!(!path.exists());
}

#[test]
fn dup_key() {
    let mut ft = KeyedFileTree::new(false).unwrap();

    let mut path = None;
    for _ in 0..10 {
        let new_p = ft.get(String::from("key1")).unwrap();
        if path.is_none() {
            path = Some(new_p);
            continue;
        }
        assert_eq!(new_p, path.unwrap());
        path = Some(new_p);
    }
}

#[test]
fn new_key() {
    let mut ft = KeyedFileTree::new(false).unwrap();
    let path1 = ft.get(String::from("key1")).unwrap();
    let path2 = ft.get(String::from("key1")).unwrap();
    let path3 = ft.get(String::from("key2")).unwrap();
    assert_eq!(path1, path2);
    assert_ne!(path1, path3);
}

#[test]
fn keep_persistent() {
    let mut ft = KeyedFileTree::new(true).unwrap();
    let file = ft.get(String::from("key1")).unwrap();
    drop(ft);
    assert!(file.parent().unwrap().exists());

    let mut ft2 = KeyedFileTree::new_in(temp_dir(), true).unwrap();
    let file2 = ft2.get(String::from("key2")).unwrap();
    drop(ft2);
    assert!(file2.parent().unwrap().exists());
}

#[test]
fn delete_nonpersistent() {
    let mut ft = KeyedFileTree::new(false).unwrap();
    let file = ft.get(String::from("key1")).unwrap();
    drop(ft);
    assert!(!file.parent().unwrap().exists());

    let mut ft2 = KeyedFileTree::new_in(temp_dir(), false).unwrap();
    let file2 = ft2.get(String::from("key1")).unwrap();
    drop(ft2);
    assert!(!file2.parent().unwrap().exists());
}

#[test]
fn from_existing() {
    let mut ft = KeyedFileTree::new(true).unwrap();
    let file_path = ft.get(String::from("key1")).unwrap();
    File::create(file_path.clone()).unwrap();
    let base = ft.get_root();
    let map = ft.get_existing_files();

    let mut ft2 = KeyedFileTree::from_existing(base, map);
    let file2 = ft2.get(String::from("key1")).unwrap();
    let file3 = ft2.get(String::from("key2")).unwrap();
    assert_eq!(file_path, file2);
    assert_ne!(file2, file3);
}
