// Importar el modulo fmt
use std::fmt;

// Definir estructura, se agrega el Debug para visualizar la diferencia
#[derive(Debug)]
struct MinMax(i64, i64);

// Se implementa 'fmt::Display' para poder imprimir usando {}
// Si se quiere imprimir en binario tambien se debe implementar fmt::Binary
impl fmt::Display for MinMax {
    // Se requiere si o si la siguiente estructura de función
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // En f se guarda el string
        // Devuelve exito o error
        // Con `self.number` nos podemos referir a cada campo de la estructura
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Se define otra estructura pero con nombres en los campos
#[derive(Debug)]
struct Point2D {
    real: f64,
    imag: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{} {:+}i", self.real, self.imag) Con el :+ se quiere mostrar el signo del argumento
        let sign = if self.imag.is_sign_negative() { "-" } else { "+" };
        write!(f, "{} {} {}", self.real, sign, self.imag.abs())
    }
}

pub fn print_second_part() {
    // Para poder mostrar tipos nuevos creados se puede utilizar el siguiente atributo
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Number(i32); // Solo contiene un entero de 32bits con signo
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Deep(Number);

    // Para imprimir en debug se añade un ':?' entre los {}
    println!("Months: {:?}", 12);
    println!("Number: {:?}", Number(3));
    println!("Deep: {:?}", Deep(Number(7))); // Para que esto se vea mejor hay otro modificador
    println!("Deep: {:#?}", Deep(Number(7)));

    let minmax = MinMax(0, 14);
    println!("\nCompare prints:");
    println!("Display: {}", minmax); // Imprime con el formato dado
    println!("Debug: {:?}", minmax); // Imprime con debug

    let big_range = MinMax(-20, 456);
    let small_range = MinMax(0, 10);
    println!("Big range: {big}; Small range: {small}", small = small_range, big = big_range);

    let point = Point2D { real: 3.3, imag: 7.2 };
    println!("\nCompare prints:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
} 
