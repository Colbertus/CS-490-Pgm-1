// Author: Colby McClure
// CS 490 Pgm 1 
// Date: 9/14/2024

use std::io; 

/*
    Function name: 
        main

    Description: 
        This function is the entry point of the program. It prompts the user to enter a radius and height to calculate the volume of a cone.
        This function primarily runs on a loop that will continue to prompt the user for input until the user types "quit".
*/
fn main() {

    // Welcome message
    println!("Welcome to the cone volume calculator!");

    // Loop used keep computing volumes until the user types "quit"
    loop {

        println!("Enter a radius in centimeters (or quit to exit):");

        // Initialize mutable variables to store user input
        let mut radius = String::new();
        let mut height = String::new();
        
        // Read user input
        io::stdin()
            .read_line(&mut radius)
            .expect("Failed to read line");
        
        // Parse and validate user input
        let radius: f32 = match radius.trim().parse() {
            Ok(num) => num,

            // For the catch all case, we check if the user typed "quit" to exit the program
            Err(_) => match radius.trim() {
                "quit" => break,
                _ => {
                    println!("Invalid measurement. Please try again.\n");
                    continue;
                },
            },
        };
        
        // If the radius is less than or equal to 0, we print an error message and prompt the user to try again
        if radius <= 0.0 {
            println!("Invalid measurement. Please try again.\n");
            continue;
        }

        println!("Enter a height in centimeters (or quit to exit):");
        
        // Read user input
        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read line");
        
        // Parse and validate user input
        let height: f32 = match height.trim().parse() {
            Ok(num) => num,

            // For the catch all case, we check if the user typed "quit" to exit the program
            Err(_) => match height.trim() {
                "quit" => break,
                _ => {
                    println!("Invalid measurement. Please try again.\n");
                    continue;
                },
            },
        };

        // If the height is less than or equal to 0, we print an error message and prompt the user to try again
        if height <= 0.0 {
            println!("\nInvalid measurement. Please try again.\n");
            continue;
        }

        // Output the volume of the cone using the compute_cone_volume function
        println!("The volume of the cone is: {} milliliters.\n", compute_cone_volume(radius, height));

    }

    // Exit message
    println!("\nGoodbye!");
}

/* 
    Function name: 
        compute_cone_volume

    Parameters:
        radius (f32), height (f32)

    Returns:
        volume (f32)

    Description: 
        This function calculates the volume of a cone given the radius and height.
*/
fn compute_cone_volume(radius: f32, height: f32) -> f32 {

    // Constant value for pi
    const PI: f32 = 3.14159;

    // Calculate the volume of the cone using the formula: V = π * r^2 * h / 3
    let volume = PI * radius * radius * height / 3.0;

    // Return the volume
    volume
}
