// ***************************************************************************
// About
// ***************************************************************************
//
//! Example of how to use the `Hal` struct to communicate with the underlying
//! hardware via serial port.

use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    /// Forward speed of the motors in rads per second.
    /// Default is PI: 3.14159... (half rotation per second)
    #[arg(short, long, default_value_t = std::f64::consts::PI)]
    default_forward_speed: f64,

    /// Encoder ticks per revolution.
    #[arg(short, long, default_value_t = 700)]
    ticks_per_revolution: u64,

    /// Serial device name.
    #[arg(short, long, default_value_t = String::from("/dev/ttyUSB0"))]
    serial_device: String,

    /// Baud rate for the serial connection.
    #[arg(short, long, default_value_t = 57600)]
    baud_rate: u32,

    /// Timeout for the serial connection in milliseconds.
    #[arg(long, default_value_t = 3000)]
    timeout: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    println!("\rHal Interface Example:");
    println!("\rCommands:");
    println!("\r  - 'w' to move forward");
    println!("\r  - 's' to move backward");
    println!("\r  - ' ' to stop");
    println!("\r  - 'c' to increase forward speed");
    println!("\r  - 'x' to decrease forward speed");
    println!("\r  - 'q' or 'Esc' to quit");

    log::info!("Creates an instance of andino::core::hal::Hal");
    let mut hal = andino::core::hal::Hal::new(&andino::core::hal::HalConfig {
        serial_device: args.serial_device,
        baud_rate: args.baud_rate,
        timeout: args.timeout,
        motor_ticks_per_revolution: args.ticks_per_revolution,
    })?;
    // TODO(francocipollone): Add a method to check if the serial connection is open and ready.
    log::info!("Waits 3 seconds for the serial connection to be established");
    std::thread::sleep(std::time::Duration::from_secs(3));

    terminal::enable_raw_mode()?;
    // Create a separate thread for getting the commands from the user
    let command = Arc::new(Mutex::new(KeyCode::Char(' ')));
    let command_clone = Arc::clone(&command);
    // Thread to read user input
    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(300)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    let mut cmd = command_clone.lock().unwrap();
                    *cmd = key_event.code;
                }
            } else {
                // If no event is detected, set the command to "stop"
                let mut cmd = command_clone.lock().unwrap();
                *cmd = KeyCode::Char(' ');
            }
        }
    });

    // Define a rate for the loop
    let rate = 10.0; // Hz
    let mut last_time = std::time::Instant::now();
    let mut last_cmd = KeyCode::Char(' ');
    let mut forward_speed = args.default_forward_speed;
    loop {
        let start_time = std::time::Instant::now();
        let delta_time = (start_time - last_time).as_secs_f64();
        last_time = start_time;

        let _hal_state = hal.poll_state(delta_time)?;

        let cmd = *command.lock().unwrap();

        match cmd {
            KeyCode::Char('q') => {
                if last_cmd != KeyCode::Char('q') {
                    println!("\r* Exiting the program by the user");
                }
                hal.set_motor_speed(0.0, 0.0)?;
                break;
            }
            KeyCode::Char('w') => {
                if last_cmd != KeyCode::Char('w') {
                    println!("\r* Command: Move forward");
                }
                hal.set_motor_speed(forward_speed, forward_speed)?
            }
            KeyCode::Char('s') => {
                if last_cmd != KeyCode::Char('s') {
                    println!("\r* Command: Move backward");
                }
                hal.set_motor_speed(-forward_speed, -forward_speed)?;
            }
            KeyCode::Char('c') => {
                if last_cmd != KeyCode::Char('c') {
                    println!("\r* Increase forward speed to: {}", forward_speed);
                }
                forward_speed += 0.1;
            }
            KeyCode::Char('x') => {
                if last_cmd != KeyCode::Char('x') {
                    println!("\r* Decrease forward speed to: {}", forward_speed);
                }
                forward_speed -= 0.1;
            }
            KeyCode::Char(' ') => {
                if last_cmd != KeyCode::Char(' ') {
                    println!("\r* Command: Stop");
                }
                hal.set_motor_speed(0.0, 0.0)?;
            }
            _ => {
                println!("\r* Command: Unknown command");
                hal.set_motor_speed(0.0, 0.0)?;
            }
        }

        // Sleep to maintain the loop rate
        let sleep_time = (1.0 / rate) - (std::time::Instant::now() - start_time).as_secs_f64();
        if sleep_time > 0.0 {
            std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
        }
        last_cmd = cmd;
    }
    terminal::disable_raw_mode()?;
    println!("\r* Exiting the program");
    Ok(())
}
