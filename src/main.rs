use std::error::Error;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::{env::args, fs};

const VM_FILE_EXTENSION: &str = ".vm";

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = PathBuf::new().join(
        args()
            .nth(1)
            .ok_or_else(|| "Expected filepath as first argument")?,
    );

    println!("Input: {}", input_path.display());

    let metadata = fs::metadata(&input_path)?;

    let filename = get_filename(&input_path)?;

    if metadata.is_file() {
        let out_filename = format!("{}.asm", filename.replace(VM_FILE_EXTENSION, ""));
        let output_path = PathBuf::new().join("out").join(out_filename);
        println!("Output: {}", output_path.display());

        let input = fs::read_to_string(&input_path)?;
        let output = vm_translator::translate(filename.into(), input)?;
        let res = fs::create_dir("out");
        if let Err(err) = res {
            if err.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(err.into());
            }
        }
        fs::write(output_path, output)?;
        return Ok(());
    } else if metadata.is_dir() {
        let out_filename = format!("{}.asm", filename);
        let output_path = PathBuf::new().join("out").join(out_filename);
        println!("Output: {}", output_path.display());

        let entries: Vec<PathBuf> = fs::read_dir(&input_path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?
            .into_iter()
            .filter(|path| {
                path.is_file()
                    && path
                        .to_str()
                        .is_some_and(|str| str.ends_with(VM_FILE_EXTENSION))
            })
            .collect();

        if entries.is_empty() {
            return Err(std::io::Error::new(
                ErrorKind::InvalidFilename,
                format!("No {} files found in {}", VM_FILE_EXTENSION, filename),
            ))?;
        }

        let mut output_file = fs::File::create(output_path)?;

        entries
            .into_iter()
            .map(|input_path| {
                print!("Processing {:?} - ", input_path);
                let input = fs::read_to_string(&input_path)?;
                let output = vm_translator::translate(get_filename(&input_path)?, input)?;
                output_file.write(output.as_bytes())?;
                println!("done");
                Ok(())
            })
            .collect::<Result<(), Box<dyn Error>>>()?;
        println!("Finished");
        return Ok(());
    }

    Err(std::io::Error::new(
        ErrorKind::InvalidFilename,
        format!("{} expected to be file or directory", filename),
    ))?
}

fn get_filename(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let filename = path
        .file_name()
        .ok_or_else(|| "Can't get filename")?
        .to_str()
        .ok_or_else(|| "Invalid filename")?;
    Ok(filename.into())
}
