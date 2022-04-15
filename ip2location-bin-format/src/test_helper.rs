use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub(crate) fn ip2location_bin_files() -> Vec<PathBuf> {
    WalkDir::new(Path::new("data"))
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .as_os_str()
                .to_str()
                .map(|x| x.contains("/ip2location-") && x.ends_with(".BIN"))
                == Some(true)
        })
        .map(|e| e.path().into())
        .collect()
}

pub(crate) fn ip2proxy_bin_files() -> Vec<PathBuf> {
    WalkDir::new(Path::new("data"))
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .as_os_str()
                .to_str()
                .map(|x| x.contains("/ip2proxy-") && x.ends_with(".BIN"))
                == Some(true)
        })
        .map(|e| e.path().into())
        .collect()
}
