extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivinhe o numero!");

	let numbero_secreto = rand::thread_rng()
		.gen_range(1, 101);

	// println!("O número secreto é: {}", numbero_secreto);

	loop {
		println!("Digite o seu palpite: ");

		let mut palpite = String::new();

		io::stdin()
			.read_line(&mut palpite)
			.expect("Falha ao ler a entrada");

		let palpite: u32 = match palpite.trim().parse(){
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("Voce disse: {}", palpite);

		match palpite.cmp(&numbero_secreto){
			Ordering::Less => println!("Muito baixo!"),
			Ordering::Greater => println!("Muito alto!"),
			Ordering::Equal => {
				println!("Você acertou!");
				break ;
			}
		}
	}
}