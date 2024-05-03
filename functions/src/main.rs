fn main() {
    let x = fibonacci(10);
    println!("{x}"); //prints 55
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

// 1. In the main function, you call fibonacci(10).
// 2. Inside the fibonacci function, when n is 10, it's not less than 2, so the function proceeds to calculate fibonacci(9) and fibonacci(8).
// 3. fibonacci(9) then calculates fibonacci(8) and fibonacci(7), and so on, until it reaches the base cases where n is less than 2.
// 4. Once it reaches the base cases (where n is 0 or 1), it starts returning values back up the call stack, adding them together as it goes.
// 5. Eventually, it returns fibonacci(10) as the sum of fibonacci(9) and fibonacci(8), which recursively sums up all the Fibonacci numbers
//leading up to 10.

// So, when you call fibonacci(10), it calculates the 10th Fibonacci number, which is 55, and that's why your program prints 55.
