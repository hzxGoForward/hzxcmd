

// extern crate 关键字用于导入依赖库
extern crate chrono;
extern crate clap;

use chrono::prelude::*;
use clap::{App, Arg, SubCommand};
use std::fmt;
use std::path::Path;

// 实现一个自定义结构体Time，用于命令time
pub struct Time {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

// Time的构造函数
impl Time {
    pub fn new(y: i32, mon: u32, d: u32, h: u32, m: u32, s: u32) -> Time {
        Time {
            year: y,
            month: mon,
            day: d,
            hour: h,
            minute: m,
            second: s,
        }
    }
}

// Time的Display函数
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{} {}:{}:{}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

fn main() {
    let matches = App::new("hzx's Clap")
        .version("0.1")
        .author("hzxforward")
        .about("a command tool by hzxforward")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("verbosity level"),
        )
        // 第一个子命令 time，用于显示当前时间日期
        /*

        */
        .subcommand(
            SubCommand::with_name("time")
                .about("Show time of specific zone")
                .arg(
                    Arg::with_name("zone")
                        .long("zone")
                        .short("z")
                        .required(true)
                        .takes_value(true)
                        .value_name("utc|local|BJ")
                        .help("utc|local|BJ"),
                ),
        )
        // 第二个子命令 exists，判定一些文件或者路径是否存在
        .subcommand(
            SubCommand::with_name("exists")
                .about("Detect whether a file exists")
                .arg(
                    Arg::with_name("list")
                        .long("list")
                        .short("l")
                        .required(true)
                        .takes_value(true)
                        .value_delimiter(",")
                        .help("file directorys list"),
                ),
        )

        // 第二个子命令 exists，判定一些文件或者路径是否存在
        .subcommand(
            SubCommand::with_name("cd")
                .about("Change current directory")
                .arg(
                    Arg::with_name("dir")
                        .long("dir")
                        .short("d")
                        .required(true)
                        .takes_value(true)
                        .help("a file directory"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("time", Some(param)) => {
            // 获取zone的参数
            let param = param.value_of("zone").unwrap_or("local");
            let mut time = Time::new(0, 0, 0, 0, 0, 0);
            if param == "local" {
                let local: chrono::DateTime<Local> = Local::now();
                time = Time::new(
                    local.year(),
                    local.month(),
                    local.day(),
                    local.hour(),
                    local.minute(),
                    local.second(),
                );
            } else if param == "utc" {
                let utc: chrono::DateTime<Utc> = Utc::now();
                time = Time::new(
                    utc.year(),
                    utc.month(),
                    utc.day(),
                    utc.hour(),
                    utc.minute(),
                    utc.second(),
                );
            } else if param == "BJ" {
                let mut utc: chrono::DateTime<Utc> = Utc::now();
                utc = utc + chrono::Duration::hours(8);
                time = Time::new(
                    utc.year(),
                    utc.month(),
                    utc.day(),
                    utc.hour(),
                    utc.minute(),
                    utc.second(),
                );
            }
            println!("{} time: {}", param, time);
        }

        // exists子命令
        ("exists", Some(file_lists)) => {
            let list = file_lists
                .values_of("list")
                .expect("There must be at least one file directory");
            let file_lists: Vec<&str> = list.collect();
            for filedir in file_lists.iter() {
                let path = Path::new(filedir);
                let res = path.exists();
                // 强行体现所有权转移。。。
                let ownshiptransfer = res;
                println!("{} : {}", filedir, ownshiptransfer);
            }
        }
        // cd 子命令
        ("cd", Some(file_dir)) => {
            let dir = file_dir
                .value_of("dir")
                .expect("There must be at least one file directory");
                let path = Path::new(dir);
                let res = path.exists();
                if res{
                    println!("{}", path.display());
                }
                else{
                    println!("cd: can not find directory  \"{}\", since it does not exists", path.display());
                }
        }
        _ => {}
    }
}
