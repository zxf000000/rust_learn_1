use std::io;

fn main() {
    println!("Please select what you what to do: \n");
    println!("1. Fahrenheit to Celsius\n");
    println!("2. Celsius to Fahrenheit\n");
    let mut selected = String::new();
    
    io::stdin().read_line(&mut selected)
        .expect("Error reading");
    
    let select = match selected.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    
    F=C×1.8+32
    C=(F-32)÷1.8
}
