#![warn(missing_docs)]

//! Creates a directory structure suitable for storing large numbers of files.
//! Optionally deletes the created directory and files when dropped.

mod file_tree;

pub use crate::file_tree::{FileTree, KeyedFileTree};
