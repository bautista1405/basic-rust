fn main() {
    // First create a variable, x
    let x: u32 = 10;

    // Then create a reference to it:
    let ref_x: &u32 = &x;

    // Then print out the reference!
    println!("x = {}", *ref_x);

    //the * is to dereference, but Rust can do it automatically so it's not really needed
}

/*

1. In your own words, what is the difference between allocating on the heap and on the stack?

    Heap: more work to access it since it lives on as long as we define it. we have to deliberately allocate and deallocate resources here,
    leading us to deal manually with memory allocation, specifically using a type that indicates so

    Stack: easier to access since resources live on as long as the function call frame is on the stack, until it gets popped off when the
    the function ends. also Rust does this for us automatically, just declare inside functions

2. In your own words, what is the difference between a reference and a pointer?

    References and pointers are very alike. They reference or point to a specific resource in memory. But each do so in their own way.
    References "make reference" to a variable in memory, while pointers "point" to an address in memory. Also pointers are not often used
    in Rust since we have to use the language in "unsafe" mode, claiming that their use could come with some hazard.

*/
