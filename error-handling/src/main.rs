use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            oe => { // Why is this other_error and not _ ? Ohh because this is the variable name that contains the error. It can be any name. 
                panic!("Problem opening the file {:?}", oe)
            },
        } 

    };
}
