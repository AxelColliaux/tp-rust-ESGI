use std::io;
use chrono::Utc;

#[tokio::main]
async fn main() {
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

    // Tableau
    let tableau = [1, 2, 3, 4, 5];
    for i in 0..tableau.len() {
        println!("Élément {}: {}", i, tableau[i]);
    }

    // &elt itération sur des références aux élements du tableau 
    // &tab on passe une référence au tableau pour éviter de prendre la possession du tableau entier
    for &elt in &tableau {
        println!("Élément: {}", elt);
    }

    // Les loops
    let mut compteur = 0;
    loop {
        compteur += 1;
        if compteur == 3 {
            break;
        }
        println!("Compteur: {}", compteur);
    }

    let mut compteur2 = 0;
    while compteur2 < 5 {
        compteur2 += 1;
        println!("Compteur 2: {}", compteur2);
    }

    struct Salarie {
        nom: String,
        age: u32,
        ville: String
    }

    let salarie1 = Salarie {
        nom: String::from("Alice"),
        age: 28,
        ville: String::from("Paris"),
    };

    println!("Nom: {}, Age: {}, Ville: {}", salarie1.nom, salarie1.age, salarie1.ville);

    let nombre = 10;
    match nombre {
        1 => println!("Un"),
        2 => println!("Deux"),
        3 => println!("Trois"),
        _ => println!("Autre nombre"),
    }

    struct Personne {
        nom: String,
    }

    impl Personne {
        fn afficher(&self) {
            println!("La personne suivante {} est convoqué", self.nom);
        }
    }
    
    let personne = Personne {
        nom: "Alexandre".to_string()
    };

    personne.afficher();

    // Usage  de chrono 
    let maintenant = Utc::now();
    println!(" la date et l heure actuelle UTC est {}",maintenant);
    println!("Format FR : {}", maintenant.format("%d/%m/%Y")); // Format 24/07/2025
    println!("Format FR : {}", maintenant.format("%d/%m/%Y %H:%M:%S")); // Format 24/07/2025 10:18:22

    // Notions d'ownership (propriétaire) et membership ( appartenance  à une structure)
      // => pour garantir la sécurité mémoire 
        //1. Ownership
           // chaque valeur a un propriétaire unique, responsable de libérer la mémoire
           // lorsqu'elle sort du scop 
           // quand le propriétaire est déplacé, l'ancien propriétaire ,ne peut plus y accéder
           // quand le propriétaire sort du scop, la valeur est automatiquement libérée 

         //exemple : 

         let prenom = String::from("Nourddine"); // prenom est propriétaire de la String
         // println!("{}",prenom); 
         let secu = String::from("1897272824252");
         let prenom2 = prenom.clone();
         greetings(prenom); // propriétaire est transferé à la fonction greetings()
          println!("{}",prenom2); 

          greetings2(&secu);  // emprunt immuable 
          println!("{}",secu); 

       // 3 MemberShip : ( Appartenance à une structure )
          // décrit quelles sont les données contenues dans une structure Struct

          // exemple :

          let user = User {
            nom : String::from("Alexandre"),
            secu: String::from("1825678290 55")
          };
        
         println!("nom {}",user.nom);
         // display(&user); // &emprumter un champ d'une structure
         display(user); 
        //  async_test().await; // Appel de la fonction asynchrone
        async_test2().await; // Appel de la fonction asynchrone
    }

    struct User {
        nom: String,
        secu: String,
    }
    
    // en C/C++     int age = 25; 
    //           Contact contact1; 
    //     usage contact1.age  contact.prenom 
    //  public  class Voiture ( int puissance , String couleur , Vec  marque) {}
    // fonction display
    fn display(user: User) -> User{
      println!(" Nom: {}, num secu : {}", user.nom, user.secu);
      user
    }


  // avec emprunt & 
      fn greetings2(msg:&String){
      println!("Hello Mister {}",msg);
     }   

  // sans emprunt
     fn greetings(msg:String){
      println!("Hello Mister {}",msg);
     
}

fn addition(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn say_hello(nom: &str) {
    println!("Hello, {}!", nom);
}

struct Compteur {
    value: i32,
}

impl Compteur {
    fn afficher(&self){
        println!("la valeur actuelle :{}", self.value);
    }

    fn incrementer(&mut self){
        self.value += 1;
    }

    fn deplacer(self){
        println!("Déplacé : {}", self.value);  // self déplacé ici, plus accessible 
    }
}

use tokio::time::{sleep, Duration};

// créer une fonction asynchrone et futures 
async fn task ( nom:&str, duree:u64)  -> String{

    println!(" Début de la tâche :{}", nom);
    sleep(Duration::from_secs(duree)).await;
    println!("Fin de tâche :{}", nom);
    format!("Resultat de {}", nom)
}

async fn async_test() {


println!("début de mon programme !");
// je crée une fonction asynchrone aui attend 3 secondes 
sleep(Duration::from_secs(3)).await;

println!(" fin du programme après 3 secondes"); 

let resultat = task("Task1",5).await;
println!("Résultat reçu : {}", resultat); 


}

async fn async_test2() {
    
    let  debut = std::time::Instant::now();

    println!("début de mon programme ! 222");
    // je crée une fonction asynchrone qui attend 3 secondes 
      sleep(Duration::from_secs(3)).await;
      println!(" fin du programme après 3 secondes"); 
      let resultat = task("Task1",5).await;
        println!("Résultat 1 reçu : {}", resultat); 
      let resultat2 = task("Task1",5).await;
        println!("Résultat 2 reçu : {}", resultat2); 
      let resultat3 = task("Task1",10).await;
        println!("Résultat 3 reçu : {}", resultat3); 

       println!("Temps total : {:?}", debut.elapsed()); 
    

     // si on veut lancer 3 tâches en parallèle on utilise  join
     // use tokio::join   appel de la bibliothèque avant le main 
     // sinon directement 
     /*
                  let ( res1, res2, res3 ) = tokio::join!(
                          task("Task1",3),
                          task("Task2",5),
                          task("Task3",3),
                  )
     

     */
}