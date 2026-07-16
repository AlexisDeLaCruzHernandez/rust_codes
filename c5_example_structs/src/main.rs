// Vamos a hacer códigos de ejemplo para diferentes casos para calcular areas de rectangulos

// Sin usar ni tuplas ni estructuras
/*
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("El área del rectangulo es: {}", area(width1, height1));
}
*/

// Usando tuplas
/*
fn area(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}

fn main() {
    let rect1 = (30, 50);
    println!("El área del rectangulo es: {}", area(rect1));
}
*/

// Usando estructuras
// Para poder imprimir estructuras para hacer un debug devemos incluir la siguiente linea
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Si no lo pasamos por referencia le damos la propiedad a rectangle 
// y no podremos volver a utilizar la función
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("El área del rectangulo es: {}", area(&rect1));

    let _rect_area = dbg!(area(&rect1)); // Usando la macro dbg! imprime por consola el resultado

    println!("Width: {}", rect1.height);

    // Para imprimir la estructura en debug tenemos las siguientes opciones
    println!("rect1: {rect1:?}"); // Lo imprime todo en una linea 
    println!("rect1: {rect1:#?}"); // Lo imprime en lineas separadas
}
