use std::path::Path;
use image::{GenericImageView, ImageError};
use std::fs;
use std::ffi::OsStr;
use std::env;
use image::io::Reader as ImageReader;

fn process_image(path_string: &str, file_stem: &str, ext: &str, out_directory: &str) -> std::result::Result<String, ImageError> {
    let mut img = ImageReader::open(path_string)?.decode()?;
    let new_path = format!("{}{}_crop.{}", out_directory, file_stem, ext);

    let (width, height) = img.dimensions();

    println!("Cropping {}", file_stem);
    let starting_x = width / 2 - 256;
    let starting_y = height / 2 - 256;

    img
    .crop(starting_x, starting_y, 512, 512)
    .save(new_path)?;
    return Ok("Okay".to_string());
}

fn get_extension_from_path(path_string: &str) -> Option<&str> {
    Path::new(path_string)
        .extension()
        .and_then(OsStr::to_str)
}

fn get_file_stem_from_path(path_string: &str) -> &str {
    match Path::new(path_string)
        .file_stem()
        .and_then(OsStr::to_str) {
            Some(file_stem) => return file_stem,
            None => return path_string
        }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    let in_directory = &args[1];
    let out_directory = &args[2];
    // "/Users/thomas/img_normalizer"
    let paths = fs::read_dir(in_directory).unwrap();

    for path in paths {
        let path_string = path.unwrap().path().display().to_string();
        let file_extension = get_extension_from_path(&path_string);
        let file_stem = get_file_stem_from_path(&path_string);
        // println!("Path: {}", path_string);
        // println!("File stem: {}", file_stem);
        match file_extension {
            // The division was valid
            Some(ext) => {
                println!("Extension: {}", ext);
                if ext == "jpeg" || ext == "jpg" {
                    match process_image(&path_string, file_stem, ext, out_directory) {
                        Ok(_) => println!("...Done"),
                        Err(err) => {
                            println!("There was an error: {}", err);
                        }
                    }
                }
            }
            None    => ()
        }
    }


    // match process_image() {
    //     Ok(_) => println!("Done"),
    //     Err(err) => {
    //         println!("There was an error: {}", err);
    //     }
    // }
    // let mut file = File::create("/Users/thomas/img_normalizer/test.txt")?;
    // let buffer = "Hello Thomas!\n";
    // file.write_all(buffer.as_bytes())?;
    // println!("Finish writing...");

    // fs::rename("/Users/thomas/img_normalizer/test.txt", "/Users/thomas/img_normalizer/hello_thomas.txt")?;
    // fs::remove_file("/tmp/LJ.txt")?;
    // println!("Finish deleting...");

}