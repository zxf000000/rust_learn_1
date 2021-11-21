// 生成斐波那契数列
use std::io;
println!(" 生成斐波那契数列程序: \n");
fn main() {
    println!(" 请输入结数: \n");
    let mut num = String::new();
    loop {
        match io::stdin().read_line(&mut num) {
            Err(_) => {
                println!("无效输入!");
                continue;
            },
            Ok(_) => {},
        }
        let num = match num.trim().parse::<i64>() {
            Ok(value) => value,
            Err(_) => {
                println!("输入的不是数字!");
                continue;
            },
        };
        let vec = Vec::<u64>::new();
        for i in 0..num {
        }
    }
}
