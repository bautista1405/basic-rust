fn main() {
    let msg = "Hello, world";

    //declare a block (piece of code). variables inside have a local scope, meaning they live only inside the braces
    // Blocks return a value, which is precisely the value of the last expression in the block. If the last expression of the block ends
    // with a semicolon, then it's a statement, which returns (), the unit value, so it functionally has no return value.
    {
        // we're in the pirate block now
        let msg = "Ahoy, matey!";
        println!("{msg}");
    }
    println!("{msg}");

    //here's an example of a block returning a value (30)
    let parrots = 5;
    let shipmates = 10;

    let legs_on_ship = {
        let parrot_legs = 2 * parrots;
        let human_legs = 2 * shipmates;
        parrot_legs + human_legs
    };
    println!("{legs_on_ship}");

    // //for loop
    let count: i8 = 0;
    for count in 0..10 {
        println!("iteration {}", count + 1);
    }

    // // This behaves the same as above. 0..10 gives us the range from 0 to 10, excluding the 10. That means we have to do the awkward +1 to
    // // get count the same. We can specify the range as an inclusive range instead to avoid that, using 1..=10, which ranges from 1 to 10,
    // // including the 10.
    for count in 1..=10 {
        println!("iteration {count}");
    }

    let mut n = 0;

    // fizz buzz while loop
    while n < 10 {
        if n % 3 == 0 {
            println!("fizz");
        }
        if n % 5 == 0 {
            println!("buzz");
        }
        n += 1;
    }

    //fizz buzz for loop
    for mut n in 0..=10 {
        if n % 3 == 0 {
            println!("fizz");
        }
        if n % 5 == 0 {
            println!("buzz");
        }
        n += 1;
    }
}

//since Rust is a expression-oriented language, everything returns a value. we'll delve into how arguments can be formatted, printed and
//returned as strings
//https://doc.rust-lang.org/std/fmt/
//https://yet-another-rust-resource.pages.dev/control-flow
