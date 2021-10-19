use std::convert::TryFrom;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// defines a wrapper around a PathBuf that has been checked to
/// return some name when calling file_name() on it.
#[derive(Debug, Clone)]
pub struct FilePath(PathBuf);

impl TryFrom<PathBuf> for FilePath {
    type Error = ();

    fn try_from(p: PathBuf) -> Result<Self, Self::Error> {
        if p.file_name().is_none() {
            Err(())
        } else {
            Ok(FilePath(p))
        }
    }
}

impl AsRef<Path> for FilePath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl From<FilePath> for PathBuf {
    fn from(fp: FilePath) -> Self {
        fp.0
    }
}

impl FilePath {
    pub fn file_name(&self) -> &OsStr {
        self.0.file_name().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::ffi::OsStr;
    use std::path::PathBuf;

    use crate::file_path::FilePath;

    #[test]
    fn file_path_construction_works() {
        assert!(FilePath::try_from(PathBuf::from("hello.txt")).is_ok());
        assert!(FilePath::try_from(PathBuf::from("C:\\")).is_err());
        assert!(FilePath::try_from(PathBuf::from("C:\\tmp\\")).is_ok());
    }

    #[test]
    fn file_path_name_works() {
        assert_eq!(
            OsStr::new("hello.txt"),
            FilePath::try_from(PathBuf::from("C:\\tmp\\hello.txt"))
                .unwrap()
                .file_name()
        );
    }
}
