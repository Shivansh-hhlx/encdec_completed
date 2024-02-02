pub mod encryption {
    use std::{
        fs::{self, OpenOptions, File},
        io::{self, Write},
        path::Path,
    };
    use rand::Rng;

    const DECRYPT_FOLDER_FILE: &str = ".decrypt";

//////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////CODE BELOW THIS LINE///////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn byte_shift(text: Vec<u8>, shift_by: i8, backwards: bool) -> Vec<u8> {
        /* 
        if !backwards {
            y = shift_by;
        }
        else {
            y = -1 * shift_by;
        }
        text.iter().for_each(|a| a.wrapping_add_signed(y););
        text
        */ 
        
        /*
        text.iter().for_each(|a| if !backwards { a.wrapping_add(shift_by);} else { a.wrapping_sub(shift_by);});
        text
        */

        let mut new_content: Vec<u8> = vec![];
        let y;
        if !backwards {
            y = shift_by;
        }
        else {
            y = -1 * shift_by;
        }
        for i in text {
            let x;
            x = i.wrapping_add_signed(y);
            new_content.push(x);
        }
        new_content
    }

    pub fn process_file(file_path: &Path, decrypting: bool) -> io::Result<()> {
        let contents = fs::read(file_path)?;
    
        let new_content = byte_shift(contents, 2, decrypting);
    
        let mut file = OpenOptions::new().write(true).open(file_path)?;
    
        file.write_all(&new_content)?;
        file.flush()?;  // Explicitly flush the changes to the file
        drop(file);     // Explicitly close the file
    
        Ok(())
    }

    pub fn process_directory(directory_path: &Path, decrypting: bool) -> io::Result<()> {
        for entry in fs::read_dir(directory_path)? {
            let entry = entry?;
            let path = entry.path();
    
            if path.is_file() {
                process_file(&path, decrypting)?;
            }
        }
    
        if !decrypting {
            create_decrypt_file(directory_path)?;
        } else {
            remove_decrypt_file(directory_path)?;
        }
    
        Ok(())
    }

    pub fn verify_code(decrypting: bool) -> bool {
        if decrypting {
            let mut rng = rand::thread_rng();
            let random_code: u32 = rng.gen_range(1000..10000); 
            println!("Enter the 4-digit code: ");
            println!("Key is {}", random_code);
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line");

            match user_input.trim().parse::<u32>() {
                Ok(code) if code == random_code => true,
                _ => false,
            }
        } else {
            true // No verification required for encryption
        }
    }
    
//////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////CODE ABOVE THIS LINE///////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn create_decrypt_file(directory_path: &Path) -> io::Result<()> {
        let mut decrypt_file = File::create(directory_path.join(DECRYPT_FOLDER_FILE))?;
        decrypt_file.write_all(b"")?;
        Ok(())
    }

    pub fn remove_decrypt_file(directory_path: &Path) -> io::Result<()> {
        let decrypt_file_path = directory_path.join(DECRYPT_FOLDER_FILE);
        fs::remove_file(decrypt_file_path)
    }

    pub fn is_encrypted(directory_path: &Path) -> bool {
        directory_path.join(DECRYPT_FOLDER_FILE).exists()
    }
}