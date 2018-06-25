extern crate file_tree;

use file_tree::KeyedFileTree;

fn main() {
    let mut file_tree = KeyedFileTree::new(false).unwrap();

    let writeable_path_1 = file_tree.get(String::from("key1")).unwrap();
    let writeable_path_2 = file_tree.get(String::from("key2")).unwrap();

    assert_ne!(writeable_path_1, writeable_path_2);
}