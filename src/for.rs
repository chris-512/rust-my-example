fn main() {
    println!("[1] iter() example");

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    println!("[2] into_iter() example");
    let names1 = vec!["Bob", "Frank", "Ferris"];

    for (i, name) in names1.into_iter().enumerate() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names1);

    println!("[3] iter_mut() example");
    let mut names2 = vec!["Bob", "Frank", "Ferris"];

    for name in names2.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names2);
}
