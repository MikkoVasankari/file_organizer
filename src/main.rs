use std::{ fs, io };
use std::fs::DirBuilder;
use std::ffi::OsStr;
use std::path::{ PathBuf };
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    /// The path to the directory to read from cli
    dir: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    // Creating new Directories
    let mut path_txt = PathBuf::from(&args.dir);
    path_txt.push("texts");
    DirBuilder::new().recursive(true).create(path_txt).unwrap();

    let mut path_pics: PathBuf = PathBuf::from(&args.dir);
    path_pics.push("pictures");
    DirBuilder::new().recursive(true).create(path_pics).unwrap();

    // Going through given Directory
    let entries = fs
        ::read_dir(&args.dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let vec_os_string: Vec<&OsStr> = vec![
        OsStr::new("png"),
        OsStr::new("jpg"),
        OsStr::new("txt")
    ];

    for item in &entries {

        // Adding created directories to the path

        let path_txt = PathBuf::from(&args.dir);
        let mut path_txt_os_string = path_txt.clone().into_os_string();
        path_txt_os_string.push("/texts/");

        let path_pics: PathBuf = PathBuf::from(&args.dir);
        let mut path_pics_os_string = path_pics.clone().into_os_string();
        path_pics_os_string.push("/pictures/");


        // Checking file extension and adding file to correct directory
        if item.extension() == vec_os_string.get(0).copied() || item.extension() == vec_os_string.get(1).copied() {
            path_pics_os_string.push(item.file_name().unwrap());
            fs::rename(item, path_pics_os_string)?;
        }

        if item.extension() == vec_os_string.get(2).copied() {
            path_txt_os_string.push(item.file_name().unwrap());
            fs::rename(item, path_txt_os_string)?;
        }

        // figure out match in rust

        //match item.extension() {
        

        //Some(_) if item.extension() == vec_os_string.get(0).copied() => {
        //    println!("jou se o filu")
        //},

        //_ => println!("this is a directory"),

        //}
    }

    Ok(())
}