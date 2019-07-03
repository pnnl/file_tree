use file_tree::FileTree;

fn main() {
    let mut file_tree = FileTree::new(false).unwrap();

    let writeable_path = file_tree.get_new_file().unwrap();
    assert_eq!(
        writeable_path,
        file_tree.get_root().join("000/000/000/000000000000")
    );

    let writeable_path_2 = file_tree.get_new_file().unwrap();
    assert_eq!(
        writeable_path_2,
        file_tree.get_root().join("000/000/000/000000000001")
    );
}