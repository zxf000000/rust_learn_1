use std::io;
// 生成斐波那契数列
fn main() {
    println!(" 生成斐波那契数列程序: \n");

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
        let num = match num.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("输入的不是数字!");
                continue;
            },
        };
        let vec = Vec::<u64>::new();
        for i in 0..num {
            if i == 0 || i == 1 {
                vec.push(1);
            } else {
                let res = vec[i - 1] + vec[i - 2];
            }
        }
        println!("reslut is {:?}", vec);
    }
}
