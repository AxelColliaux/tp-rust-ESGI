use std::io;

fn main() {
    let nom = "Kevin";
    let age: u32 = 30;
    let age_papa = 60;
    let temperature: f32 = 36.6;

    println!("Hello, world!");
    println!("J'ai {} ans", age);
    println!("Mon nom est {}", nom);
    println!("Mon père a {} ans", age_papa);
    println!("Ma température est de {} degrés", temperature);



    let result = addition(5, 10);
    println!("Le résultat de l'addition est {}", result);
    say_hello(nom);

    let nombre =16;
    if nombre % 2 == 0 {
        println!("{} est un nombre pair", nombre);
    } else {
        println!("{} est un nombre impair", nombre);
    }

    for i in 1..=10 {
        println!("i vaut {}", i);
    }

    let voitures = ["Renault", "Peugeot", "Citroën"];
    for voiture in voitures.iter() {
        println!("Voiture: {}", voiture);
    }

    for (i, voiture) in voitures.iter().enumerate() {
        println!("Index {}: {}", i + 1, voiture);
    }
    // Iter() crée un itérateur sur la collection
    // Enumerate() transforme l'itérateur en une séquence d'index et de valeurs

    // Exemple de vecteur
    let noms = vec!["Alice", "Bob", "Charlie"];
    for (i, nom) in noms.iter().enumerate() {
        println!("Nom {}: {}", i, nom);
    }

    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

    println!("Menu:");
    for (i, option) in options.iter().enumerate() {
        println!("{}: {}", i + 1, option);
    }
    println!("Veuillez saisir un numéro de votre choix:");
    let mut choix = String::new();
    io::stdin()
        .read_line(&mut choix)
        .expect("Erreur de lecture");
    let choix: usize = match choix.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un numéro valide.");
            return;
        }
    };
    if (choix < 1 || choix > options.len()) {
        println!("Choix invalide. Veuillez réessayer.");
    } else {
        println!("Vous avez choisi: {}", options[choix - 1]);
    }
}

fn addition(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn say_hello(nom: &str) {
    println!("Hello, {}!", nom);
}