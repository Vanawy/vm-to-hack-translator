use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::exit;
use std::{env::args, fs};

fn main() {
    let input_path = PathBuf::new().join(
        args()
            .nth(1)
            .expect("Path to .vm file path expected as first argument"),
    );

    println!("Input: {}", input_path.display());
    let input = fs::read_to_string(&input_path).expect("Can't read input file");

    let filename = input_path
        .file_name()
        .expect("Can't get filename")
        .to_str()
        .expect("Can't convert filename to str");

    let output = vm_translator::translate(filename.into(), input);
    let out_filename = format!("{}.asm", filename.replace(".vm", ""));
    let output_path = PathBuf::new().join("out").join(out_filename);
    let res = fs::create_dir("out");
    if let Err(err) = res {
        if err.kind() != ErrorKind::AlreadyExists {
            panic!("{}", err);
        }
    }
    println!("Output: {}", output_path.display());

    match output {
        Ok(out) => {
            fs::write(output_path, out).expect("Can't write output to file");
        }
        Err(err) => {
            eprintln!("{}", err.message);
            exit(1)
        }
    }
}
