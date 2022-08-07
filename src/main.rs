use colored::Colorize;
use walkdir::WalkDir;
use clap::Parser;
/// 遍历目标目录中包含指定关键字的文件，并从匹配到的文件中匹配特定字符串所在行
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 查询文件目标目录
    #[clap(short, long)]
    dir: String,

    /// 指定文件名中包含的关键字
    #[clap(short, long)]
    pre: String,

    /// 指定文件内容包含的关键字（支持正则）
    #[clap(short, long)]
    keys: String,

    /// 是否启用正则(默认关闭，输入y开启正则模式)
    #[clap(short, long,takes_value = false)]
    reg:bool,
}

fn main() {

    let args = Args::parse();
    let dir  = args.dir;
    let pre:Vec<&str> = args.pre.as_str().split(",").collect();
    println!("{}","Walking dir....please wait.....".red());
    let f_name:Vec<String> = WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
            .map(|x| 
                x.path().to_string_lossy().to_string())
            .collect();

    let keys = if args.reg{
        vec![args.keys.as_str().to_string()]
    }else{
        args.keys.as_str().split(",").map(|x| x.to_string()).collect()
    };
    for i in pre{
        for j in &f_name{
            if j.contains(i){
                let engine = lazyfinder::FileMatcher::new(j,&keys,&args.reg);
                if let Err(e) = lazyfinder::run(engine){
                    println!("Error !:{}",e);
                };
            }
        }
    }
    println!("{}","done....".red());
}