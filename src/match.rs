fn main() {
    let number = 13;

    println!("Tell me about {}", number);

    match number {
        // Match a single value
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime added by 13: {:?}", n + 13),
        13..=19 => println!("A teen!"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
