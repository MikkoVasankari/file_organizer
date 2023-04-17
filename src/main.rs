use std::{ fs, io };
use std::fs::DirBuilder;
use std::ffi::OsStr;
use std::path::{ Path , PathBuf};
use clap::Parser;


#[derive(Parser , Debug)]
struct Cli {
    /// The path to the file to read
    dir: PathBuf,
}


fn main() -> io::Result<()> {

    let args = Cli::parse();
    println!("{:#?}", args);

    // Creating new Directory
    let path = "pictures";
    DirBuilder::new().recursive(true).create(path).unwrap();

    let path = "texts";
    DirBuilder::new().recursive(true).create(path).unwrap();

    // Going through current Directory
    let entries = fs
        ::read_dir(args.dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for i in &entries {
        // Taking wanted file extension
        let os_str_png = OsStr::new("png");
        let os_str_jpg = OsStr::new("jpg");

        let os_str_txt = OsStr::new("txt");

        // Adding jou/ Path to the files current Path
        let location_path_pic = Path::new("pictures/").join(i);
        let location_path_txt = Path::new("texts/").join(i);

        if i.extension() == Some(os_str_png) {
           fs::rename(i, &location_path_pic)?;
        }


        if i.extension() == Some(os_str_jpg) {
            fs::rename(i, &location_path_pic)?;
        }
        

        if i.extension() == Some(os_str_txt) {
            fs::rename(i, &location_path_txt)?;
        }
        

        // figurre out match in rust
        //rintln!("{:#?}", i.extension());
        //match i.extension() {


            //None | Some(_) => todo!(),

            //Some(_) => println!("jou teksti!"),

            //None => println!("this is a directory"),
            
            //}

        }   

    Ok(())
}