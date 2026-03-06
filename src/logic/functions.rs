use rs_sha256::{HasherContext, Sha256Hasher};
use std::fs::{self};
use std::hash::Hasher;
use std::io;
use std::path::Path;

use crate::logic::utils::check_file_exists;
use crate::logic::utils::file_con_parse;
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

pub fn hash_memory(contents: &[u8], obj_type: &str) -> (String, Vec<u8>) {
    // Create the header
    // Format: "<type> <length>\0"
    let header = format!("{} {}\0", obj_type, contents.len());

    // Create the hash
    let mut sha256_hasher = Sha256Hasher::default();
    sha256_hasher.write(header.as_bytes());
    sha256_hasher.write(contents);

    //let bytes_result hold the final hashed bytes
    let bytes_result = HasherContext::finish(&mut sha256_hasher);

    // Convert the bytes to a hex string
    let hash_hex = bytes_result
        .as_ref()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    (hash_hex, contents.to_vec())
}

pub fn con_hash_256(file: String) -> io::Result<(String, Vec<u8>)> {
    // Treat the string called file as the path to the file using path::Path;
    // Check if the file exists and is a file
    // Read the file contents
    let path = Path::new(&file);
    let contents = fs::read(path)?;

    println!("File length: {}", contents.len());

    let (hash_hex, _) = hash_memory(&contents, "blob");

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

pub fn write_tree() -> io::Result<String> {
    // From adequate research,
    // Trees are a snapshot of the directory structure at a given point in time.
    // They are stored in the .git/objects directory and are identified by their hash.
    // The trees are immutable, meaning that once a tree is created, it cannot be changed.
    // And multiple trees are created to track the changes in the directory structure per commit.

    // Here goes nothing, we by:

    // Locate the file index
    let index_path = ".gmv/index".to_string();

    // Parse the contents of the file index with crate::logic::utils::file_con_parse;
    // file_con_parse returns a vector where each element is a line from the index file
    let parsed_content = file_con_parse(index_path)?;

    // This vector will store the formatted tree entries
    let mut fin_tree_con: Vec<String> = Vec::new();

    // Iterate through each line of the parsed index
    for item in parsed_content {
        // Each index entry currently looks like:
        // filename -> hash
        // Example:
        // avalanche.txt -> 37a01f02...

        // Split the line into two parts
        let parts: Vec<&str> = item.split(" -> ").collect();

        // Ensure the split worked correctly
        if parts.len() != 2 {
            continue;
        }

        // Extract the filename and blob hash
        let filename = parts[0];
        let hash = parts[1];

        // Construct a tree entry in Git-style format
        // Format: <mode> <type> <hash> <filename>
        // Example:
        // 100644 blob 37a01f02... avalanche.txt
        let value = format!("100644 blob {} {}", hash, filename);

        // Add the formatted entry to the tree vector
        fin_tree_con.push(value);
    }

    // Concatenate all tree entries into one string separated by new lines
    let tree_content = fin_tree_con.join("\n");

    // Hash the complete tree content as a "tree" object in memory
    let (tree_hash, _) = hash_memory(tree_content.as_bytes(), "tree");

    // Write the tree object to the objects database
    write_to_objects(tree_hash.clone(), tree_content.into_bytes())?;

    // Return the hash of the tree object
    Ok(tree_hash)
}
