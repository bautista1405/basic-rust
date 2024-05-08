fn main() {
    // The most basic, fundamental heap allocation is using Box.
    // A boxed value is simply one that lives on the heap instead of the stack. A Box is a boxed value, and it's generic: a Box<u32> is a u32
    // that's heap-allocated, and a Box<f64> is a 64-bit float that's heap-allocated.
    // To construct one, you use the Box::new constructor. Often the compiler can infer what type the Box has, but if not, you have to provide
    // type annotations.
    let x: Box<u32> = Box::new(42);
    let y = Box::<f64>::new(4.2);

    //To move a value back onto the stack, out of the box, you can dereference it
    let x: Box<u32> = Box::new(42);
    let y = *x; // y is now a u32 on the stack

    //Sometimes you just need to build a list of things. You do this with Vec, which is the type for dynamically sized arrays in Rust.
    //(Rust's arrays are of fixed size.)
    let mut parrots: Vec<&str> = Vec::new();
    parrots.push("Shivers");
    parrots.push("Tweety");
    parrots.push("Dinner");
    println!("parrots: {:#?}", parrots); //{:#?} is for a "pretty" print, otherwise {:?} will do it. note that {} will throw an error

    //You can also do it inline, using the vec! macro:
    let parrots = vec!["Shivers", "Tweety", "Dinner"];
    println!("parrots: {:?}", parrots);

    //You can also iterate over the elements of a Vec:
    let parrots = vec!["Shivers", "Tweety", "Dinner"];
    for parrot in parrots.iter() {
        println!("{} says hi.", parrot);
    }
}

//https://yet-another-rust-resource.pages.dev/heap-allocation
