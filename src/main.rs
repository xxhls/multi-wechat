use std::io;

fn main() {
    let mut stored_value: i32 = 0;

    loop {
        println!("请输入一个待开启的微信数量 (输入 'q' 退出):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        let input = input.trim();
        if input == "q" {
            println!("程序退出");
            break;
        }

        match input.parse::<i32>() {
            Ok(num) => {
                stored_value = num;
                break;
            }
            Err(_) => {
                println!("输入无效，请输入一个合法的整数");
            }
        }
    }

    if stored_value <= 0 {
        println!("输入的值必须大于 0");
        return;
    }

    println!("开始开启 {} 个微信", stored_value);
}