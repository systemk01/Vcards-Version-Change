/// Export contacts as VCards from Outlook and import them in ICloud contacts
/// When exporting VCards from Microsoft Outlook as .vcf-files they are exported as: 
/// BEGIN:VCARD
/// VERSION:2.3
/// ICloud accepts only VERSION:3.0
/// The .vcf-files are Windows-1252 encoded. Rust accepts only utf-8 encoding
/// This software iterates over all .vcf-files in a given directory, decodes them to utf-8
/// changes the VERSION to 3.0 and converts the files back to Windows-1252

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};

fn get_files_in_directory(path: &str) -> io::Result<Vec<String>> {
    // Get a list of all entries in the folder
    let entries = fs::read_dir(path)?;

    // Extract the filenames from the directory entries and store them in a vector
    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(file_names)
}

fn main() -> io::Result<()> {
    //Directory path
    let dir_path = "./Kontakte/";

    match get_files_in_directory(dir_path) {
        Ok(file_names) => {
            for file_name in file_names {
                //println!("{}", file_name);
                let file_path = dir_path.to_owned() + &file_name;

                // Open the file and decode its contents from WINDOWS-1252 to UTF-8
                let file = fs::File::open(&file_path)?;
                let mut decoder = DecodeReaderBytesBuilder::new()
                    .encoding(Some(WINDOWS_1252)) // Specify the WINDOWS-1252 encoding
                    .build(file);

                // Read the file contents into a string
                let mut contents = String::new();
                decoder.read_to_string(&mut contents)?;

                // Perform string replacement
                let target = "VERSION:4.0";
                let replacement = "VERSION:3.0";
                contents = contents.replace(target, replacement);

                // Encode back to WINDOWS-1252 and write the modified contents to the file
                let mut output = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(file_path)?;

                let (encoded_content, _, _) = WINDOWS_1252.encode(&contents);
                output.write_all(&encoded_content)?;
            } //End : for file_name
        } //End : OK(file_names)

        Err(e) => println!("Error: {}", e), //match arm Error
    }

    Ok(()) // io::Result OK for main()
}
