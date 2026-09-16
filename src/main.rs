mod func;

pub use crate::func::exit_codes;
use std::{fs::{self}, io::ErrorKind, process};

//程序名称
const NAME : &str = "game maker";
const END_MSG : &str = "程序已结束";

//入口
fn main() {
    println!("欢迎使用{:?}",NAME);
    println!("请输入”继续“");
    //读取用户输入，如果读到"继续"，会继续流程（当前无后续流程）
    let mut input = String::new();
    std::io::stdin()
    .read_line(&mut input)
    .expect("读取失败");
    //读到继续后会创建新文件夹用于存放相关文件
    if input.trim() == "继续"{
        let game_dir = fs::create_dir("game");
        match game_dir {
            Ok(_d) => println!("文件夹创建成功!"),
            Err(e) if e.kind() != ErrorKind::AlreadyExists=> println!("{:?}文件夹创建失败!",e),
            Err(_a) => println!("文件已存在"),
        }
        end_program(exit_codes::ExitCodes::SUCCESS as i32);
    }
}

//结束程序
fn end_program(code: i32){
    println!("{:?}",END_MSG);
    process::exit(code)
}
