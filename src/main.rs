mod func;

pub use crate::func::exit_codes;
use std::{fs, io::ErrorKind, process};

//程序名称
const NAME : &str = "game maker";
const END_MSG : &str = "程序已结束";
const LEFT_CN_QUOTE : &str = "\u{201C}";
const RIGHT_CN_QUOTE : &str = "\u{201D}";

//入口
fn main() {
    println!("欢迎使用\"{NAME}\"");
    println!("请输入{LEFT_CN_QUOTE}继续{RIGHT_CN_QUOTE}");
    //读取用户输入，如果读到"继续"，会继续流程
    get_input();
    end_program(exit_codes::ExitCodes::SUCCESS as i32);
}

fn create_game_dir(){
    let game_dir = fs::create_dir("game");
    match game_dir {
        Ok(_d) => println!("文件夹创建成功!"),
        Err(e) if e.kind() != ErrorKind::AlreadyExists=> println!("{e}文件夹创建失败!"),
        Err(_a) => println!("文件已存在"),
    }
}

fn get_input(){
    let mut input = String::new();
    std::io::stdin()
    .read_line(&mut input)
    .expect("读取失败");
    //读到继续后会创建新文件夹用于存放相关文件
    if input.trim() == "继续"{
        create_game_dir();
    }
}

//结束程序
fn end_program(code: i32){
    println!("{END_MSG}");
    process::exit(code)
}
