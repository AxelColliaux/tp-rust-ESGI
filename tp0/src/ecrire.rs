use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("test.txt")?;
    file.write_all("Bonjour, fichier crée !".as_bytes())?;
    println!("Fichier créé avec succès !");
    Ok(())
}