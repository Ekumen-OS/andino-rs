use dora_node_api::{
    DoraNode, Event,
    arrow::array::{Float64Array, StringArray},
    dora_core::config::DataId,
};
use std::vec;

fn create_twist_array(linear: f64, angular: f64) -> Float64Array {
    Float64Array::new(vec![linear, 0.0, 0.0, 0.0, 0.0, angular].into(), None)
}

pub fn main() -> eyre::Result<()> {
    let output_cmd_vel = DataId::from("cmd_vel".to_owned());
    println!("Teleop Keyboard Node initialized");

    // Configure the node
    let mut linear_speed = std::env::var("DEFAULT_LINEAR_SPEED")
        .unwrap_or_else(|_| "0.4".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    let mut angular_speed = std::env::var("DEFAULT_ANGULAR_SPEED")
        .unwrap_or_else(|_| "0.4".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    println!("Default linear speed: {}", linear_speed);
    println!("Default angular speed: {}", angular_speed);

    let (mut node, mut events) = DoraNode::init_from_env()?;
    while let Some(event) = events.recv() {
        match event {
            Event::Stop => {
                println!("Received stop event");
                break;
            }
            Event::Input { id, data, metadata } => {
                match id.as_str() {
                    "key" => {
                        // Get the character from the input data
                        let value = data.as_any().downcast_ref::<StringArray>().unwrap();

                        let current_cmd = value.value(0).to_string();

                        let twist_array = match current_cmd.as_str() {
                            "u" => create_twist_array(linear_speed, angular_speed),
                            "i" => create_twist_array(linear_speed, 0.0),
                            "o" => create_twist_array(linear_speed, -angular_speed),
                            "j" => create_twist_array(0.0, angular_speed),
                            "k" => create_twist_array(0.0, 0.0),
                            "l" => create_twist_array(0.0, -angular_speed),
                            "m" => create_twist_array(-linear_speed, angular_speed),
                            "," => create_twist_array(-linear_speed, 0.0),
                            "." => create_twist_array(-linear_speed, -angular_speed),
                            "q" => {
                                linear_speed *= 1.1;
                                angular_speed *= 1.1;
                                println!(
                                    "Increasing speed: linear = {}, angular = {}",
                                    linear_speed, angular_speed
                                );
                                continue;
                            }
                            "z" => {
                                linear_speed *= 0.9;
                                angular_speed *= 0.9;
                                println!(
                                    "Decreasing speed: linear = {}, angular = {}",
                                    linear_speed, angular_speed
                                );
                                continue;
                            }
                            "w" => {
                                linear_speed *= 1.1;
                                println!("Increasing linear speed: {}", linear_speed);
                                continue;
                            }
                            "x" => {
                                linear_speed *= 0.9;
                                println!("Decreasing linear speed: {}", linear_speed);
                                continue;
                            }
                            "e" => {
                                angular_speed *= 1.1;
                                println!("Increasing angular speed: {}", angular_speed);
                                continue;
                            }
                            "c" => {
                                angular_speed *= 0.9;
                                println!("Decreasing angular speed: {}", angular_speed);
                                continue;
                            }
                            "stop" => create_twist_array(0.0, 0.0),
                            "quit" => {
                                break;
                            }
                            _ => create_twist_array(0.0, 0.0),
                        };
                        node.send_output(output_cmd_vel.clone(), metadata.parameters.clone(), twist_array)?;
                    }
                    _ => {
                        println!("Unexpected input id: {:?}", id);
                    }
                }
            }
            Event::Reload { operator_id } => {
                eprintln!("Not expected: Received reload event for operator: {:?}", operator_id);
            }
            Event::InputClosed { id } => {
                eprintln!("Not expected: Received input closed event: id = {:?}", id);
            }
            Event::Error(err) => {
                eprintln!("Not expected: Received error event: {:?}", err);
            }
            _ => {
                eprintln!("Received unexpected event: {:?}", event);
            }
        }
    }

    Ok(())
}
