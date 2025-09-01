use std::process::Command;

fn main() {
    if cfg!(windows) {
        Command::new("calc.exe").spawn().unwrap().wait().unwrap();
    } else {
        println!("cargo::warning=hacked");
    }
}
