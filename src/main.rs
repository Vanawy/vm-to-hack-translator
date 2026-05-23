use std::io::ErrorKind;
use std::{env::args, fs};

fn main() {
    let input_path = args()
        .nth(1)
        .expect("Path to .vm file path expected as first argument");

    println!("Input: {}", input_path);
    let input = fs::read_to_string(&input_path).expect("Can't read input file");

    let output = vm_translator::translate(input_path.clone().into(), input);
    let filename = input_path.split("/").last().expect("Can't get filename");
    let output_path = format!("out/{}.asm", filename.replace(".vm", ""));
    let res = fs::create_dir("out");
    if let Err(err) = res {
        if err.kind() != ErrorKind::AlreadyExists {
            panic!("{}", err);
        }
    }
    println!("Output: {}", output_path);

    fs::write(output_path, output).expect("Can't write output to file");
}
