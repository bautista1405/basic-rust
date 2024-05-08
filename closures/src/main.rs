fn main() {
    // Previously, we looked at named functions and we looked at ownership. In Rust, we also have closures, which give us the ability to make
    // anonymous functions.

    // You can make a closure with explicit or inferred types:
    let y: u32 = 10;
    let annotated = |x: u32| -> u32 { x + y };
    let inferred = |x| x + y;

    println!("annotated: {}", annotated(32));
    println!("inferred: {}", inferred(32));

    // The basic syntax for a closure is to use pipes around the parameter list followed by an expression for the return value. Sometimes
    // you'll see a no-argument closure, which looks like it's using the or-operator (||) but that's the empty parameter list here, and could
    // be written with spaces for clarity (| |).

    // Closures can reference values from their outer scope, which is really handy. They can also capture the outer values and use them. This
    // is handy for things like counters:
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };

    println!("count is {}", increment());
    println!("count is {}", increment());
    println!("count is {}", increment());

    // Note that the closure must be mut if it captures a mutable variable. This is because what it captures is part of the closure, so if
    // it's mutating it then it's mutating itself.

    let addTwo = |x: u32, z: u32| -> u32 { x + z };
    println!("addTwo = {}", addTwo(10, 30));

    fn make_counter() -> impl FnMut() -> u32 {
        let mut count = 0;
        let increment = move || {
            count += 1;
            count
        };
        increment
    }

    let mut counter = make_counter();

    println!("count is {}", counter()); // prints 1
    println!("count is {}", counter()); // prints 2
    println!("count is {}", counter()); // prints 3

    // You'll notice the return types for this last functions is different. What it returns is an impl of a trait. We'll get to what traits
    // are in a later section; for now, you can think of them like interfaces, so we know what we can do with the thing, but not its specific
    // type.

    // There are three traits for functions: Fn, FnMut, and FnOnce, which provide various restrictions on how the caller of the function can
    // use it. The other thing you'll notice is the impl keyword, which is new. This says we'll return something which implements this trait
    // (like an interface in other languages), but we don't specify exactly what it is. This is how you return closures, generally, because
    // each closure is its own type.

    // - Fn can be called be called multiple times, and it doesn't modify its underlying state.
    // - FnMut can be called multiple times, but it may mutate itself when you do (so it needs a mutable reference to itself)
    // - FnOnce can be called once. It consumes itself in that call, and you can't use it a second time.
}

//https://yet-another-rust-resource.pages.dev/closures
