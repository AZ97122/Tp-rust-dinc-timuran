/* ############################## */
/* TP de contrôle contenu Rust */
/* ############################## */

// Librairie
use std::io;

// La structure compte bancaire
struct CompteBancaire {
    nom: String,
    solde: f64,
}

// Le trait pour afficher les données en lecture seule
trait Afficher {
    fn afficher(&self);
}

// On implémente le trait Afficher pour la structure Compte, l'affichage de la solde est formaté avec 2 décimales
impl Afficher for CompteBancaire {
    fn afficher(&self) {
        println!("Compte de : {}, Solde : {:.2} €",self.nom, self.solde)
    }
}

// Méthodes associés au compte bancaire
impl CompteBancaire {
    // methode deposer avec mut self pour que la méthode puisse modifier la valeur
    fn deposer(&mut self, montant: f64) {
        self.solde += montant;
        println!("Vous avez déposé {} € sur le compte de {}", montant, self.nom); 
    }
    // methode retirer
    fn retirer(&mut self, montant: f64) {
        if self.solde >= montant {
            self.solde -= montant;
            println!("Vous avez retiré {} € du compte de {}", montant, self.nom);
        }
        // Dans le cas ou il n'y aurai pas assez de fonds sur le compte
        else {
            println!("Impossible de retirer cette somme, il n'y a pas assez de fonds sur le compte de {}", self.nom);
        }
    }

    // methode pour fermer un compte
    fn fermer(&mut self) {
        println!("Le compte de {} a été fermé", self.nom);
        self.solde = 0.0;
    }
}

// Méthode de lecture de chaîne qui affiche un prompt et renvoie la chaîne en String
fn lire_str(prompt: &str) -> String {
    println!("{}", prompt);
    let mut saisie = String::new();
    io::stdin().read_line(&mut saisie).expect("Erreur de lecture");
    // Pour formater la chaîne
    saisie.trim().to_string()
}

// Méthode de lecture mais cette fois pour un float
fn lire_float(prompt: &str) -> f64 {
    // C'est une boucle infinie jusqu'à ce qu'on saisisse un float positif
    loop {
        let input = lire_str(prompt);
        // on transforme la chaîne en float
        match input.parse::<f64>() {
            Ok(valeur) if valeur >= 0.0 =>return valeur,
            // Dans le cas ou l'input n'est pas correct
            _=> println!("Veuillez saisir un nombre valide et positif")
        } 
    }
}

// Méthode de lecture similaire a celle du float mais pour des entiers
fn lire_usize(prompt: &str) -> usize {
    loop {
        let input = lire_str(prompt);
        match input.parse::<usize>() {
            Ok(valeur) => return valeur,
            _ => println!("Veuillez saisir un nombre entier valide."),
        }
    }
}

// Méthode d'affichage des comptes bancaires qui en prend un vecteur de compte en ref
fn afficher_comptes(comptes: &Vec<CompteBancaire>) {
    println!("Liste des comptes bancaires : ");
    // Boucle d'affichage
    for (i, compte) in comptes.iter().enumerate() {
        println!("{} - ", i + 1);
        compte.afficher();
    }
}

fn main() {
    // Initialisation avec 3 comptes bancaires avec un vecteur mutable
    let mut comptes: Vec<CompteBancaire> = vec![
        CompteBancaire { nom: String::from("Azerty"), solde: 500.0 },
        CompteBancaire { nom: String::from("Qwerty"), solde: 800.0 },
        CompteBancaire { nom: String::from("Azqwerty"), solde: 1300.0 },];

        // Le menu
    loop {
        println!("\n===== Menu =====");

        println!("1. Afficher les comptes existants");
        println!("2. Effectuer un dépôt");
        println!("3. Effectuer un retrait");
        println!("4. Fermer un compte");
        println!("5. Créer un nouveau compte");
        println!("3. Quitter l'application");

        let choix = lire_usize("Entrer le numéro correspondant à l'action souhaitée : ");

        // Gestio ndes différents choix
        match choix {
            1 => {
                afficher_comptes(&comptes);
            }
            // Pour le dépôt
            2 => {
                // On affiche la liste des comptes
                afficher_comptes(&comptes);
                let liste_depot = lire_usize("Sélectionner le numéro du compte pour le dépôt : ");
                // Gestion des nombres invalides
                if liste_depot == 0 || liste_depot > comptes.len() {
                    println!("Le numéro de compte est invalide");
                    continue;
                }

                // L'utilisateur peut entrer le montant du dépot et on appelle la méthode deposer
                let montant = lire_float("Montant du dépôt : ");
                comptes[liste_depot - 1].deposer(montant);
            }
            // Similaire que le 2 mais pour le retrait
            3 => {
                afficher_comptes(&comptes);
                let liste_retrait = lire_usize("Sélectionnez le numéro de compte pour retirer : ");
                if liste_retrait == 0 || liste_retrait > comptes.len() {
                    println!("Numéro de compte invalide");
                    continue;
                }
                let montant = lire_float("Montant du retrait : ");
                comptes[liste_retrait - 1].retirer(montant);
            }
            // Fermeture du compte
            4 => {
                afficher_comptes(&comptes);
                    let liste_fermeture = lire_usize("Sélectionnez le numéro de compte à fermer :");
                    // Gestion des erreurs
                    if liste_fermeture == 0 || liste_fermeture > comptes.len() {
                        println!("Numéro de compte invalide.");
                        continue;
                    }
                    comptes[liste_fermeture - 1].fermer();
                    // Suppression ducompte
                    comptes.remove(liste_fermeture - 1);
            }
            5 => {
                let nom = lire_str("Nom du nouveau compte :");
                let solde = lire_float("Solde initial :");
                // Rajout du nouveau compte
                comptes.push(CompteBancaire { nom, solde });
                println!("Nouveau compte créé !");
            }
            // Pour quitter l'appli
            6 => {
                println!("Décconnexion...");
                break;
            }

            _ => println!("Choix non valide"),
        }
    }
}