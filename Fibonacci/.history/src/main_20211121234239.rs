// 生成斐波那契数列
use std::io;
println!(" 生成斐波那契数列程序: \n");
fn main() {
    println!(" 请输入结数: \n");
    let mut num = String::new();
    loop {
        match io::stdin().read_line(&mut num) {
            Err(_) => continue,
            Ok(_) => {},
        }
        let num = matchz num.trim().parse::<i64> {
            Ok(value) => value,
            Err(_) => {
                println!("Cannot get right number");
                continue;
            },
        }
    }
}
