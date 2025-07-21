// TP Evaluation 

// Créer un fichier tp1.rs 
// dans ce TP vous Créer un compte bancaire 
// avec ce Menu    let options = ["Afficher solde","Retrait","Liste comptes","Quitter"];
// et les actions associés

use std::io; 

fn main() {
    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];
    let mut solde: f32 = 1000.0;

    println!("Menu:");
    for (i, option) in options.iter().enumerate() {
        // afficher chaque option et on commence par 1 
        println!("{}.{}", i + 1, option); 
    }

    println!("Veuillez saisir un numéro de votre choix:");

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Attention erreur de lecture");
    
    let choix: usize = match choix.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez saisir un numero valide");
            return;
        }
    };

    if choix == 0 || choix > options.len() {
        println!("choix hors système !! limite système");
    } else {
        println!("Vous avez sélectionné : {}", options[choix - 1]);
        match choix {
            1 => {
                println!("Votre solde est : {:.2} €", solde);
            }
            2 => {
                println!("Entrez le montant à retirer :");
                let mut montant = String::new();
                io::stdin().read_line(&mut montant).expect("Erreur de lecture");
                let montant: f32 = match montant.trim().parse() {
                    Ok(m) => m,
                    Err(_) => {
                        println!("Montant invalide");
                        return;
                    }
                };
                if montant > solde {
                    println!("Solde insuffisant !");
                } else {
                    solde -= montant;
                    println!("Retrait effectué. Nouveau solde : {:.2} €", solde);
                }
            }
            3 => {
                println!("Liste des comptes");
            }
            4 => {
                println!("Au revoir !");
            }
            _ => {}
        }
    }
}
