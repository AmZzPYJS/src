use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let nombre = rand::thread_rng().gen_range(1..101);
    println!("Devine mon nombre !");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let conv: u32 = input.trim().parse().expect("Veuillez entrer un nombre valide");

        match conv.cmp(&nombre) {
            Ordering::Less => println!("Le nombre est plus grand !"),
            Ordering::Greater => println!("Le nombre est plus petit !"),
            Ordering::Equal => {
                println!("Tu as trouvé le bon nombre !");
                break;
            }
        }
    }

    
}