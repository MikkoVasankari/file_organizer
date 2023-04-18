use std::{ fs, io };
use std::fs::DirBuilder;
use std::ffi::OsStr;
use std::path::{ Path, PathBuf };
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    /// The path to the file to read
    dir: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    println!("{:#?}", args);

    // Creating new Directory
    let mut path_txt = PathBuf::from(&args.dir);
    path_txt.push("texts");

    //println!("{:#?} 1", path_txt);
    DirBuilder::new().recursive(true).create(path_txt).unwrap();

    //let path = "texts";
    //DirBuilder::new().recursive(true).create(path).unwrap();

    // Going through current Directory
    let entries = fs
        ::read_dir(&args.dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for item in &entries {
        // Taking wanted file extension
        let os_str_png = OsStr::new("png");
        let os_str_jpg = OsStr::new("jpg");

        let os_str_txt = OsStr::new("txt");

        // Adding jou/ Path to the files current Path
        let location_path_pic = Path::new("pictures").join(item);


        let test_path_txt = PathBuf::from(&args.dir);

        let mut test_os_string = test_path_txt.clone().into_os_string();

        test_os_string.push("/texts/");
        
        //println!("{:#?}1", &test_os_string);
        //  println!("{:#?}2", item);
        // let location_path_txt = Path::new("texts").join(i);

        //if i.extension() == Some(os_str_png) {
        //   fs::rename(i, &location_path_pic)?;
        //}

        //if i.extension() == Some(os_str_jpg) {
        //    fs::rename(i, &location_path_pic)?;
        //}


        // Tää toimii nyt missä vaan kansiossa :D 

        if item.extension() == Some(os_str_txt) {

            //println!("{:#?}", item.file_name().unwrap());
            //println!("{:#?}", test_os_string);

            test_os_string.push(item.file_name().unwrap());
            fs::rename(item, test_os_string)?;
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