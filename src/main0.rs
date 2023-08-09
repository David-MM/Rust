use std::io;
use rand::Rng;

fn main() {
    println!("¡Bienvenido al juego de adivinanzas!");
    
    let numero_secreto = rand::thread_rng().gen_range(1..101);
    let mut intentos = 0;
    
    loop {
        println!("Por favor, ingresa tu adivinanza (entre 1 y 100):");
        
        let mut adivinanza = String::new();
        io::stdin().read_line(&mut adivinanza)
            .expect("Error al leer la línea");
        
        let adivinanza: u32 = match adivinanza.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        intentos += 1;
        
        match adivinanza.cmp(&numero_secreto) {
            std::cmp::Ordering::Less => println!("Demasiado bajo."),
            std::cmp::Ordering::Greater => println!("Demasiado alto."),
            std::cmp::Ordering::Equal => {
                println!("¡Adivinaste en {} intentos!", intentos);
                break;
            }
        }
    }
}
