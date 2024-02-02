use encdec::encryption;

use std::{
    env,
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Usage: {} <directory>", args[0]);
        println!("Example: {} /path/to/directory", args[0]);
        return;
    }

    let directory_path = Path::new(&args[1]);

    if directory_path.is_dir() {
        if encryption::is_encrypted(directory_path) {
            
            println!("Folder is already encrypted. Proceeding with decryption.");

            
            if !encryption::verify_code(true) {
                println!("Incorrect code. Exiting.");
                return;
            }

            if let Err(e) = encryption::process_directory(directory_path, true) {
                println!("Error processing directory (decryption): {:?}", e);
            }
        } else {
            

            if let Err(e) = encryption::process_directory(directory_path, false) {
                println!("Error processing directory (encryption): {:?}", e);
                return;
            }

            
            if !encryption::verify_code(true) {
                println!("Incorrect code. Exiting.");
                return;
            }

            if let Err(e) = encryption::process_directory(directory_path, true) {
                println!("Error processing directory (decryption): {:?}", e);
            }
        }
    } else {
        println!("The specified path is not a directory.");
    }

    println!("Successfully Done");
}
