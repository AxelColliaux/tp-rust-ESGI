// créer un programme qui permet de lire et ecrire et modifier, supprimer définitivement  des fichiers
// utiliser les notions abordés ensembles
// utliser les loop et les whiles et match ... OwnerShip et MemberShip
// utiliser les impl pour  la structure 
// ajouter la date de création de fichier et la date de modification

use std::fs::{File, OpenOptions, remove_file};
use std::io::{self, Write, Read};
use chrono::Utc;

fn main() {
    loop {
        println!("1. Créer");
        println!("2. Lire");
        println!("3. Modifier");
        println!("4. Supprimer");
        println!("5. Quitter");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        match choix.trim() {
            "1" => { creer().ok(); }
            "2" => { lire().ok(); }
            "3" => { modifier().ok(); }
            "4" => { supprimer().ok(); }
            "5" => break,
            _ => println!("Choix invalide."),
        }
    }
}

fn creer() -> io::Result<()> {
    println!("Nom du fichier :");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom)?;
    let nom = nom.trim();

    let date = Utc::now().format("%d/%m/%Y %H:%M:%S");
    let contenu = format!("Créé le: {}\nModifié le: {}\n", date, date);

    let mut fichier = File::create(nom)?;
    fichier.write_all(contenu.as_bytes())?;
    println!("Fichier créé.");
    Ok(())
}

fn lire() -> io::Result<()> {
    println!("Nom du fichier :");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom)?;
    let nom = nom.trim();

    let mut fichier = File::open(nom)?;
    let mut contenu = String::new();
    fichier.read_to_string(&mut contenu)?;
    println!("{}", contenu);
    Ok(())
}

fn modifier() -> io::Result<()> {
    println!("Nom du fichier :");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom)?;
    let nom = nom.trim();

    let mut ancien = String::new();
    File::open(nom)?.read_to_string(&mut ancien)?;

    let lignes: Vec<&str> = ancien.lines().collect();
    let creation = if !lignes.is_empty() { lignes[0] } else { "" };
    let date = Utc::now().format("%d/%m/%Y %H:%M:%S");

    println!("Nouveau contenu :");
    let mut nouveau = String::new();
    io::stdin().read_line(&mut nouveau)?;

    let contenu = format!("{}\nModifié le: {}\n{}", creation, date, nouveau);

    let mut fichier = OpenOptions::new().write(true).truncate(true).open(nom)?;
    fichier.write_all(contenu.as_bytes())?;
    println!("Fichier modifié.");
    Ok(())
}

fn supprimer() -> io::Result<()> {
    println!("Nom du fichier :");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom)?;
    let nom = nom.trim();

    remove_file(nom)?;
    println!("Fichier supprimé.");
    Ok(())
}