use std::io::Read;
use std::env;
use std::fs::File;
use std::process;
use std::path::Path;

use dtos::Api;
#[path="./dtos/api.rs"]
pub mod dtos;


fn read_from_file<P: AsRef<Path>>(path: P) -> String {
    let mut data = String::new();
    let mut file = File::open(path).expect("Unable to open file");
    file.read_to_string(&mut data).expect("Unable to read data");

    return data;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        eprintln!("no file provided");
        process::exit(1);
    }

    let open_api_payload_path = &args[1];

    let file_text: String = read_from_file(open_api_payload_path);

    let api: dtos::Api = serde_json::from_str(&file_text).expect("Failed to parse jason");
    api.print_path_summaries();
}
