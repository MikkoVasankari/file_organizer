use clap::Parser;
use std::ffi::{OsStr, OsString};
use std::fs::DirBuilder;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Parser, Debug)]
struct Cli {
    dir: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let vec_os_string: Vec<&OsStr> = vec![
        OsStr::new("png"),
        OsStr::new("jpg"),
        OsStr::new("txt"),
        OsStr::new("mp4"),
        OsStr::new("mp3"),
        OsStr::new("webm"),
    ];

    let entries = fs::read_dir(&args.dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for entry in &entries {
        if let Some(ext) = entry.extension() {
            create_files_in_dir(
                args.dir.clone(),
                entry,
                ext,
                vec_os_string.len(),
                vec_os_string.clone(),
            )
        }
    }

    Ok(())
}

fn create_files_in_dir(
    path: PathBuf,
    entry: &PathBuf,
    ext: &OsStr,
    item_list_len: usize,
    file_exts: Vec<&OsStr>,
) {
    for i in 0..item_list_len {
        if Some(ext) == file_exts.get(i).copied() {
            let path_os_string = create_new_directory(path.clone(), ext);
            create_new_files(path_os_string, entry);
        }
    }
}

fn create_new_directory(path: PathBuf, ext: &OsStr) -> OsString {
    let mut pathi_os_string = path.clone().into_os_string();

    pathi_os_string.push("/");
    pathi_os_string.push(ext);
    pathi_os_string.push("/");

    DirBuilder::new()
        .recursive(true)
        .create(pathi_os_string.clone())
        .unwrap();

    return pathi_os_string;
}

fn create_new_files(mut path: OsString, item: &PathBuf) {
    path.push(item.file_name().unwrap());
    fs::rename(item, path.clone()).expect("creating a file failed");
    println!("File was organized to {:?}", path);
    path.clear();
}
