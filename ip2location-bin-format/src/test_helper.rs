use std::{
    fs::{self, DirEntry},
    io::Error as IoError,
    path::{Path, PathBuf},
};

// https://doc.rust-lang.org/std/fs/fn.read_dir.html
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> Result<(), IoError> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

pub(crate) fn ip2location_bin_files(cb: &dyn Fn(&PathBuf)) -> Result<(), IoError> {
    visit_dirs(Path::new("data"), &|entry| {
        let path = entry.path();
        if path
            .as_os_str()
            .to_str()
            .map(|x| x.contains("/IP2LOCATION") && x.ends_with(".BIN"))
            == Some(true)
        {
            cb(&path)
        }
    })
}

pub(crate) fn ip2proxy_bin_files(cb: &dyn Fn(&PathBuf)) -> Result<(), IoError> {
    visit_dirs(Path::new("data"), &|entry| {
        let path = entry.path();
        if path
            .as_os_str()
            .to_str()
            .map(|x| x.contains("/IP2PROXY") && x.ends_with(".BIN"))
            == Some(true)
        {
            cb(&path)
        }
    })
}
