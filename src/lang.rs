// use rust_i18n::t;
// use std::env;
// use std::fs;
// use std::path::PathBuf;

// // Option()

// pub fn load_lang(lang_code: &str) {
//     let lang_dir = PathBuf::from("lang").join(lang_code);
//     if lang_dir.exists() {
//         rust_i18n::set_locale(lang_code, &[lang_dir]);
//     } else {
//         eprintln!(
//             "The specified language pack does not exist/n指定的语言包不存在: {}",
//             lang_code
//         );
//     }
// }

// pub fn main() {
//     // 先尝试从环境变量获取语言代码
//     if let Ok(lang_from_env) = env::var("APP_LANG") {
//         load_lang(&lang_from_env);
//     } else {
//         // 解析命令行参数
//         let args = Args::parse();
//         if let Some(lang_from_arg) = args.lang {
//             load_lang(&lang_from_arg);
//         } else {
//             // 尝试从配置文件读取（这里简单假设配置文件名为config.toml，里面有个lang字段指定语言代码）
//             if let Ok(config_contents) = fs::read_to_string("config.toml") {
//                 if let Some((_, lang_from_config)) = config_contents.split_once("lang = ") {
//                     let lang_from_config = lang_from_config.trim_matches('"');
//                     load_lang(lang_from_config);
//                 }
//             }
//         }
//     }

//     // 使用翻译后的文本输出
//     println!("{}", t!("hello-world"));
// }

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

pub const PICO_ENV_LANG: &str = "PICO_LANG";

pub fn get_language_from_env() -> Option<String> {
    env::var(PICO_ENV_LANG).ok()
}

pub fn get_language_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    }
}

pub fn get_language_from_config_file() -> Option<String> {
    if let Ok(file) = File::open("config.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line_str) = line {
                if line_str.starts_with("language=") {
                    return Some(line_str[9..].to_string());
                }
            }
        }
    }
    None
}

pub fn load() -> Option<String> {
    let language_from_env = get_language_from_env();
    let language_from_args = get_language_from_args();
    let language_from_config = get_language_from_config_file();

    let selected_language = match (language_from_env, language_from_args, language_from_config) {
        (Some(lang), _, _) => lang,
        (_, Some(lang), _) => lang,
        (_, _, Some(lang)) => lang,
        _ => {
            eprintln!("No language specified.");
            process::exit(1);
        }
    };

    // println!("Selected language: {}", selected_language);
    Some(selected_language)
    // ...
}
