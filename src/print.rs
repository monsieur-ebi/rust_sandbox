pub fn run() {
    // Basic Formatting
    println!("This print is coming from the print file");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}", 
        "Ebizimor", "Nigeria", "Code"
    );

    // Named Arguments
    print!(
        "{name} likes to play {activity}", 
        name = "John", 
        activity = "Baseball" 
    );


    // Placeholder Traits
    print!(
        "Binary: {:b} Hex: {:x} Octal {:o}",
        10, 10, 10
    );

    // Placeholder for debug Trait
    print!("{:?}", (1, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
    
}