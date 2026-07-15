/*
Ownership son las reglas que explican como se maneja la memoria
Stack y heap: ambos son partes de la memoria disponibles para trabajar
Stack: guarda los valores en el orden que llegan y los remueve en el orden inverso,
es decir, es last in first out (una pila), añadir datos se llama push y extraer pop
Heap: asigna determinado espacio (allocate) de memoria donde tenga lugar retornando un puntero
sobre la localización de ese espacio eb el stack.
Utilizar el stack es más rapido que el heap porque no hay que encontrar espacio de memoria
y no se utilizan punteros para ubicar el dato.
Cuando se llama a una función los valores pasados a la funciín (ya sea por valor o referencia)
y las variables locales de la función se pushean al stack, cuando se termina se hace pop de los valores.

Reglas del ownership:
    - Cada valor tiene un propietario (owner)
    - Solo puede haber un propietario (owner) a la vez
    - Cuando un propietario se va del alcance el valor se desecha
*/

fn main() { 
    { // En este punto la variable s no es valida (no declarada)
    let _s = "Hello"; // A partir de acá en adelante s es valido
    // Este tipo de string es inmutable y tiene un espacio de memoria conocido
    // Se conoce como string literal
    // Utilizar variable s
    } // Acá se termina el alcance de la variable s y ya no es valida

    // Las variables i8, u8, f32, char y bool se almacenan en el stack
    // Para demostrar las reglas del ownership se utilizará el tipo String
    // El tipo String se almacena en el heap y es de tamaño desconocido en tiempo de compilación
    // Para crear un String se puede utilizar
    let mut s = String::from("Hello");
    s.push_str(", world!"); // Ahora s queda como "Hello, world!"
    println!("{s}");
    // En el caso del String se ubica el espacio de memoria en tiempo de ejecución
    // Al llamar a String::from estamos solicitando un espacio en memoria
    // Cuando se llega al fin del alcance con el } rust llama a la función drop que libera el espacio

    let x = 5;
    let _y = x;
    // En este caso se crean dos variables que son iguales a 5, si modificamos x y no se toca
    // Esto sucede ya que al ser de espacio conocido se almacenan en el stack

    let s1 = String::from("Hello");
    let _s2 = s1;
    // Un string está hecho de tres partes, un puntero a la memoria donde está "Hello", 
    // la longitud y la capacidad, ambos en bytes. Por lo tanto estamos copiando la posición de memoria
    // No estamos asignando un nuevo Hello en memoria, es el mismo Hello apuntado por dos variables
    // Esto tiene un problema ya que al salir del alcance se liberaria de memoria dos veces la misma parte
    // eso es un bug y, para prevenirlo, Rust invalida la variable s1, pudiendo usar unicamente s2,
    // corrigiendo el problema de la nueva liberación. A eso se le llama que s1 se "movió" hacia s2.

    let mut _s = String::from("Hello");
    _s = String::from("World");
    // En este caso se está designando un nuevo String completamente
    // Originalmente s apuntaba a "Hello" y luego se asignó un nuevo espacio y apunta a World
    // El primer bloque de memoria de Hello se libera automaticamente al hacer el String::from

    let s1 = String::from("Hello");
    let _s2 = s1.clone();
    // Si se quiere hacer una copia independiente de s1 se puede usar el metodo clone
    // Este metodo asigna un nuevo espacio en memoria con el mismo contenido que s1 pero s2 apunta a ese nuevo espacio

    let s = String::from("Hello"); // s entra al alcance
    takes_ownership(s); // el valor de s va a la función
    // Como s contiene el puntero, longitud y capacidad se envian estos datos a la función
    // Al terminar la función libera la memoria asociada a some_string que coincide con la variable s

    let x = 5; // x entra al alcance
    makes_copy(x); // Como x es un entero la función obtiene una copia del valor del stack
    // Por lo tanto acá x todavía está disponible

    let _s1 = giver_ownership(); // Ahora s1 va a tener lo mismo que some_string
    let s2 = String::from("Hello"); // s2 entra al alcance
    let _s3 = takes_and_gives_back(s2);
    // lo que se hace es que s2 se mueve a s3 pero con pasos extra

    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1); // s1 se mueve a s2 y obtiene la longitud
    println!("The length of '{s2}' is {len}");

    let s1 = String::from("Hello");
    let len = calculate_length_reference(&s1); // Se le "presta" el String s1 a la función
    println!("The length of '{s1}' is {len}");

    let mut s1 = String::from("Hello");
    change_string(&mut s1); // Solo se puede tener una referencia mutable a la vez
    // No podemos hacer:
    let _r1 = &mut s1;
    let _r2 = &mut s1;
    // Y luego utilizar r1, solo r2
    // Tampoco se permiten tener referencias inmutables y mutables a la vez, solo una de las dos
    let _r1 = &s1;
    let _r2 = &s1;
    // Acá se pueden usar tanto r1 como r2, se pueden tener multiples referencias inmutables 
    // ya que ninguna modificará a s1
    let _r3 = &mut s1; // A partir de acá se termina el alcance de r1 y r2, solo se podrá usar r3
    println!("{s1}");

    // Rust no permite que hayan punteros apuntando a basura o datos que ya no existen
    // En caso de detectar esto dará error de compilación, hay un ejemplo abajo del todo
}

fn takes_ownership(some_string: String) { // some_string entra al alcance
    println!("{some_string}");
} // some_string se va del alcance y la memoria es liberada 

fn makes_copy(some_integer: i32) { // some_integer entra al alcance
    println!("{some_integer}");
} // some_integer se va del alcance, no pasa nada especial

fn giver_ownership() -> String {
    let some_string = String::from("yours"); // some_string entra al alcance
    return some_string; // En este caso se devuelve un tipo String, es equivalente al s1 = s2
    // Por lo tanto some_string se mueve a donde se esté llamando y no libera la memoria asociada a "yours"
}

// Forma de no perder los Strings que se envian a las funciones es retornarlas nuevamente
// Esto se soluciona pasando valores por referencia sin trasladar el ownership
fn takes_and_gives_back(a_string: String) -> String { // a_string entra al alcance
    return a_string; // Igualmente a_string se mueve a donde se esté llamando
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    return (s, length);
}

// Pasaje por referencia: se utiliza el simbolo & para indicar que pasamos un referencia
// La referencia es una variable que apunta al mismo lugar que la variable que pasamos.
// Si s1 apunta a 0x33 y pasamos &s1 hacia una funcion con parametro ref: &String ref apuntará a 0x33
fn calculate_length_reference(s: &String) -> usize { // s es una referencia a string
    return s.len();
} // Acá se acaba el alcance de s, como es por referencia no es el propietario del String por lo que no libera la memoria
// Si pasamos el parametro como &String no podemos modificar el valor de la variable pasada por referencia, para eso hay que hacerla mutable

fn change_string(s: &mut String) {
    s.push_str(", world!");
}

/*
fn dangle() -> &String { // Indica que retornará una referencia a un string
    let s = String::from("Hello"); // Crea el string s
    return &s; // Quiere retornar la referencia a la variable s
} // s deja su alcance y va a liberar la memoria del Hello de s, por lo que retornará una referencia a la nada -> Error de compilación
*/