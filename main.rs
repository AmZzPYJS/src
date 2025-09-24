use std::io; //! Importe le module pour Entrée/sortie
use std::cmp::Ordering; //! Importe le module pour comparaison (Less, Greater, Egual) entre 2 variables de même type
use rand::Rng; //! Importe le module random pour génerer aléatoirement des nombres
fn read_int_from_stdin() -> Option<u32> {
    /// Cette fonction créer une nouvelle chaine de caractère input, ouvre l'entré clavier, récupère les valeurs et la stock dans input
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<u32>().ok()
}

fn get_ordering(secret_number: u32, input: u32) -> Ordering {
    /// Cette fonction permet d'obtenir le résulat de la comparaison entre deux variables de même type
    input.cmp(&secret_number)
}

fn display_result(comparison: Ordering) {
    /// Cette fonction définit l'output, la valeur de sortie pour chaque comparaison qu'on fera.
    match comparison {
        Ordering::Less => println!("Le nombre est plus grand !"),
        Ordering::Greater => println!("Le nombre est plus petit !"),
        Ordering::Equal => println!("Tu as trouvé le bon nombre !"),
    }
}

fn has_found(comparison: Ordering) -> bool {
    /// Cette fonction permet de dire si nous trouvons la même valeur pour nos deux variables, retourne vrai, sinon retourne faux.
    comparison == Ordering::Equal
}

fn main() {
    /// Génère un nombre secret entre 1 et 100 inclus
    let nombre = rand::thread_rng().gen_range(1..=100);
    /// Envoi un message pour nous faire deviner le nombre
    println!("Devine mon nombre !");

    loop {
        /// Envoi un message pour nous insister à taper un nombre sur notre clavier, et ouvre l'entrée clavier
        println!("Entrez un nombre :");
        let input = read_int_from_stdin();

        /// Nous comparons la valeur d'entrée avec notre valeur nombre et bous arretons le programme si celui-ci est trouvé
        if let Some(input) = input {
            let comparison = get_ordering(nombre, input);
            display_result(comparison);

            if has_found(comparison) {
                break;
            }
        /// Sinon si on ne trouve pas la bonne valeur, ça renvoie 'Saisie incorrecte' et le loop continue
        } else {
            println!("Saisie incorrecte");
        }
    }
}
