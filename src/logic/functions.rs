use std::io;

pub fn init_gmv(path: &str) -> io::Result<()> {
    std::env::set_current_dir(path)?;

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

pub fn con_hash_256(path: String) -> io::Result<()> {
    println!("Hash Called at {}", path);
    Ok(())
}

pub fn add_gmv(path: String) -> io::Result<()> {
    println!("Added content at {} ", path);
    Ok(())
}
