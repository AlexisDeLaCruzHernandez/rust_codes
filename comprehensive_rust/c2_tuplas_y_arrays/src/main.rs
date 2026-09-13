fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut t_matrix = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            t_matrix[j][i] = matrix[i][j];
        }
    }
    t_matrix
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], 
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], 
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    // Cada nuevo array se considera un tipo nuevo, en este caso [i8; 10] contiene 10 elementos del mismo tipo i8
    // Dos arrays [u8; 3] y [u8; 4] se consideran tipos distintos
    let mut a: [i8; 10] = [42; 10]; // Al iniciar de esta forma el array se guarda como [42, 42, 42, 42, 42, 42, 42, 42, 42, 42]
    a[5] = 0; // Asigna el valor 0 a la posición 5 (desde la posición 0)
    println!("a: {a:?}"); // El modificados :? es para hacer un print de depuración, ya que con {} simple no se pueden visualizar arrays
    // println!("a beauty: {a:#?}"); // Con el # se imprime de una forma de más fácil lectura
    // println!("Fuera de rango: {}", a[10]); -> Error de compilación

    // Las tuplas pueden agrupar tipos distintos
    let t: (i8, bool) = (7, true);
    println!("Primer valor: {}", t.0); // Para acceder a los campos se hace con "." y el indice del valor
    println!("Segundo valor: {}", t.1);
    // La tupla vacia "()" se llama "tipo de unidad" y significa ausencia de un valor de retorno (como el void)

    // Con for podemos iterar un array pero no tuplas
    let numeros_primos = [2, 3, 5, 7, 11, 13, 17, 19];
    for primos in numeros_primos { // primos pasará por los valores del array de numeros primos -> 2, 3, 5, 7...
        for i in 2..primos { // Luego i pasará desde 2 hasta primos - 1 (no inclusivo), verificando así que los numeros sean primos
            println!("{primos}: {i}");
            assert_ne!(primos % i, 0); // Macro que verifica que las dos expresiones no sean iguales
            // Si en numeros_primos estaría el numero 4 sucederia esto:
            /*
            numeros_primos = 2 -> no entrará en el segundo for porque ya cumple la limitación del bucle
            numeros_primos = 3 -> entra al bucle i = 2 -> 3 % 2 = 1 != 0 -> Verifica la inegualdad
            numeros_primos = 4 -> i = 2 -> 4 % 2 = 0 == 0 -> No verifica -> Resulta en error
            */
        }
    }

    // Hay dos formas de extrar los valores de las tuplas a variables locales
    let tupla = (8, true);
    let _izquierda = tupla.0; // Usando los indices 0
    let _derecha = tupla.1; // y 1
    let (_izquierda, _derecha) = tupla; // Haciendo coincidir el patrón para desestructurarlo
    // El patrón debe coincidir perfectamente para no tener errores de compilación

    let matrix = [ // Es una matriz 3x3, o un array de arrays
        [101, 102, 103], 
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matriz: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("traspuesto: {:#?}", transposed);
}
