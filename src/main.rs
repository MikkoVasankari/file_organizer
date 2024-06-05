use std::{ fs, io };
use std::fs::DirBuilder;
use std::ffi::{ OsStr, OsString };
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    /// The path to the directory to read from cli
    dir: PathBuf,
}

fn main() -> io::Result<()> {
    
    let args = Cli::parse();
    
    let vec_os_string: Vec<&OsStr> = vec![
        OsStr::new("png"),
        OsStr::new("jpg"),
        OsStr::new("txt"),
    ];

    let entries = fs
            ::read_dir(&args.dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

    fn create_new_directory(path: PathBuf, ext: &OsStr) -> OsString {
        let mut pathi_os_string = path.clone().into_os_string();

        pathi_os_string.push("/");
        pathi_os_string.push(ext);
        pathi_os_string.push("/");

        DirBuilder::new().recursive(true).create(pathi_os_string.clone()).unwrap();

        return pathi_os_string;
    }

    fn create_new_files(mut path: OsString, item: &PathBuf) {
        path.push(item.file_name().unwrap());
        fs::rename(item, path.clone()).expect("creating filed failed");
        println!("File was organized to directory {:?}", path);
        path.clear();
    }

    if args.dir.is_dir() == true {
      
        for item in &entries {
            if let Some(x) = item.extension() {
                for i in 0..vec_os_string.len() {
                    if Some(x) == vec_os_string.get(i).copied() {
                        let path_os_string = create_new_directory(args.dir.clone(), x);
                        create_new_files(path_os_string, item);
                    }
                }
            }
        }

    } else {
        println!("Path you gave is not a directory");
    }

    Ok(())
}
