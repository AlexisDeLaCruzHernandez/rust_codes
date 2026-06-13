fn main() {
    let temperature = 24.0;
    println!("Celsius temperature: {temperature}°C");
    let temperature = celsius_to_fahrenheit(temperature);
    println!("Fahrenheit temperature: {temperature}°F");

    let fibonacci = fibonacci_number(8);
    println!("fibonacci number: {fibonacci}");

    print_the_twelve_days_of_christmas();
}

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    (temperature * 1.8) + 32.0
}

fn fibonacci_number(number: u32) -> u32 {
    if number <= 1 {
        return number;
    } else {
        return fibonacci_number(number - 1) + fibonacci_number(number - 2);
    }
}

fn print_the_twelve_days_of_christmas() {
    let ordinal_numbers = [
        "first", "second", "third", "fourth", 
        "fifth", "sixth", "seventh", "eighth", 
        "ninth", "tenth", "eleventh", "twelfth"
    ];
    for i in 0..12 {
        let mut number = i;
        println!("On the {} day of christmas, my true love sent to me", ordinal_numbers[i]);
        while number != 0 {
            match number {
                1  => println!("Two turtle doves and"),
                2  => println!("Three french hens"),
                3  => println!("Four calling birds"),
                4  => println!("Five golden rings"),
                5  => println!("Six geese a-laying"),
                6  => println!("Seven swans a-swimming"),
                7  => println!("Eight maids a-milking"),
                8  => println!("Nine ladies dancing"),
                9  => println!("Ten lords a-leaping"),
                10 => println!("Eleven pipers piping"),
                11 => println!("Twelve drummers drumming"),
                _  => {}, // En caso que que no sea ninguno no hacer nada
            };
            number -= 1;
        }
        println!("A partridge in a pear tree\n");
    }
}
