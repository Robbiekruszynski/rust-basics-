mod arrays;
mod loops;
mod print;
mod strings;
mod tuples;
mod types;
mod vars;
mod vectors;
fn main() {
    print::run();
    vars::run();
    types::run();
    loops::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    //placeholder print
    println!("Number: {} ", 1);

    //basic formatting
    println!("{} is from {}", "Robbie", "Chicago");

    //positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Robbie", "Chicago", "bang"
    );

    //named arguments
    println!(
        "{name} likes to {act}",
        name = "Robbie",
        act = "eact cheese"
    );

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
}
