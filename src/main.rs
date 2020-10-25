use std::env;
mod common;
mod video;

fn main() {
    let args: Vec<String> = env::args().collect();

    // utils
    if args.len() >= 3 && args[1] == "util" {
        let module_name: &str = &args[2];
        common::utils::exe_module(module_name, (&args[3..]).to_vec());
    }
}
