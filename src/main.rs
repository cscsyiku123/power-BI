use clap::Parser;
use properties::Property;



#[macro_use]
extern crate lazy_static;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::sync::Mutex;
use tera::Tera;
use tera::Context;
extern crate serde;
extern crate tera;
use serde::{Serialize, Deserialize};



/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// config file to be used
    #[arg(short, long)]
    config: String,

}

lazy_static! {
	// 此处表示声明全局可变 HashMap
    static ref MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// 读取文件,path表示文件路径
fn read_file(path: &str) -> Result<i32, io::Error> {
    // 读取文件，失败直接返回Err
    let file: File = File::open(path)?;
    let buffered: BufReader<File> = BufReader::new(file);
    // 存放`[key]`
    let mut key: String = "".to_string();
    // 缓存 : 去掉空格
    let mut new_line = "".to_string();

    for line in buffered.lines().map(|x| x.unwrap()) {
        new_line.clear();
        new_line.push_str(line.trim());
        // 定义注释为`#`, 遇到注释跳过
        if line.contains("#") {
            continue;
        } else if line.contains("[") && line.contains("]") { // 解析`[key]`为`key::`
            key.clear();
            new_line.pop();
            new_line.remove(0);
            key.push_str(new_line.as_str());
            key.push_str("::");
        } else if new_line.contains("=") { // 必须包含表达式, 才将值写入
            let kvs: Vec<&str> = new_line.as_str().split("=").collect::<Vec<&str>>();
            if kvs.len() == 2 { // 如果不满足则定义为异常数据，该数据暂不处理
                // 缓存
                let mut new_key: String = key.clone();
                new_key.push_str(kvs[0]);

                MAP.lock().unwrap().insert(new_key.trim().to_string(), kvs[1].trim().to_string());
            }
        }
    }
    return Ok(0);
}







fn main() {
    let args = Args::parse();
    // 这里只需要处理错误
    if let Err(e) = read_file(&args.config) {
        println!("err = {:?}", e);
    }

    for (key, value) in MAP.lock().unwrap().iter() {
        println!("k = {}, y = {}", key, value);
    }

    if let Some(x) = MAP.lock().unwrap().get("front-end"){
        //解析得到方法参数
        println!("x = {}", x);
    };
    if let Some(x) = MAP.lock().unwrap().get("back-end"){
        println!("x = {}", x);
    };
    if let Some(x) = MAP.lock().unwrap().get("ids"){
        println!("x = {}", x);
    };
    if let Some(x) = MAP.lock().unwrap().get("annotation"){
        println!("x = {}", x);
    };

    //魔板替换参数
    lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        // tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
    }
    //1、 Using the tera Context struct
    let mut context = Context::new();
    //2、封装参数
    let mut  project = Project::new();
    project.setName(String::from("ContentAsset"));

    //3、render
    let method = Method {
        name:String::from(""),
        params:vec![Param{name:String::from("foo"), type_:String::from("String")}],
    };
    context.insert("project", &project);
    context.insert("method", &method);
    let x1 = TEMPLATES.render("FrontTemplate", &context);
    println!("x1 = {:?}", x1);


}

#[derive(Serialize,Debug)]
struct Project {
    name: String,
}
impl Project{
    fn new() -> Self {
        Project {
            name: String::new(),
        }
    }
    fn setName(&mut self,name: String){
        self.name = name;
    }
}

#[derive(Serialize,Debug)]
struct Method {
    name: String,
    params: Vec<Param>,
}

#[derive(Serialize,Debug)]
struct Param {
    name: String,
    type_ :String ,
}

