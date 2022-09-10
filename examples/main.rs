use smart_socket::{Factory, SmartSocket};
use std::error::Error;

mod smart_socket;

fn main() -> Result<(), Box<dyn Error>> {
    let lib = Factory::new()?;
    let mut smart_socket = SmartSocket::new();

    let status = lib.get_status(&smart_socket);
    println!("{}", status);

    lib.toggle(&mut smart_socket);

    let status = lib.get_status(&smart_socket);
    println!("{}", status);
    Ok(())
}
