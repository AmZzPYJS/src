//! # Jeu du Nombre Mystère
//!
//! Ce crate est un petit jeu en ligne de commande où l’utilisateur doit
//! deviner un nombre généré aléatoirement par l’ordinateur.
//!
//! ## Fonctionnalités
//! - Génération d’un nombre aléatoire entre 1 et 100.
//! - Lecture d’une saisie clavier.
//! - Comparaison entre la saisie et le nombre secret.
//! - Affichage du résultat (trop grand, trop petit ou trouvé).
//!
//! Exécutez ce programme avec `cargo run`.

use std::io; /// Importe le module pour l’entrée/sortie.
use std::cmp::Ordering; /// Importe le module pour la comparaison (Less, Greater, Equal).
use rand::Rng; /// Importe le module pour générer des nombres aléatoires.

/// Lit un entier non signé (`u32`) depuis l'entrée standard.
/// 
/// Retourne `Some(nombre)` si la saisie est valide, sinon `None`.
///
/// # Examples
/// ```
/// let input = read_int_from_stdin();
/// assert!(input.is_none()); // Rien n’est saisi dans cet exemple
/// ```
fn read_int_from_stdin() -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<u32>().ok()
}

/// Compare la saisie de l’utilisateur avec le nombre secret.
/// 
/// Retourne un [`Ordering`] :
/// - `Less` si la saisie est plus petite.
/// - `Greater` si elle est plus grande.
/// - `Equal` si elle est identique.
fn get_ordering(secret_number: u32, input: u32) -> Ordering {
    input.cmp(&secret_number)
}

/// Affiche un message en fonction du résultat de la comparaison.
fn display_result(comparison: Ordering) {
    match comparison {
        Ordering::Less => println!("Le nombre est plus grand !"),
        Ordering::Greater => println!("Le nombre est plus petit !"),
        Ordering::Equal => println!("Tu as trouvé le bon nombre !"),
    }
}

/// Vérifie si l’utilisateur a trouvé le nombre secret.
/// 
/// Retourne `true` si les deux valeurs sont égales, sinon `false`.
fn has_found(comparison: Ordering) -> bool {
    comparison == Ordering::Equal
}

/// Point d’entrée du programme.
/// 
/// - Génère un nombre secret.
/// - Demande à l’utilisateur de saisir un nombre.
/// - Compare et affiche le résultat jusqu’à ce que l’utilisateur trouve la bonne valeur.
fn main() {
    let nombre = rand::thread_rng().gen_range(1..=100);
    println!("Devine mon nombre !");
    #[cfg(debug_assertions)]
    println!("(Debug) Le nombre secret est : {nombre}");

    loop {
        println!("Entrez un nombre :");
        let input = read_int_from_stdin();

        if let Some(input) = input {
            let comparison = get_ordering(nombre, input);
            display_result(comparison);

            if has_found(comparison) {
                break;
            }
        } else {
            println!("Saisie incorrecte");
        }
    }
}
