use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let file = File::open("test.txt");
    let file = match file {
        Ok(f) => {
            println!("Lecture réussie");
            f
        },
        Err(e) => {
            eprintln!("Erreur de lecture du fichier: {}", e);
            return Err(e);
        }
    };
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    println!("Contenu du fichier: {}", content);
    Ok(())
}

