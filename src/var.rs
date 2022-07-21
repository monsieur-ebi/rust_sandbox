// Variables hold primitive data or refrences to data
// Variables are immutable by default
// Rust is a block-scoped language


pub fn run() {  
    
    let name = "Zimor";
    let mut age = 37;
    println!("My name is {} and i am {}", name, age);
    age = 38;
    println!("My name is {}", name);
    println!("My name is {} and i am {}", name, age);

    // Define constants
    const ID: i32 = 001;
    println!("ID {}", ID);

    // Assign multiple vars
    let ( my_name, my_age ) = ("brad", 37);
    println!("{} is {}", my_name, my_age);

}

