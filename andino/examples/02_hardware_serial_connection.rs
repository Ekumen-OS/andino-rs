// ***************************************************************************
// About
// ***************************************************************************
//
//! Example of how to use the `HwSerialConnection` struct to communicate with
//! the underlying hardware via serial port.
//! Available commands to send to the serial connection:
//! - `ReadEncoderValues`
//! - `SetMotorValues <left> <right>`
//! - `SetPIDValues <kp> <ki> <kd> <ko>`
//!
//! cargo run --example 02_hardware_serial_connection
//!

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    /// Serial device name.
    #[arg(short, long, default_value_t = String::from("/dev/ttyUSB0"))]
    serial_device: String,

    /// Baud rate for the serial connection.
    #[arg(short, long, default_value_t = 57600)]
    baud_rate: u32,

    /// Timeout for the serial connection in milliseconds.
    #[arg(short, long, default_value_t = 3000)]
    timeout: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    log::info!("Creates an instance of HwSerialConnection");
    // Create a new serial connection
    let mut serial_connection =
        andino::core::comm::HwSerialConnection::new(args.serial_device, args.baud_rate, args.timeout)?;

    // TODO(francocipollone): Add a method to check if the serial connection is open and ready.
    log::info!("Waits 3 seconds for the serial connection to be established");
    std::thread::sleep(std::time::Duration::from_secs(3));

    loop {
        // Ask the user for a command, the input can have several arguments
        // For example: "SetMotorValues 100 200"
        println!(
            "Available commands:
            \t - ReadEncoderValues
            \t - SetMotorValues <tps_left> <tps_right>
            \t - SetPIDValues <kp> <ki> <kd> <ko>
            Note: <tps_left> and <tps_right> are ticks(encoder) per second."
        );
        println!("* Enter a command (or 'exit' to quit):");
        let mut input = String::new();

        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input, try again.");
            continue;
        }
        let input = input.trim();
        if input == "exit" {
            break;
        }
        // Split the input into arguments
        let input_args = input.split_whitespace().collect::<Vec<&str>>();

        // Check if the user wants to exit
        // Parse the command
        let command = match input_args[0] {
            "ReadEncoderValues" => andino::core::comm::SerialCommands::ReadEncoderValues,
            "SetMotorValues" => {
                if input_args.len() < 3 {
                    println!("SetMotorValues command requires two arguments.");
                    continue;
                }
                let left = input_args[1].parse::<i64>();
                if left.is_err() {
                    println!("Invalid value for left motor: {}", input_args[1]);
                    continue;
                }
                let right = input_args[2].parse::<i64>();
                if right.is_err() {
                    println!("Invalid value for right motor: {}", input_args[2]);
                    continue;
                }
                andino::core::comm::SerialCommands::SetMotorValues {
                    left: left.unwrap(),
                    right: right.unwrap(),
                }
            }
            "SetPIDValues" => {
                if input_args.len() < 5 {
                    println!("SetPIDValues command requires four arguments.");
                    continue;
                }
                let kp = input_args[1].parse::<f32>();
                if kp.is_err() {
                    println!("Invalid value for kp: {}", input_args[1]);
                    continue;
                }
                let ki = input_args[2].parse::<f32>();
                if ki.is_err() {
                    println!("Invalid value for ki: {}", input_args[2]);
                    continue;
                }
                let kd = input_args[3].parse::<f32>();
                if kd.is_err() {
                    println!("Invalid value for kd: {}", input_args[3]);
                    continue;
                }
                let ko = input_args[4].parse::<f32>();
                if ko.is_err() {
                    println!("Invalid value for ko: {}", input_args[4]);
                    continue;
                }
                andino::core::comm::SerialCommands::SetPIDValues {
                    kp: kp.unwrap(),
                    ki: ki.unwrap(),
                    kd: kd.unwrap(),
                    ko: ko.unwrap(),
                }
            }
            _ => {
                println!("Unknown command: {}\nTry again", input_args[0]);
                continue;
            }
        };

        println!("* Sending command: {:?}", command);
        // Example of sending a command to the serial connection
        let response = serial_connection.send_command(command)?;
        println!("* RESPONSE: {:?}", response);
        println!();
        println!();
    }
    println!("* Exiting the program");
    Ok(())
}
