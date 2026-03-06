use std::fs;
use std::io;
use std::path::Path;

pub fn check_file_exists(path: &Path) -> Result<(), io::Error> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        ));
    }

    if !path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is not a file",
        ));
    }

    Ok(())
}

pub fn write_to_objects(hash_hex: String, contents: Vec<u8>) -> Result<(), io::Error> {
    // Split the hash into two parts
    // The first part is the directory name
    // The second part is the file name
    let (dir, file) = hash_hex.split_at(2);

    // Create or check to see if the object folder already exists
    // If it does not exist, create it.
    if let Err(e) = fs::create_dir(".gmv/objects") {
        if e.kind() != io::ErrorKind::AlreadyExists {
            return Err(e);
        }
    }

    //create the directory
    fs::create_dir(format!(".gmv/objects/{}", dir))?;

    //create the file
    fs::write(format!(".gmv/objects/{}/{}", dir, file), contents)?;
    Ok(())
}

pub fn file_con_parse(file: String) -> io::Result<Vec<String>> {
    // This function is to read files, and parse them into a vector for processing
    // convert the file path from string to path using Path::path
    let path = Path::new(&file);

    //We read the contents from the file
    let file_con = fs::read_to_string(path)?;

    let lines: Vec<String> = file_con.lines().map(String::from).collect();

    Ok(lines)
}
