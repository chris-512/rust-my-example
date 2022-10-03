fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be use to bind the member of a tuple to variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// The following struct is for the activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    println!("{} days", 31);

    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    println!("{} integer", an_integer);

    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Variables can be overwritten by shadowing.
    let mutable = true;

    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinte loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 3 {
            println!("OK, that's enough!");
            break;
        }
    }

    // A tuple with a bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f32, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));
}
