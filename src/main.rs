// Authored by MensahPrince
// GMV is GIT Minimal Version

use clap::Parser as ClapParser;
use clap::Subcommand;
use std::io;

#[derive(clap::Parser, Debug)]
#[command(name = "gmv")]
#[command(about = "A GIT minimal version")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new .gmv
    Init { path: String },

    /// Quit the GMV
    Quit,

    /// List the elements in the directory
    Ls,
}

struct InputParser;

impl InputParser {
    fn new() -> Self {
        InputParser
    }

    fn parse(&self, input: &str) -> Result<Cli, clap::Error> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return ClapParser::try_parse_from(vec!["gmv", "--help"]);
        }

        let mut words = vec!["gmv"];
        let split_words: Vec<&str> = trimmed.split_whitespace().collect();

        // If the user already typed 'gmv', don't add it again
        if !split_words.is_empty() && split_words[0] == "gmv" {
            ClapParser::try_parse_from(split_words)
        } else {
            words.extend(split_words);
            ClapParser::try_parse_from(words)
        }
    }
}

//Call and create neccessary dirs during the init of gmv
fn init_gmv(path: &str) -> io::Result<()> {
    std::env::set_current_dir(path)?;
    std::fs::create_dir(".gmv")?;
    std::fs::create_dir(".gmv/objects")?;
    std::fs::create_dir(".gmv/refs")?;
    std::fs::write(".gmv/HEAD", "ref: refs/heads/main")?;
    Ok(())
}

fn main() {
    let parser = InputParser::new();

    loop {
        use std::io::Write;
        print!("gmv> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input_line = String::new();

        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        let trimmed_input = input_line.trim();

        if trimmed_input.is_empty() {
            continue;
        }

        match parser.parse(trimmed_input) {
            Ok(cli) => match cli.command {
                Commands::Init { path } => {
                    if let Err(e) = init_gmv(&path) {
                        println!("gmv: error: {}", e);
                    } else {
                        println!("Initialized GMV at {}", path);
                    }
                }
                Commands::Quit => {
                    println!("Exiting...");
                    break;
                }
                Commands::Ls => match std::fs::read_dir(".") {
                    Ok(entries) => {
                        for entry in entries.flatten() {
                            let file_name = entry.file_name();
                            let display_name = file_name.to_string_lossy();
                            if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                                println!("{}", display_name);
                            }
                        }
                    }
                    Err(e) => println!("fsearch: ls error: {}", e),
                },
            },
            Err(e) => {
                let _ = e.print();
                println!();
            }
        }
    }
}
