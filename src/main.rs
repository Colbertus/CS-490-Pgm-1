use std::io; 

fn main() {

    println!("Welcome to the cone volume calculator!");
    
    loop {
        println!("Enter a radius in centimeters (or quit to exit):");

        let mut radius = String::new();
        let mut height = String::new();
    
        io::stdin()
            .read_line(&mut radius)
            .expect("Failed to read line");

        let radius: f32 = match radius.trim().parse() {
            Ok(num) => num,
            Err(_) => match radius.trim() {
                "quit" => break,
                _ => {
                    println!("Invalid measurement. Please try again.\n");
                    continue;
                },
            },
        };

        if radius <= 0.0 {
            println!("Invalid measurement. Please try again.\n");
            continue;
        }

        println!("Enter a height in centimeters (or quit to exit):");

        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read line");

        let height: f32 = match height.trim().parse() {
            Ok(num) => num,
            Err(_) => match height.trim() {
                "quit" => break,
                _ => {
                    println!("Invalid measurement. Please try again.\n");
                    continue;
                },
            },
        };

        if height <= 0.0 {
            println!("\nInvalid measurement. Please try again.\n");
            continue;
        }

        println!("The volume of the cone is: {} milliliters\n", compute_cone_volume(radius, height));

    }

    println!("\nGoodbye!");
}

fn compute_cone_volume(radius: f32, height: f32) -> f32 {
    const PI: f32 = 3.14159;
    let volume = PI * radius * radius * height / 3.0;
    volume
}
