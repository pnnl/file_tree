use std::env::temp_dir;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

use tempdir::TempDir;

/// Creates a directory structure suitable for storing large numbers of files.
/// Optionally deletes the created directory and files when dropped.
///
/// Slots for new files are allocated using `get_new_file()`. This struct will
/// create new subdirectories as needed to ensure that no subdirectory contains
/// more than 10,000 files/subdirectories.
pub struct FileTree {
    tmp_dir: Option<TempDir>,
    persistent_dir: Option<PathBuf>,
    counter: u64
}

impl FileTree {
    /// Create a new directory structure under `path`. If `persistent` is
    /// `false` the directory and all it's contents will be deleted when
    /// the returned `FileTree` is dropped
    ///
    /// # Errors
    ///
    /// If `persistent` is `false`, the directory will be created using
    /// `tempdir::TempDir`, and any related errors will be returned here
    pub fn new_in(path: PathBuf, persistent: bool) -> Result<FileTree> {
        if persistent {
            Ok(FileTree { tmp_dir: None,
                          persistent_dir: Some(path),
                          counter: 0 })
        } else {
            Ok(FileTree { tmp_dir: Some(TempDir::new_in(path, "file_tree")?),
                          persistent_dir: None,
                          counter: 0 })
        }
    }

    /// Create a new directory structure. If `persistent` is `false` the
    /// directory and all it's contents will be deleted when the returned
    /// `FileTree` is dropped.    
    ///
    /// # Errors
    ///
    /// If `persistent` is `false`, the directory will be created using
    /// `tempdir::TempDir`, and any related errors will be returned here
    pub fn new(persistent: bool) -> Result<FileTree> {
        if persistent {
            Ok(FileTree { tmp_dir: None,
                          persistent_dir: Some(temp_dir()),
                          counter: 0 })
        } else {
            Ok(FileTree { tmp_dir: Some(TempDir::new("file_tree")?),
                          persistent_dir: None,
                          counter: 0 })
        }
    }

    /// Returns a PathBuf pointing to an available slot in the file tree. The
    /// file pointed to by the returned `PathBuf` will not be created by
    /// this method call.     
    ///
    /// # Errors
    ///
    /// If a new subdirectory is required, `fs::create_dir_all` will be called.
    /// Any errors from that call will be returned here
    pub fn get_new_file(&mut self) -> Result<PathBuf> {
        let uid = format!("{:012}", self.counter);
        self.counter += 1;
        let mut buff = String::new();
        let mut parts = Vec::new();
        for c in uid.chars() {
            if buff.chars().count() >= 3 {
                parts.push(buff);
                buff = String::new();
            }
            buff.push(c);
        }
        if buff.chars().count() > 0 {
            parts.push(buff);
        }
        let path_str = format!("{0}/{1}/{2}", parts[0], parts[1], parts[2]);
        let path = self.get_path().join(path_str);
        match fs::create_dir_all(&path) {
            Ok(_) => Ok(path.join(uid)),
            Err(e) => Err(e)
        }
    }

    /// Return the root path for the file tree
    pub fn get_path(&self) -> PathBuf {
        match self.tmp_dir {
            Some(ref p) => p.path().to_path_buf(),
            None => self.persistent_dir.as_ref().unwrap().to_path_buf()
        }
    }
}
