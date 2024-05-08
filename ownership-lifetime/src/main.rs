fn main() {
    // As we went over earlier, Rust does not have a garbage collector. Instead, the compiler is tracking when memory should be allocated and deallocated, and ensuring that your references remain valid.

    // How it does that is by keeping track of the lifetime of variables as well as their ownership. To unpack those, in short:

    // A value has one owner at a time, and ownership is used for tracking when memory is valid and when it is dropped.
    // The lifetime of a variable is the time during which references to it are valid.
    // These two concepts are highly related. The lifetime of a reference is linked to the lifetime of its owner.

    //outer scope
    // define a reference to a u32, but do not initialize it
    let outer_ref: &u32;

    {
        // create an inner scope

        // declare and initialize a u32
        let inner_val: u32 = 10;

        // try to assign a reference to the inner val to our ref
        outer_ref = &inner_val; // <-- this line will fail to compile.
    } // <-- inner_val goes out of scope here

    // <-- but the compiler needs outer_ref to be valid here!
    println!("outer_ref value: {}", outer_ref);

    //we can fix it with: `let inner_val: &'static u32 = &10;`
    //e: 'static is a lifetime notation meaning "referred to data will live for the duration of the program", in this case, it
    //will live during our whole main function execution. note that we heavily implement references and not actual values with `&`

    // This might be your first interaction with the borrow checker. It's a vital piece of Rust machinery which helps prevent major issues.
    // Notably, you could get both of the previous examples to compile in languages like C or C++, leading to use-after-free errors. With
    // Rust, you can't do that

    // Ownership is related to lifetimes, and we can see a few examples of it. Each variable has an owner, and this ownership can be moved to
    // another place. This happens if you pass something by value: the new place receives the value and promises that it will deallocate it
    // when it needs to. But since the new place owns it, the old place is not allowed to use it anymore!

    // Here's one example that shows a value moving to a new owner, one that new Rust programmers often encounter:
    let xs = vec![1, 2, 3];

    for x in xs {
        println!("{x}");
    }

    println!("total len: {}", xs.len());

    // This looks totally reasonable, but there's a problem: When we iterated over xs, since we used the value of xs, we moved it, and we don't
    // own it anymore! This example does not compile.

    // Instead, we need to use a reference to let the for loop borrow the Vec, and then we can use it in both places:
    let xs = vec![1, 2, 3];

    for x in &xs {
        println!("{x}");
    }

    println!("total len: {}", xs.len());

    // That's the basics of ownership. Just remember that when you pass a value around, it moves the value to the other place.

    // Well, that's not true. It moves it if it can't copy it.

    // There are some variables, the simplest ones, which "are Copy", which means they implement the Copy trait. If something is Copy, then it
    // will get copied instead of moved, and you can keep using it.

    // As an example, we can use a primitive u32 twice:
    let x: u32 = 10;
    println!("x is {x}");
    println!("x*2 is {}", x * 2);
}

//https://yet-another-rust-resource.pages.dev/ownership
