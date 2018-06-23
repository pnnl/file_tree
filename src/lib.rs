#![warn(missing_docs)]

//! Creates a directory structure suitable for storing large numbers of files.
//! Optionally deletes the created directory and files when dropped.

extern crate tempdir;
extern crate uuid;

mod file_tree;

pub use file_tree::FileTree;
