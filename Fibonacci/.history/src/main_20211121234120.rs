// 生成斐波那契数列
use std::io;

fn main() {
    println!(" 生成斐波那契数列程序: \n");
    println!(" 请输入结数: \n");
    let mut num = String::new();
    loop {
        match io::stdin().read_line(&mut num) {
            Err(_) => continue,
            Ok(_) => {},
        }
    }
}
