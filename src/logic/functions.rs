use rs_sha256::{HasherContext, Sha256Hasher};
use std::fs::{self};
use std::hash::Hasher;
use std::io;
use std::path::Path;

use crate::logic::utils::check_file_exists;
use crate::logic::utils::write_to_objects;

pub fn init_gmv(path: &str) -> io::Result<()> {
    std::env::set_current_dir(path)?;

    // Check if the path exists
    // If it does not exist, create it.
    // If it does exist, return an error.
    // See logic/utils.rs for more implementation details
    check_file_exists(Path::new(path))?;

    // creates repo dir
    std::fs::create_dir(".gmv")?;
    // Stores all contents.
    std::fs::create_dir(".gmv/objects")?;
    // Branch pointers
    std::fs::create_dir(".gmv/refs")?;
    // points to the current Branch
    // Contents set to main on init.
    std::fs::write(".gmv/HEAD", "ref: refs/heads/main")?;
    // staging area
    std::fs::write(".gmv/index", "")?;
    // repository configuration
    std::fs::write(".gmv/config", "")?;
    Ok(())
}

pub fn con_hash_256(file: String) -> io::Result<(String, Vec<u8>)> {
    // Treat the string called file as the path to the file using path::Path;
    // Check if the file exists and is a file
    // Read the file contents
    // Calculate the hash of the file contents
    // Print the hash
    let path = Path::new(&file);

    let contents = fs::read(path)?;

    println!("File length: {}", contents.len());

    // Create the header
    // Convert to blob
    // Add null byte
    let header = format!("blob {}\0", contents.len());

    // Create the hash
    let mut sha256_hasher = Sha256Hasher::default();
    sha256_hasher.write(header.as_bytes());
    sha256_hasher.write(&contents);

    //let bytes_result hold the final hashed bytes
    let bytes_result = HasherContext::finish(&mut sha256_hasher);

    // Convert the bytes to a hex string
    // This is to prepare the content for storage in the .git/objects directory
    // and for display to the user
    let hash_hex = bytes_result
        .as_ref()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    println!("{}", hash_hex);
    Ok((hash_hex, contents))
}

pub fn add_gmv(path: String) -> io::Result<()> {
    let (hash_hex, contents) = con_hash_256(path.clone())?;
    write_to_objects(hash_hex.clone(), contents)?;

    let index_path = ".gmv/index";
    // Read existing index if it exists, otherwise start empty
    let index_contents = fs::read_to_string(index_path).unwrap_or_default();

    let mut entries: Vec<String> = Vec::new();
    let mut found = false;

    // Parse lines and update the hash if the path matches
    for line in index_contents.lines() {
        if let Some((p, _)) = line.split_once(" -> ") {
            if p == path {
                entries.push(format!("{} -> {}", path, hash_hex));
                found = true;
            } else {
                entries.push(line.to_string());
            }
        } else if !line.trim().is_empty() {
            // Keep invalid/empty lines as they are to avoid data loss
            entries.push(line.to_string());
        }
    }

    // If we didn't find the path, append it
    if !found {
        entries.push(format!("{} -> {}", path, hash_hex));
    }

    // Write back the updated index
    let new_index_content = entries.join("\n") + "\n";
    fs::write(index_path, new_index_content)?;

    println!("Added: {}", path);
    Ok(())
}
