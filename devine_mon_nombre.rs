use std::io;
use rand::Rng;

fn main() {
    let nombre = rand::thread_rng().gen_range(1..101);
    println!("Devine mon nombre !");
    println!("Le nombre est {nombre}");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let conv: u32 = input.trim().parse().expect("Veuillez entrer un nombre valide");

    let message if conv == nombre {
        "Tu as trouvé le bon nombre !"
    } else if xonv < nombre {
        "Le nombre est plus grand !"
    } else {
        "Le nombre est plus petit !"
    };

    println!("{message}");
}