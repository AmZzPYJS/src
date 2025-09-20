use std::io;

fn main() {
    let mut input = String::new();
    println!("Devine mon nombre !");

    io::stdin()
        .read_line(&mut input)
        .unwrap(); 

    println!("Tu as saisi : {}", input.trim());
}
