use std::env::temp_dir;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use uuid::Uuid;

use tempdir::TempDir;

/// Creates a directory structure suitable for storing large numbers of files.
/// Optionally deletes the created directory and files when dropped.
///
/// Slots for new files are allocated using `get_new_file()`. This struct will
/// create new subdirectories as needed to ensure that no subdirectory contains
/// more than 1,000 files/subdirectories.
pub struct FileTree {
    tmp_dir: Option<TempDir>,
    persistent_dir: Option<PathBuf>,
    counter: u64,
}

impl FileTree {
    /// Create a new directory structure under `path`. If `persistent` is
    /// `false` the directory and all it's contents will be deleted when
    /// the returned `FileTree` is dropped
    ///
    /// # Examples
    ///
    /// Create a new temporary data structure and make sure the base path exists
    ///
    /// ```
    /// use file_tree::FileTree;
    /// use std::env::temp_dir;
    ///
    /// let file_tree = FileTree::new_in(temp_dir(), false).unwrap();
    /// assert!(file_tree.get_root().exists());
    /// ```
    ///
    /// # Errors
    ///
    /// If `persistent` is `false`, the directory will be created using
    /// `tempdir::TempDir`, and any related errors will be returned here
    pub fn new_in(path: PathBuf, persistent: bool) -> Result<FileTree> {
        if persistent {
            Ok(FileTree {
                tmp_dir: None,
                persistent_dir: Some(path),
                counter: 0,
            })
        } else {
            Ok(FileTree {
                tmp_dir: Some(TempDir::new_in(path, "file_tree")?),
                persistent_dir: None,
                counter: 0,
            })
        }
    }

    /// Create a new directory structure. If `persistent` is `false` the
    /// directory and all it's contents will be deleted when the returned
    /// `FileTree` is dropped.    
    /// 
    /// # Examples
    /// 
    /// Create a new temporary data structure and make sure the base path exists
    /// 
    /// ```
    /// use file_tree::FileTree;
    ///
    /// let file_tree = FileTree::new(false).unwrap();
    /// assert!(file_tree.get_root().exists());
    /// ```
    ///
    /// # Errors
    ///
    /// If `persistent` is `false`, the directory will be created using
    /// `tempdir::TempDir`, and any related errors will be returned here
    pub fn new(persistent: bool) -> Result<FileTree> {
        if persistent {
            let uuid = Uuid::new_v4().hyphenated().to_string();

            Ok(FileTree {
                tmp_dir: None,
                persistent_dir: Some(temp_dir().join(uuid)),
                counter: 0,
            })
        } else {
            Ok(FileTree {
                tmp_dir: Some(TempDir::new("file_tree")?),
                persistent_dir: None,
                counter: 0,
            })
        }
    }

    /// Creates a `FileTree` from an existing directory structure. `path` should
    /// be equivalent to the result of calling `get_root()` on the previous
    /// (persistent) `FileTree`.
    /// 
    /// # Examples
    /// 
    /// Re-create a `FileTree` using an existing file structure
    /// 
    /// ```
    /// use file_tree::FileTree;
    /// use std::fs::File;
    /// 
    /// // create a `FileTree` with one file
    /// let mut ft = FileTree::new(true).unwrap();
    /// let file_path = ft.get_new_file().unwrap();
    /// File::create(file_path.clone()).unwrap();
    /// let base = ft.get_root();
    /// drop(ft);
    /// 
    /// // create a `FileTree` using the existing path, and make sure that the
    /// // files we pull back don't overwrite the existing one
    /// let mut ft2 = FileTree::from_existing(base);
    /// let file2 = ft2.get_new_file().unwrap();
    /// assert_eq!(file_path.file_name().unwrap(), "000000000000");
    /// assert_eq!(file2.file_name().unwrap(), "000000000001");
    /// ```
    pub fn from_existing(path: PathBuf) -> FileTree {
        FileTree {
            tmp_dir: None,
            persistent_dir: Some(path),
            counter: 0,
        }
    }

    /// Returns a PathBuf pointing to an available slot in the file tree. The
    /// file pointed to by the returned `PathBuf` will not be created by
    /// this method call, but a new directory will be created if necessary.
    ///
    /// This method will ensure that the file pointed to by the returned
    /// `PathBuf` does not exist. If this struct was created using an existing
    /// directory structure existing files will be skipped over when generating
    /// new file names to return.
    ///
    /// File paths are generated such that each new leaf directory (starting
    /// with `000/000/000/`) will be filled entirely before creating a new
    /// directory (next would be `000/000/001/`).
    /// 
    /// 
    /// # Examples
    /// 
    /// Retrieve two distinct file paths via `get_new_file()`
    /// 
    /// ```
    /// use file_tree::FileTree;
    /// 
    /// let mut file_tree = FileTree::new(false).unwrap();
    /// 
    /// let writeable_path = file_tree.get_new_file().unwrap();
    /// assert_eq!(
    ///     writeable_path,
    ///     file_tree.get_root().join("000/000/000/000000000000")
    /// );
    /// 
    /// let writeable_path_2 = file_tree.get_new_file().unwrap();
    /// assert_eq!(
    ///     writeable_path_2,
    ///     file_tree.get_root().join("000/000/000/000000000001")
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// If a new subdirectory is required, `fs::create_dir_all` will be called.
    /// Any errors from that call will be returned here
    pub fn get_new_file(&mut self) -> Result<PathBuf> {
        let mut new_file = self.get_new_file_uniq()?;
        while new_file.exists() {
            new_file = self.get_new_file_uniq()?;
        }
        Ok(new_file)
    }

    fn get_new_file_uniq(&mut self) -> Result<PathBuf> {
        let uid = format!("{:012}", self.counter);
        self.counter += 1;
        let mut buff = String::with_capacity(3);
        let mut parts = Vec::with_capacity(4);
        for c in uid.chars() {
            if buff.chars().count() >= 3 {
                parts.push(buff);
                buff = String::with_capacity(3);
            }
            buff.push(c);
        }
        if buff.chars().count() > 0 {
            parts.push(buff);
        }
        let path_str = format!("{0}/{1}/{2}", parts[0], parts[1], parts[2]);
        let path = self.get_root().join(path_str);
        match fs::create_dir_all(&path) {
            Ok(_) => Ok(path.join(uid)),
            Err(e) => Err(e),
        }
    }

    /// Return the root path for the file tree
    pub fn get_root(&self) -> PathBuf {
        match self.tmp_dir {
            Some(ref p) => p.path().to_path_buf(),
            None => self.persistent_dir.as_ref().unwrap().to_path_buf(),
        }
    }
}
