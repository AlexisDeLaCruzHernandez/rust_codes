use std::cmp::Ordering; // Nos incluye Less, Greater y Equal para comparar valores
use std::io; // Incluye la libreria io de la libreria estandar std
use rand::Rng; // Incluye la caracteristica Rng de rand

fn main() {
    println!("Guess the number!");

    // Primero rand::thread:rng() indica el generador vamos a usar,
    // está basado en el hilo de ejecución con una seed del OS.
    // El metodo gen_range está definido en Rng y toma un rango para el numero
    // El rango está tomado como: inicio..=fin, incluye tanto el valor de inicio como el de fin.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    // Hace un loop infinito
    loop {
        
        println!("Please input your guess.");
        
        // let se usa para crear variables, ej: let a = 5;
        // Por defecto son inmutables, así que su valor no puede cambiar, cambiarlo crearia otra variable
        // Para hacerlas mutables se añade mut
        // String::new() instancia un String vacio en la variable guess (puede crecer el string)
        // El :: indica que new es una función asociada al tipo String.
        // Resumiendo: se crea un String vacio mutable
        let mut guess = String::new();

        // Se llama a la funcion stdin que está en la libreria io
        // Se podría haber llamado usando std::io::stdin sacando la linea 1
        // La función devuelve una instancia de std::io::Stdin que es un tipo que representa
        // un handler al standar input del terminal
        io::stdin()
            // .read_line llama a un metodo del handler del standar input
            // Lo que hace es un append (agregar al final) de lo ingresado sobre la variable guess
            // El & indica que es por referencia, las referencias tambien son inmutables por defecto
            // así que se añade el mut para hacerla mutable
            .read_line(&mut guess)
            // read_line ademas de cargar datos en la variable devuelve un valor de Result, que es un enum
            // Cada estado posible del enum se llama variante.
            // Las variantes de Result son Ok y Err, el Err incluye info de como y porque falló
            // El metodo expect hace que si retorna Err el programa crashee y muestre el mensaje por pantalla
            // Si devuelve Ok simplemente retornará el numero de bytes en el input del usuario
            // Si no se usa el .expect al compilar dará un warning
            .expect("Failed to read line");
        
        // Se pueden "crear" variables con el mismo nombre, ya que la nueva va a prevalecer, se usa para cambiar tipos
        // El metodo trim de String elimina los espacios al inicio y final como tambien los \r\n del teclado
        // El metodo parse sirve para tranformar el String a otro tipo que debe ser especificado
        // usando : despues del nombre de la variable le indica a Rust que se especifica un tipo, en este caso seria un uint32_t de C
        // En este caso el expect al recibir un Ok devuelve el numero que queremos
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // Si no queremos crashear el programa podemos hacer un handle de los Result
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }, // El _ indica que cualquier valor en Err sirve, continue va al siguiente loop
        };
        // en el println! se puede imprimir variables poniendolas entre {}
        // Si se quiere poner una expresión se dejan los {} vacios y se pone al final despues de una coma
        // Ej: let x = 5; let y = 10;
        //     println!("x = {x} and y + 2 = {}", y + 2);
        println!("You guessed: {guess}");
        
        // El metodo cmp se usa para comparar valores y devuelve un Ordering
        // El match se utiliza para saber que hacer basado en el Ordering que devuelve cmp. Es como el switch
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                // El break rompe el loop como en C
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
