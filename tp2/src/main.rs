struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire {
    fn afficher(&self) {
        println!("Compte de {}: Solde = {}", self.nom, self.solde);
    }

    fn deposer(&mut self, montant: f64) {
        if montant > 0.0 {
            self.solde += montant;
            println!("Dépôt de {} effectué. Nouveau solde: {}", montant, self.solde);
        } else {
            println!("Erreur: Le montant du dépôt doit être positif. Montant demandé: {}", montant);
        }
    }

    fn retirer(&mut self, montant: f64) {
        if self.solde >= montant {
            self.solde -= montant;
            println!("Retrait de {} effectué. Nouveau solde: {}", montant, self.solde);
        } else {
            println!("Fonds insuffisants pour retirer {}. Solde actuel: {}", montant, self.solde);
        }
    }

    fn renommer(&self, nouveau_nom: String) -> CompteBancaire {
        CompteBancaire {
            nom: nouveau_nom,
            solde: self.solde,
        }
    }

    fn fermer(self) {
        println!("Compte de {} fermé. Solde final: {}", self.nom, self.solde);
    }
}

fn main() {
    println!("Gestion de comptes bancaires");

    let mut comptes = vec![
        CompteBancaire {
            nom: String::from("Axel"),
            solde: 1000.0,
        },
        CompteBancaire {
            nom: String::from("Marie"),
            solde: 1500.0,
        },
        CompteBancaire {
            nom: String::from("Pierre"),
            solde: 750.0,
        },
    ];

    println!("\n=== Liste des comptes ===");
    for (index, compte) in comptes.iter().enumerate() {
        println!("Compte #{}: {}, Solde: {}", index + 1, compte.nom, compte.solde);
    }

    println!("\n=== Opérations sur le compte d'Axel ===");
    comptes[0].afficher();
    comptes[0].deposer(500.0);
    comptes[0].deposer(-100.0);
    comptes[0].retirer(200.0);

    println!("\n=== Renommage du compte de Marie ===");
    let compte_renomme = comptes[1].renommer(String::from("Marie Dupont"));
    println!("Ancien compte:");
    comptes[1].afficher();
    println!("Nouveau compte:");
    compte_renomme.afficher();

    println!("\n=== Liste finale des comptes ===");
    for (index, compte) in comptes.iter().enumerate() {
        println!("Compte #{}: {}, Solde: {}", index + 1, compte.nom, compte.solde);
    }
}
