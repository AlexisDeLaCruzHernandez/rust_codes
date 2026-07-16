// Para definir se utiliza la palabra struct junto a su nombre de tipo
struct User {
    active: bool, // Se debe especificar el nombre del campo como así su tipo
    username: String,
    email: String,
    sign_in_count: u64,
}

// Se pueden definir estructuras como tuplas
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Podemos armar una funcion para llenar los campos de la estructura
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // Podemos asignar de esta forma
        email, // O de esta forma siempre que el parametro tenga el mismo nombre
        sign_in_count: 1,
    }
}

fn main() {
    // Para usar la estructura se crea una instancia, y se inician los valores usando: campo: valor,
    let _user1 = User {
        active: true,
        username: String::from("Alexis"),
        email: String::from("ejemplo@gmail.com"),
        sign_in_count: 1,
    };
    let _example = build_user(String::from("email@gmail.com"), String::from("ejemplo"));
    // Para poder modificar un campo toda la estructura debe ser mutable
    let mut user1 = User {
        active: true,
        username: String::from("Alexis"),
        email: String::from("ejemplo@gmail.com"),
        sign_in_count: 1,
    };
    // Para modificar un campo se hace indicando el campo con un punto
    user1.active = false;
    user1.username = String::from("Hernandez");
    user1.email = String::from("ejemplo2@gmail.com");
    user1.sign_in_count += 1;
    // Si queremos crear un user2 solo cambiando el username podemos hacer lo siguiente
    let _user2 = User {
        username: String::from("Alexis"),
        ..user1 // Pone los datos restantes de user 1
        // En este caso especifico como estamos copiando un String (email) user1 ya no será
        // usable porque mueve el string a user2
    };
    // Las estructuras como tuplas son todas de tipos diferentes aunque tengan los mismos tipos
    let black = Color(0, 0, 0);
    let point = Point(0, 1, 2);
    // No podemos igualar un Color con un Point aunque todos los datos sean i32
    // Podemos obtener los datos desestructurando como las tuplas
    let Point(x, y, z) = point;
    println!("X: {x}, Y: {y}, Z: {z}");
    let Color(r, g, b) = black;
    println!("r: {r}, g: {g}, b: {b}");
}
