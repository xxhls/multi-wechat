use std::io;
use std::fs;
use std::process::Command;

fn main() {
    let mut stored_value: i32 = 0;

    loop {
        println!("请输入一个待开启的微信数量 (输入 'q' 退出):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        let input = input.trim().to_lowercase();
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

    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-Volume | Select-Object -ExpandProperty DriveLetter")
        .output()
        .expect("Failed to execute PowerShell command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let disks: Vec<String> = output_str.split_whitespace().map(|s| s.to_string()).collect();

    println!("磁盘名称列表: {:?}", disks);

    let mut wechat_paths = Vec::new();

    for disk in disks {
        let disk_clone = disk.clone();
        let output = Command::new("cmd")
            .args(&["/C", &format!("WHERE /R {}\\ WeChat.exe", disk_clone)])
            .output()
            .expect("Failed to execute cmd command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        let paths: Vec<&str> = output_str.split_whitespace().collect();
        if paths.len() > 0 {
            let path = paths.get(0).unwrap_or(&"").to_string();
            wechat_paths.push(path);
            break;
        }
    }

    if wechat_paths.is_empty() {
        println!("未找到 WeChat.exe，请手动输入路径:");
        let mut manual_path = String::new();
        io::stdin().read_line(&mut manual_path).expect("读取输入失败");
        let manual_path = manual_path.trim().to_string();
        if fs::metadata(&manual_path).is_ok() {
            wechat_paths.push(manual_path);
        } else {
            println!("输入的路径无效");
            return;
        }
    }

    println!("WeChat.exe 路径列表: {:?}", wechat_paths);
}