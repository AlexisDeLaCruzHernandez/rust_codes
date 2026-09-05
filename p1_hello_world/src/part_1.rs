pub fn print_first_part() {
    // Comentario de una linea

    /* Comentario
    Multi-linea */

    // El _ al comienzo del nombre de la variable indica que no se utilizará luego de crearla
    let _text = format!("Hola"); // format! devuelve un String
    // El print! es como el format! pero lo muestra el texto en pantalla
    println!("Hello World!"); // println! es una macro que imprime en pantalla (io::stdout) y añade un \n
    print!("Without \\n\n"); // print! imprime en pantalla sin añadir el \n por defecto
    eprintln!("Error"); // eprintln! imprime en io::stderr (añade el \n). eprint! no añade el \n
    println!("I'm a Rustacean!");

    let x = 5 + /* 90 + */ 5; // Se puede poner un comentario entre lineas de código

    println!("X is {}", x); // Para añadir una variable en print se indica entre {}
    // Se puede especificar el orden de las variables a mostrar, 0 es la primer variable que se incluye
    println!("Segundo: {1}, Primero: {0}", "primero", "segundo"); 
    // Se pueden nombrar especificamente cada argumento
    println!("{variable}: {valor}{unidad}", variable = "Temperatura", valor = 40, unidad = "°C");

    // Para imprimir en otro formato se indica con ':'
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binario):      {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    // Para imprimir punteros se puede usar println!("{pointer:p}", pointer = ...);
    // Agregando un # incluye el formato: {:#x} imprimirá el 0x adelante del número

    // Se puede justificar el texto a la derecha a determinada distancia
    // En el siguiente caso tiene 5 espacios, imprimirá 4 espacios y un 1
    println!("{number:>5}", number=1);
    // Se puede ademas añadir numeros a la izquierda para rellenar los espacios
    println!("{number:0>5}", number=1); // 00001
    // Se puede hacer lo mismo alineando a izquierda añadiendo numeros a la derecha
    println!("{:-<5}", 1); // 13333
    // Tambien se puede alinear al centro con '^'
    // Se puede nombrar tambien el argumento del ancho
    println!("{number:>width$}", number = 1, width = 6);

    // Solo se puede incluir entre los {} los tipos que implementan fmt::Display
    // Por defecto los nuevos tipos no lo implementan

    #[allow(dead_code)] // Quita la adverencia de no utilizado, solo valido para la linea inmediatamente debajo
    struct Structure(i32); // -> No se podrá imprimir porque no tiene fmt::Display implementado

    // Para truncar fraccionarios se hace lo siguiente
    let pi = 3.141592;
    println!("Pi is: {pi:.decimals$}", decimals = 3);
    // Formas equivalentes:
    println!("Pi is: {number:.3}", number = pi);
    println!("Pi is: {0:.1$}", pi, 3);

    // Por último, se pueden nombrar directamente las variables a mostrar en los argumentos
    let num = 1.2;
    let width = 4;
    println!("{num:>width$}");
}
