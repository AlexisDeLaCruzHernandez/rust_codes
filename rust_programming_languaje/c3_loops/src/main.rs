fn main() {
    // Loop infinito
    // Se detiene al usar break;
    // Si se utiliza continue; vuele al principio del loop
    let mut i = 1;
    loop {
        if i == 5 {
            break; // Detiene en la iteración 5
        } else if i == 3 {
            i += 1;
            continue; // Se saltea el print de la iteracion 3
        }
        println!("The iteration is: {i}");
        i += 1;
    }

    i = 0;
    // Tambien se puede usar el loop para retornar valores con break
    // Se puede usar return pero sale de todos los loops, break solo sale del loop actual
    let result = loop {
        i += 1;
        if i == 10 {
            break i * 2;
        }
    };
    println!("The result is: {result}");

    let mut count = 0;
    // Se le pueden poner etiquetas a los loops para hacer break de un loop mayor
    // Estas etiquetas empiezan con un '
    'counting_up: loop {
        println!("Count: {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count: {count}");

    let mut number = 3;
    // Se usa el while para verificar si seguir el loop, cuando no se cumple la condición hace break
    while number != 0 {
        println!("Number: {number}");
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    // Se puede usar un for para moverse por los valores de un array
    for element in a {
        println!("The value is: {element}");
    }
    // Otra forma es armar la lista de numeros directamente en el for para determinar cuantos ciclos hacer
    for number in (1..=3).rev() { // El metodo rev invierte el rango dado, de 1, 2, 3 a 3, 2, 1
        println!("{number}");
    }
}
