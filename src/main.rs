use std::{ fs, io };
use std::fs::DirBuilder;
use std::ffi::OsStr;
use std::path::{ Path };

fn main() -> io::Result<()> {
    // Creating new Directory
    let path = "pictures";
    DirBuilder::new().recursive(true).create(path).unwrap();

    let path = "texts";
    DirBuilder::new().recursive(true).create(path).unwrap();

    // Going through current Directory
    let entries = fs
        ::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for i in &entries {
        // Taking wanted file extension
        let os_str_png = OsStr::new("png");

        let os_str_txt = OsStr::new("txt");

        // Adding jou/ Path to the files current Path
        let location_path_pic = Path::new("pictures/").join(i);
        let location_path_txt = Path::new("texts/").join(i);

        if i.extension() == Some(os_str_png) {
            fs::rename(i, location_path_pic)?;
        }
        
        if i.extension() == Some(os_str_txt) {
            fs::rename(i, location_path_txt)?;
        }

    }

    Ok(())
}