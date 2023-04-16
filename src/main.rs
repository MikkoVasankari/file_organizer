use std::{fs, io};
use std::fs::DirBuilder;
use std::ffi::OsStr;
use std::path::{Path };

fn main() -> io::Result<()> {

    // Creating new Directory
    let path = "jou";
    DirBuilder::new()
        .recursive(true)
        .create(path).unwrap();

    // Going through current Directory
    let entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;


    for i in &entries {    
        // Taking wanted file extension 
        let os_str = OsStr::new("png");

        // Adding jou/ Path to the files current Path
        let location_path = Path::new("jou/").join(i);


        if i.extension() == Some(os_str) {

            fs::rename(i, location_path)?;

        }

    }

    Ok(())
}