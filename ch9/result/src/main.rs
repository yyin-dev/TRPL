use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    
    // Shortcuts for panic on error
    // - unwrap(): if the Result value is Ok, return the value inside Ok; if the
    // Result is Err, call panic!()    
    // - expect(): similar to unwrap(), but allow you to specify panic message

    // Propagating Errors
    // Return the error to the calling code so that it can decide what to do.
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    // All branches of `match` must have the same type. Why `return Err(e)` 
    // type-checks with `file`?
    // https://discord.com/channels/442252698964721669/448238009733742612/1012926389529362453
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    // ? placed after a Result is defined to work the same way as above `match`:
    // If the value of Result is Ok, then the value inside Ok is returned from
    // the expression; if the value is Err, the Err will be returned from the
    // whole function. 
    // ? can only be used in functions that return Result or Option
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}