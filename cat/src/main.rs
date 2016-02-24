use std::io;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => match io::stdout().write(&buffer.as_bytes()) {
            Ok(_) => {}
            Err(err) => println!("Error writing: {:?}", err),
        },
        Err(err) => println!("Error reading: {:?}", err),
    }
}
