use std::process;

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

    if input.trim() == "继续"{
        println!("抱歉，程序无法继续，因为后面还没做");
        end_program(0);
    }
}

//结束程序
fn end_program(code: i32){
    println!("{:?}",END_MSG);
    process::exit(code)
}
