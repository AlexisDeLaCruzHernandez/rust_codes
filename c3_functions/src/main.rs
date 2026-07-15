fn main() {
    println!("Hello, world!");

    another_function();

    print_number(100_i32);

    print_value_with_unit(10.0, 'A');

    let y = {
        let x = 4;
        x + 1 // Como no tiene ; es un retorno de valor hacia y
    };
    print_number(y);

    let x = asign_value(32);
    print_number(x);
}

// Existen statements (instrucciones que hacen una acción y no tienen retorno)
// y expressions (tienen un retorno)

// Las funciones pueden definirse antes o despues del main
fn another_function() {
    println!("Another function");
}

// Al definir parametros es necesario indicar el tipo de dato que va a enviar
fn print_number(x: i32) {
    println!("The number is: {x}");
}

// Para multiples parametros se separan con comas
fn print_value_with_unit(value: f32, unit: char) {
    println!("The value is: {value}{unit}");
}

// Para indicar que valor se retorna se utiliza -> tipo de variable
fn asign_value(value: i32) -> i32 {
    print_number(value);
    value // En el final de la funcion no hace falta usar el return
    // La linea sin ; indica la expresión o cosa que devuelve
    // Se puede utilizar return value; tambien
}