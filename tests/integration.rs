use std::{fs, path::PathBuf};
use vm_translator::{TranslatorError, translate};

fn compare(filename: &str) {
    let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push("tests");
    path.push("res");

    let asm = path.join(filename.to_owned() + ".asm");
    let vm = path.join(filename.to_owned() + ".vm");

    assert_eq!(
        fs::read_to_string(asm).unwrap(),
        translate(filename.to_owned() + ".vm", fs::read_to_string(vm).unwrap()).unwrap(),
    )
}

#[test]
fn basic() {
    compare("BasicTest");
}
#[test]
fn pointer() {
    compare("PointerTest");
}
#[test]
fn simple_add() {
    compare("SimpleAdd");
}
#[test]
fn stack() {
    compare("StackTest");
}
#[test]
fn static_test() {
    compare("StaticTest");
}

#[test]
fn basic_loop() {
    compare("BasicLoop");
}

#[test]
fn file_with_error() {
    let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/res/");

    let filename = "BasicLoopBroken.vm";
    path.push(filename.to_owned());

    assert_eq!(
        Err(TranslatorError {
            message: "line 12: CommandParseError { command: \"pus argument 0\" }".to_owned()
        }),
        translate(filename.to_owned(), fs::read_to_string(path).unwrap())
    );
}
