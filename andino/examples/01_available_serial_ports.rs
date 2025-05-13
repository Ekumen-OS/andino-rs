// ***************************************************************************
// About
// ***************************************************************************
//
//! This example shows how to list all available serial ports on the system using
//! the `serialport` crate.

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}
