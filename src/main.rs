use std::{env::args, fs};

fn main() {
    let input_path = args()
        .nth(1)
        .expect("Path to .vm file path expected as first argument");

    println!("Input: {}", input_path);
    let input = fs::read_to_string(&input_path).expect("Can't read input file");

    let output = vm_translator::translate(input);
    let output_path = input_path.replace(".vm", "") + ".hack";
    println!("Output: {}", output_path);

    fs::write(output_path, output).expect("Can't write output to file");
}
