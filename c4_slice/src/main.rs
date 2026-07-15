fn main() {
    let mut s1 = String::from("Hello world!"); // Creamos s1 mutable
    let _pos = first_word(&s1); // Obtenemos la posición hasta el primer espacio
    s1.clear(); // Se modifica s1
    // El problema es que ahora la posición es erronea y no se adapta a cambios en el String

    // String slices: son porciones del String, tienen de tipo &str
    let s = String::from("Hello, world!");
    let _hello = &s[0..5]; // Guarda un puntero desde 0 con una longitud de 5 - 0 = 5
    let _world = &s[7..12]; // Guarda un puntero desde 7 con una longitud de 12 - 7 = 5
    // Si es desde el principio se puede hacer &s[..10];
    // Si es hasta el final se puede hacer &s[2..];
    // Si es todo se puede hacer &s[..]; 
    let _first_word = first_word_slice(&s); // Ahora que first_word es una referencia a String
    // Si modificamos s con clear por ejemplo se romperá la referencia de first_word
    // Ademas de que first_word de una referencia inmutable, clear necesita una referencia mutable 
    // ya que modifica la String, así que si luego de modificar s queremos usar first_word dara error
    // de compilación antes de dejarnos el error en tiempo de ejecución

    let _literal = "Hello, world"; // Esto se guarda en el binario y es una referencia inmutable 

    // Tambien se pueden hacer slices de otros tipos de datos:
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("2: {}", slice[0]);
}

// Esta función sirve para resolver el siguiente problema:
// Dado un string dar la primer palabra antes del primer espacio
fn first_word(s: &String) -> usize { // Retornará el indice del primer espacio
    let bytes = s.as_bytes(); // Combierte el string en un array de bytes para buscar el espacio

    for (i, &item) in bytes.iter().enumerate() {
        // El conjunto iter con enumerate hace que retorne una tupla con el indice y su valor como referencia
        // Siendo i el indice y &item la referencia del valor
        if item == b' ' { // Acá se asegura si es un espacio o no
            return i; // Retorna el indice donde se encontró el espacio
        }
    }

    s.len() // Si no hay espacios se quiere la palabra completa
}

fn first_word_slice(s: &str) -> &str { // Se puede recibir un slice (o literal string, son lo mismo) para generalizar
    let bytes = s.as_bytes(); // Combierte el string en un array de bytes para buscar el espacio

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { 
            return &s[0..i];
        }
    }

    &s[..] // Si no hay espacios se quiere la palabra completa
}
