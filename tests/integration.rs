use std::{fs, path::PathBuf};
use vm_translator::translate;

fn compare(filename: String) {
    let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push("tests");
    path.push("res");

    let asm = path.join(filename.clone() + ".asm");
    let vm = path.join(filename.clone() + ".vm");

    assert_eq!(
        fs::read_to_string(asm).unwrap(),
        translate(filename.clone() + ".vm", fs::read_to_string(vm).unwrap())
    )
}

#[test]
fn basic() {
    compare("BasicTest".into());
}
#[test]
fn pointer() {
    compare("PointerTest".into());
}
#[test]
fn simple_add() {
    compare("SimpleAdd".into());
}
#[test]
fn stack() {
    compare("StackTest".into());
}
#[test]
fn static_test() {
    compare("StaticTest".into());
}
