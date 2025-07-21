# Notes de cours

## 21/07/2025

### Rust

C’est un langage compilé, possibilité de faire de l’embarqué, gestion de la mémoire sécurisé

### Cargo

Cargo est un gestionnaire de package pour rust

Les fichiers Cargo.lock et Cargo.toml sont l’équivalent des package.json en javascript 

### Commandes essentielles :

`cargo build`  : compile le projet mode debug par defaut
`cargo check`   : vérifie le code sans produire de binaire
`cargo update`  met à jour les dépendances
`cargo doc --open`  : gènère la docutmentation et l'ouvre dans le navigateur Web

`cargo run` : compile et exécute le projet

`main.rs`  est le point d’entrée

### Typage

Par convention on utilise le snake_case

Typages : 

- `i32` : entier signé, 32 bits
- `u32` : entier non signé, 32 bits
- `i64` : entier signé, 64 bits
- `u8` : entier non signé, 8 bits (plage : 0 à 255)
- `f32` : nombre à virgule flottante (float), 32 bits, précision simple
- `&str` : référence à une chaîne de caractères
- **i** : signed (négatif/positif), **u** : unsigned (positif uniquement)

Le typage n’est pas obligatoire par exemple :

```bash
let age = 30; 
```

Le typage sera automatiquement i32

### Fonctions

On définit une fonction avec `fn`  

```rust
fn addition(n1: i32, n2: i32) -> i32 {
        n1 + n2
    }

let result = addition(5, 10);
println!("Le résultat de l'addition est {}", result);
```

Le return est automatique

### Conditions

```rust
let nombre =16;
if nombre % 2 == 0 {
    println!("{} est un nombre pair", nombre);
} else {
    println!("{} est un nombre impair", nombre);
}
```

### Boucles

```rust
for i in 1..=10 {
    println!("i vaut {}", i);
}
```

..= intervalle inclusif (fin incluse)

.. intervalle exclusif (fin excluse)

Autre exemple

```rust
let voitures = ["Renault", "Peugeot", "Citroën"];
for voiture in voitures.iter() {
    println!("Voiture: {}", voiture);
}
```

```rust
for (i, voiture) in voitures.iter().enumerate() {
    println!("Index {}: {}", i + 1, voiture);
}

// Iter() crée un itérateur sur la collection
// Enumerate() transforme l'itérateur en une séquence d'index et de valeurs
```

```rust
// Exemple de vecteur
let noms = vec!["Alice", "Bob", "Charlie"];
for (i, nom) in noms.iter().enumerate() {
    println!("Nom {}: {}", i, nom);
}
```

### Tableau

```rust
let tableau = [1, 2, 3, 4, 5];
for i in 0..tableau.len() {
    println!("Élément {}: {}", i, tableau[i]);
}

// &elt itération sur des références aux élements du tableau 
// &tab on passe une référence au tableau pour éviter de prendre la possession du tableau entier
for &elt in &tableau {
    println!("Élément: {}", elt);
}
```

### Loop

```rust
let mut compteur = 0;
loop {
    compteur += 1;
    if compteur == 3 {
        break; // On sort de la bouvle quand le compteur atteins 3
    }
    println!("Compteur: {}", compteur);
}
```

```rust
let mut compteur2 = 0;
while compteur2 < 5 {
    compteur2 += 1;
    println!("Compteur 2: {}", compteur2);
}
```

### Struct

```rust
struct Salarie {
    nom: String,
    age: u32,
    ville: String
}

let salarie1 = Salarie {
    nom: String::from("Kevin"),
    age: 28,
    ville: String::from("Paris"),
};

**println!("Nom: {}, Age: {}, Ville: {}", salarie1.nom, salarie1.age, salarie1.ville);**
```

### Match (Switch)

```rust
let nombre = 10;
match nombre {
    1 => println!("Un"),
    2 => println!("Deux"),
    3 => println!("Trois"),
    _ => println!("Autre nombre"),
}
```

### Implementation

```rust
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
```

&self ⇒ Lecture seule

self ⇒ transfère complet

&mut self ⇒ modification possible