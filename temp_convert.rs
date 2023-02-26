use std::io;

fn main () {
    println!("Welcome to Temperature Converter");
    println!("To end the program, type `exit` ");
    
    loop {
        println!("Type F for celsius to fahrenheit \nType C for fahrenheit to celsius : ");
        let mut int = String::new();
        io::stdin()
            .read_line(&mut int)
            .expect("");

        if int.trim() == "exit"{
            break;
        }
        else if int.trim()== "C"{
            println!("Enter temperature in celsius");
            let mut temperature = String::new();
            io::stdin()
                .read_line(&mut temperature)
                .expect("");
            let temp_rust : f32 = match temperature.trim()
            .parse() {
                Ok(temperature) => temperature,
                Err(_) => continue,
            };
            println!("Temperature in fahrenheit is {} degrees", ftoc(temp_rust)); 
            
        }
        else if int.trim() == "F"{
            println!("Enter temperature in Celsius");
            let mut temperature = String::new();
            io::stdin()
                .read_line(&mut temperature)
                .expect("");
            let temp_rust : f32 = match temperature.trim()
            .parse() {
                Ok(temperature) => temperature,
                Err(_) => continue,
            };
            println!("Temperature in fahrenheit is {} degrees", ctof(temp_rust)); 
        }
        else {
            println!("Please enter valid input");
        }
    }
}

// Celsius to Fahrenheit
fn ctof(c: f32) -> f32 {
    (c * (9.0 / 5.0)) + 32.0

}

//Fahrenheit to Celsius
fn ftoc(f: f32) -> f32 {
    (f-32.0) * (5.0 / 9.0)
}

