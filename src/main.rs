use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use text_io::read;
#[derive(Serialize, Deserialize, Debug)]
struct Vocabulary {
    words: Vec<Word>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Word {
    term: String,
    definition: String,
}

impl Vocabulary {
    fn load(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }

    fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

// Your CLI stays mostly the same
#[derive(Parser)]
#[command(name = "rs vocab")]
#[command(about = "make your own vocabulary")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Version,
    Remove { term: String },
    Add { term: String, definition: String },
    List,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let vocab_path = PathBuf::from("vocab.json");
    match cli.command {
        Commands::Version => println!("rs vocab 0.1.0"),

        Commands::Add { term, definition } => {
            // Load or create new vocabulary
            let mut vocab = match Vocabulary::load(&vocab_path) {
                Ok(v) => v,
                Err(_) => Vocabulary { words: Vec::new() },
            };

            if vocab.words.iter().any(|w| w.term == term) {
                println!("\"{term}\" already exists.");
                println!("Do you want to modify the definition? 1: true, 0: false.");

                let response: i32 = read!();

                if response == 0 {
                    return Ok(());
                }

                let new_def: String = read!();

                if let Some(word) = vocab.words.iter_mut().find(|w| w.term == term) {
                    word.definition = new_def.trim().to_string();
                    vocab.save(&vocab_path)?;
                }

                println!("Word redefined!");
                return Ok(());
            }
            vocab.words.push(Word { term, definition });
            vocab.save(&vocab_path)?;
            println!("Word added!");
        }

        Commands::List => {
            let vocab = Vocabulary::load(&vocab_path)?;
            for word in &vocab.words {
                println!("{}: {}", word.term, word.definition);
            }
        }
        Commands::Remove { term } => {
            let mut vocab = match Vocabulary::load(&vocab_path) {
                Ok(v) => v,
                Err(_) => Vocabulary { words: Vec::new() },
            };
            if !vocab.words.iter().any(|w| w.term == term) {
                println!("Word doesn't exist in vocabulary.");
                return Ok(());
            } else if let Some(pos) = vocab.words.iter().position(|x| x.term == term) {
                vocab.words.remove(pos);
                println!("Word removed!");
            }
            vocab.save(&vocab_path)?;
        }
    }

    Ok(())
}
