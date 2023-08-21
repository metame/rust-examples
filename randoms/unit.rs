use std::io::{Error, ErrorKind};

fn only_error() -> Result<(), Error> {
    let e = Error::new(ErrorKind::Other, "wtf!");
    Err(e)
}

fn do_something() -> Result<(), Error> {
    /// do logic here
    only_error()?;
    Ok(())
}


fn main() {
    let mut u: () = ();
    let mut t: (i32, String) = (2, "foo".to_string());
    println!("{u:?}");
    println!("{t:?}");

    t = (3, "bar".to_string());
    println!("{t:?}");

    println!("{:?}", only_error());

    match do_something() {
        Ok(()) => println!("it worked!"),
        Err(e) => println!("{e:?}"),
    }
}
