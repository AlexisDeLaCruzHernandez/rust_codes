fn main() {
    // Para crear una variable se usa let
    // Si no se especifica el compilador lo infiere basado en su valor y como lo usamos
    // Hay funciones donde es necesario especificar el tipo ya que puede devolver multiples tipos
    // Ese es el caso de la función parse()
    // Por defecto las variables son inmutables, así que no se le puede cambiar de valor
    let x = 5;
    println!("The value of x is: {x}");

    // Para que una variable pueda cambiar de valor hay que especificar que es mutable con mut
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The new value of y is: {y}");

    // Para declarar constantes se usa const, y siempre hay que especificar el tipo
    // El tipo se especifica luego de usar : luego del nombre
    // Por convención los const se escriben en mayusculas con _ separando palabras
    // Al ser constante su resultado no puede depender de otra variable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // El shadowing es la capacidad de nombrar una nueva variable igual que otra anterior
    // Modificando su valor y tambien puede cambiar su tipo
    // El shadowing se mantiene en su alcance como se muestra a continuación
    let x = x + 1; // x pasa a valer 6
    {
        let x = x * 2; // x pasa a valer 12
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}"); // x vuelve a valer 6

    // Acá hay un ejemplo de cambio de tipo, de string a entero
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces are: {spaces}");

    // Tipos escalares de variables:
    
    // Enteros (integer): se designan con u (unsigned) o i (signed) más el numero de bits
    // u8, i8, u16, i16, u32, i32, u64, i64, u128, i128.
    // Tambien está usize e isize que depende del sistema, 32bits si el OS es de 32bits
    // Para facilitar la lectura de estos numeros se puede usar 1_000 para escribir 1000
    // Tambien se puede poner 53u8 para indicar que es de 8bits
    // Ej: 98_222 (98200); 0xFF (Hexa); 0o77 (Octal); 0b1111_0000 (Binario); b'A' (Byte solo para u8)
    // En caso de overflow en debug (run normal) la ejecución se detendrá y lo indicará
    // En modo release el programa por defecto dará una "vuelta", si es u8 y llega a 256 pasrá a valer 1, 257 vale 2, etc
    // Para manejar esta posibilidad de overflow existen metodos: wrapping_*; checked_*; overflowing_*; saturating_*;

    // Decimales (float): pueden ser f32 o f64, por defecto se toman f64 a no ser que se especifique f32

    // Booleano: solo puede tener de valor true o false, se puede especificar con : bool

    // Caracter (char): son de un solo caracter, son de 4 bytes y pueden tomar acentos, chino, japoner, emojis, etc.

    // Tipos compuestos de variables:

    // Tupla (tuple): permite agrupar valores numericos de diferentes tipos, se puede especificar tipo con : o no.
    // Las tuplas tienen una longitud fija, una vez creadas no pueden expandirse.
    let tup: (i32, f32, u8) = (500, 3.4, 3);
    // Para obtener los valores de cada elemento de la tupla se puede hacer lo siguiente:
    // Si una variable no se usa se puede poner un _ antes del nombre.
    let (x, y, _z) = tup;
    println!("The value of y is: {y}");
    println!("The value of x is: {x} = {}", tup.0);
    let z = tup.2;
    println!("The value of z is: {z}");

    // Array: continene multiples valores numericos del mismo tipo.
    // Una vez creado el vector no puede expandirse.
    let a = [0, 1, 2];
    let _fist = a[0];
    let _second = a[1];
    println!("The third value of a is: {}", a[2]);
    let _months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let _b: [i32; 5] = [0, 1, 2, 3, 4];
    let _c = [3; 5]; // c = [3, 3, 3, 3, 3];
    // Si tratamos de acceder a la posición 20 de cualquiera de los arrays creados al ingresar por teclado su posición
    // el programa hará un panic y crasheará el programa indicando que está fuera del limite de memoria.
    // En C nos daría los datos de basura que se encuentren en la posición de memoria indicada.

    // Todos los tipos vistos se guardan en el stack de memoria, por eso no pueden crecer
    // Está el tipo Vector que si puede crecer y está alocado en el heap de memoria.
}
