fn main() {
    // El if funciona como en C:
    let x = 10;
    if x == 0 {
        println!("x es cero");
    } else if x < 100 {
        println!("x es menor que 100");
    } else {
        println!("x es mayor o igual que 100");
    }
    // Tambien se puede usar if como expresión, retornando un valor
    let size = if x < 100 { "pequeño" } else { "grande" }; // Ambos tipos de retorno deben ser iguales
    println!("El tamaño de x es: {size}");

    // While tambien es como en C:
    let mut x = 100;
    while x <= 10 {
        x = x / 2;
        println!("x es: {x}");
    }

    // El bucle for es un bucle que itera sobre un rango de valores
    for i in 0..10 { // El rango es inclusivo en el inicio y exclusivo en el final, llega hasta 9
        println!("i es: {i}");
    }
    // En caso de querer que el rango sea inclusivo en ambos extremos, se puede usar "..=" en lugar de ".." (for i in 0..=10) 
    // Tambien se puede iterar sobre un array o vector
    for j in [1, 2, 3, 4, 5] {
        println!("j es: {j}");
    }
    
    // Por ultimo el loop es un bucle infinito, que se puede romper con la palabra clave "break"
    let mut k = 0;
    loop {
        k += 1;
        println!("k es: {k}");
        if k >= 10 {
            break; // Rompe el bucle cuando i es mayor o igual a 10
        }
    }
    // Tambien se puede usar "continue" para saltar a la siguiente iteración del bucle
    let mut l = 0;
    loop {
        l += 1;
        if l > 5 {
            break; // Al llegar a 6 rompe el bucle
        }
        if l % 2 == 0 {
            continue; // Si es par no lo imprime y pasa a la siguiente iteración
        }
        println!("{}", l);
    }

    // Se pueden poner etiquetas a los bucles para poder romperlos desde dentro de otro bucle
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]]; // Matriz de 3x3
    let mut elements_searched = 0; // Contador de elementos buscados
    let target_value = 10; // Valor a buscar
    'outer: for i in 0..=2 { // Bucle externo con etiqueta 'outer para iterar sobre las filas de la matriz
        for j in 0..=2 { // Bucle interno para iterar sobre las columnas de la matriz
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer; // Rompe el bucle externo si se encuentra el valor
            }
        }
    }
    println!("elementos buscados: {elements_searched}");

    // Los bloques en Rust están delimitados por llaves y pueden contener declaraciones y expresiones
    // El ambito de las variables declaradas dentro de un bloque es local a ese bloque, y no se puede acceder a ellas desde fuera
    let z = 13;
    let x = {
        let y = 10;
        z + y // La última expresión del bloque es el valor que se retorna, no se pone punto y coma
        // En caso de poner punto y coma, el bloque retornará el valor unitario "()" y no se podrá asignar a la variable
        // Si se utiliza "return" dentro del bloque, se retornará el valor especificado y no se ejecutará el resto del bloque
    };
    println!("El valor de x es: {x}");
    // Demostramos el ambito de variables y el shadowing, que permite declarar una variable con el mismo nombre que otra en un ambito diferente
    let a = 5;
    println!("Antes: {a}");
    {
        let a = 10; // Esta variable "a" es diferente a la del ambito exterior, y solo existe dentro de este bloque
        println!("Dentro del bloque: {a}");

        let a = true; // El shadowing permite declarar una variable con el mismo nombre y un tipo diferente, pero solo dentro del ambito del bloque
        println!("Shadowing dentro del bloque: {a}");
    }
    println!("El valor de a fuera del bloque es: {a}");
    println!("5! = {}", factorial(5)); // 5! = 120

    let n = 11;
    println!("Longitud de la secuencia de Collatz para n = {} es: {}", n, collatz_length(n));
}

fn factorial(n: u32) -> u32 {
    let mut producto = 1;
    for i in 1..=n {
        producto *= dbg!(i);
    }
    producto
}

// Determina la longitud de la secuencia de Collatz que empieza por `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut i = 1;
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        }
        else {
            n = 3 * n + 1;
        }
        // Otra forma más compacta de hacerlo es:
        // n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        i += 1;
    }
    i
}