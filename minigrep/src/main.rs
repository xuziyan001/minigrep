use std::env;
use std::process;

use minigrep_shits_always_happen;
use minigrep_shits_always_happen::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // 打印到标准错误流
        process::exit(1);
    });
    if let Err(e) = minigrep_shits_always_happen::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
