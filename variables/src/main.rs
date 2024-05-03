//here to redefine a variable's value (x) we gotta use the keyword "mut" to let Rust know that is mutable, otherwise we'll get an error
//also note that to print a variable we gotta use "{}" to specify that is a string literal
fn main() {
    let mut x = 10;
    println!("{}", x);
    x = 11;
    println!("{}", x);

    let name: &str = "bautista"; //string literal
    let age: u32 = 35; //32 bit unsigned integer
    println!("{}", name);
    println!("{}", age);

    let x: i64 = -13;
    let y: u8 = 10;
    let z = x + (y as i64); //here we cast 'y' to 'i64' to be able to add with 'x'
    println!("{}", z);

    //we define three integers of different types to multiply them
    let a: i8 = 5;
    let b: i16 = 20;
    let c: i32 = 100;

    //two ways of reaching to the same result (nominally, 10.000)
    let result1: i32 = (a as i32) * (b as i32) * c; //here we convert 'a' and 'b' to i32
    let result2: i128 = ((a as i32) * (b as i32) * c).into(); //here we use the method into() to automatically transform to the previously defined type (i128)

    println!("{}", result1);
    println!("{}", result2);
}

/*
The relation between the mut keyword and the concept of mutability is apparent in its name and in its usage: it allows you to mutate things.

However, this hides another way of thinking about it which is very helpful in the context of references later on. How do you achieve
mutability? By enforcing exclusive access: when there is one reference to a mutable variable, there can be no others (but you can use
that one multiple times). So a mut reference is also an exclusive reference.
*/

//Primitive Types -> https://doc.rust-lang.org/std/primitive/index.html
//https://yet-another-rust-resource.pages.dev/variables

/*
unsigned integers: u8, u16, u32 u64, u128, usize
signed integers: i8, i16, i32, i64, i128, isize
characters
strings
booleans
unit
*/
