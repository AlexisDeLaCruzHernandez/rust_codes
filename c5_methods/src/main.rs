struct Rectangle {
    width: u32,
    height: u32,
}

// Los metodos son como funciones pero asociadas a estructuras (tambien otros tipos más adelante)
// Pueden o no retornar valores y como parametros siempre primero tiene self
// Se pueden poner dos bloques impl y el tipo de dato tendrá todos los metodos enumerados
impl Rectangle { // Todo lo que este dentro de este impl queda relacionado al tipo Rectangle
    fn area(&self) -> u32 { //&self es una abreviacion de self: &self, y toma el tipo dado en impl
        // Si queremos cambiar algun parametro se tendria que realizar &mut self
        self.width * self.height
    }

    fn width(&self) -> bool { // Tambien se puede llamar a un metodo igual que un campo
        self.width > 0
    }

    fn height(&self) -> u32 { // Generalmente se utiliza para retornar el valor del campo
        self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool { // Podemos llamar a otro rectangulo
        self.width > rectangle.width && self.height > rectangle.height // Con && es como en c
    }

    // Los metodos son funciones asociadas pero hay funciones asociadas que no son metodos
    // Estos no toman como primer parametro self y se suelen utilizar como constructores
    // Se los puede llamar como se quiera, normalmente new pero no es obligatorio
    fn square(size: u32) -> Self { // Al retornar se pone Self con S mayuscula para referir a Rectangle
        Self { 
            width: size, 
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 10,
    };
    println!("El area es: {}", rect1.area());
    if rect1.width() {
        println!("El ancho es mayor a 0: {}", rect1.width);
    }
    println!("El alto es: {}", rect1.height());

    // Se busca ver el funcionamiento del metodo can_hold
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Para llamar a las funciones asociadas que no son metodos se usan los ::
    let _square = Rectangle::square(3);
}
