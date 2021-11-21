use std::str::FromStr;
use std::io;

fn main() {
    println!("Please select what you what to do: \n");
    println!("1. Fahrenheit to Celsius\n");
    println!("2. Celsius to Fahrenheit\n");
    let mut selected = String::new();
    
    io::stdin().read_line(&mut selected)
        .expect("Error reading");
    
    'outloop: loop {
        let select = match selected.trim().parse() {
            Ok(num) => num,
            Err(_) => continue 'outloop,
        };
        
        if select != 1 && select != 2 {
            println!("Please type 1 or 2");
            continue 'outloop;
        }
        loop {
            println!("Please type your temperature");
            let mut num = String::new();
            match io::stdin().read_line(&mut num) {
                Ok(_) => {
                    let num = match f64::from_str(&(num.trim())) {
                        Ok(value) => value,
                        Err(_) => continue,
                    };
                    num = (num - 32.0)/1.8;
                    println!("you reslut is {}", num);
                    continue 'outloop;
                },
                Err(_) => continue,
            }
        }
    }


    F=C×1.8+32
    C=(F-32)÷1.8
}
