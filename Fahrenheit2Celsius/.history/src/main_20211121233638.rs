use std::io;

// 摄氏华氏转换器
fn main() {
    
    'outloop: loop {

        println!("Please select what you what to do: \n");
        println!("1. Fahrenheit to Celsius\n");
        println!("2. Celsius to Fahrenheit\n");
        let mut selected = String::new();
        
        match io::stdin().read_line(&mut selected) {
            Ok(_) => {
            },
            Err(_) => continue 'outloop,
        };

        let select = match selected.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue 'outloop,
        };
        
        if select != 1 && select != 2 {
            println!("Please type 1 or 2");
            continue 'outloop;
        }
        loop {
            println!("Please type your temperature \n");
            let mut num = String::new();
            match io::stdin().read_line(&mut num) {
                Ok(_) => {
                },
                Err(_) => continue,
            }
            let mut num = match num.trim().parse::<f64>() {
                Ok(value) => value,
                Err(_) => continue,
            };
            if select == 1 {
                num = (num - 32.0)/1.8;
            } else {
                num = num * 1.8 + 32.0;
            }
            println!("you reslut is {} \n", num);
            continue 'outloop;
        }
    }
}
