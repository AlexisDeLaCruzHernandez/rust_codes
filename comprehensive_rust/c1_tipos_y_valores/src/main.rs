fn main() {
    println!("Hello, world!"); // Al terminar con "!" se indica que es un macro, no una función
    // Macros utililes:
    // println!() -> imprime a la salida estándar con un salto de línea al final
    // format!() -> devuelve un String con el texto formateado, no imprime nada
    // dbg!() -> imprime a la salida estándar el valor de una expresión, útil para depuración
    // todo!() -> genera un panic con el mensaje "not yet implemented", útil para marcar código que aún no se ha implementado
    // unreachable!() -> genera un panic con el mensaje "internal error: entered unreachable code", útil para marcar código que no debería ejecutarse nunca

    // Una variable se declara con la palabra clave "let", se especifica el tipo con ":"
    let x: i32 = 5; // Declaración de una variable inmutable
    println!("El valor de x es: {}", x);
    // x = 6; // Esto generará un error de compilación porque x es inmutable
    let mut y: i32 = 10; // Declaración de una variable mutable
    println!("El valor de y es: {}", y);
    y = 15; // Esto es válido porque y es mutable
    println!("El nuevo valor de y es: {}", y);

    // Los tipos de variables son los siguientes:
    // Enteros con signo: i8, i16, i32, i64, i128, isize
    let _a: i8 = -121; // Cuando no se vuelve a usar la variable, se puede poner un guion bajo al inicio del nombre para evitar advertencias del compilador
    let _b: i32 = 1_000_000_i32; // Se pueden usar guiones bajos para mejorar la legibilidad de los números grandes y especificar el tipo con un sufijo
    // Es equivalente a: 1000000; 1000_000; 1000000_i32; 1_000_000i32; 1000000i32 
    // Enteros sin signo: u8, u16, u32, u64, u128, usize
    let _c: u8 = 250;
    // Flotantes: f32, f64
    let _d: f64 = 3.14159;
    let _e: f32 = -10.0e2; // Notación científica
    // Valores escalares Unicode: char
    let _f: char = 'R';
    // Booleanos: bool
    let _g: bool = true;

    // Los tipos iN, uN y fN tienen un tamaño de N bits, donde N puede ser 8, 16, 32, 64 o 128
    // Los tipos isize y usize dependen del tamaño de la arquitectura del sistema (32 bits o 64 bits)
    // El tipo char representa un solo carácter Unicode y ocupa 4 bytes (32 bits)
    // El tipo bool representa un valor booleano (true o false) y ocupa 1 byte (8 bits)

    // No necesariamente hay que especificar el tipo de variable, Rust puede inferirlo automáticamente
    // Esto lo hace dependiendo del valor que se le asigne a la variable, funciones a las que se llame, etc.
    // Al inferir el tipo, Rust puede determinar si es un entero, flotante, booleano, etc. Pero siempre queda con ese tipo, no puede cambiarlo después
    // Por defecto los enteros se infieren como i32 y los flotantes como f64

    let n = 20; // Rust infiere que n es de tipo i32
    println!("fib({n}): {}", fibonacci(n)); // Se puede usar la sintaxis de interpolación de cadenas con "{}" y el nombre de la variable entre llaves

    // Formatos de texto:
    println!("Segundo: {1}, Primero: {0}", "primero", "segundo"); // Se puede especificar el orden de los argumentos a mostrar
    println!("{variable}: {valor}{unidad}", variable = "Temperatura", valor = 40, unidad = "°C"); // Se pueden nombrar específicamente los argumentos a mostrar
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
    // Para truncar fraccionarios se hace lo siguiente
    let pi = 3.141592;
    println!("Pi is: {pi:.decimals$}", decimals = 3);
    // Formas equivalentes:
    println!("Pi is: {number:.3}", number = pi);
    println!("Pi is: {0:.1$}", pi, 3);
}

// Las funciones se definen con la palabra clave "fn", seguida del nombre de la función, los parámetros entre paréntesis y el tipo de retorno después de "->"
// Los parametros se declaran con su nombre y tipo, separados por dos puntos. Se pueden declarar múltiples parámetros separados por comas
// Siempre toman un número fijo de parámetros, no se pueden declarar funciones con un número variable de parámetros, tampoco argumentos predeterminados
// Las macros se pueden utilizar para crear funciones con un número variable de parámetros, pero no es lo mismo que una función normal
// Siempre se utiliza un solo conjunto de tipos de parámetros. Estos tipos pueden ser genéricos.
fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        // El caso base de la recursión: si n es 0 o 1, se devuelve n
        return n; // Para retornar valores en Rust se puede utilizar la palabra clave "return"
    } else {
        // El caso recursivo
        fibonacci(n - 1) + fibonacci(n - 2) // O tambien se puede retornar omitiendo el "return" y el punto y coma al final de la expresión
    }
}
