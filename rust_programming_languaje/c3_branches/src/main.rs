fn main() {
    let number = 4;

    if number < 4 {
        println!("Less than 4");
    } else if number == 4 {
        println!("Equal to 4");
    } else {
        println!("More than 4");
    }

    let condition = false;
    // Como el if es una expresion se puede usar para asignar valores
    let number = if condition { 5 } else { 6 }; // Los dos valores deben ser iguales
    println!("The number is: {number}");
}
