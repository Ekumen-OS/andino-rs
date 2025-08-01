use dora_node_api::{DoraNode, Event, arrow::array::Float64Array, dora_core::config::DataId};

pub fn main() -> eyre::Result<()> {
    let output_joints_speed = DataId::from("joints_speed_cmd".to_owned());

    let (mut node, mut events) = DoraNode::init_from_env()?;

    // Obtain information from the environment variables: wheel_radius and wheel_separation
    let wheel_radius = std::env::var("WHEEL_RADIUS")
        .unwrap_or_else(|_| "0.035".to_string())
        .parse::<f64>()
        .unwrap_or(0.035);
    let wheel_separation = std::env::var("WHEEL_SEPARATION")
        .unwrap_or_else(|_| "0.137".to_string())
        .parse::<f64>()
        .unwrap_or(0.137);
    println!(
        "Config: wheel_radius = {:?}, wheel_separation = {:?}",
        wheel_radius, wheel_separation
    );

    let diff_drive_controller = crate::controller::DiffDriveController::new(wheel_separation, wheel_radius);

    while let Some(event) = events.recv() {
        match event {
            Event::Stop(_) => {
                println!("Received stop event");
                break;
            }
            Event::Input { id, data, metadata } => {
                match id.as_str() {
                    "cmd_vel" => {
                        // Receives cmd_vel [forward, 0.0, 0.0 , 0.0, 0.0, yaw_rate]
                        let values = if let Some(float_array) = data.as_any().downcast_ref::<Float64Array>() {
                            float_array
                        } else {
                            eprintln!("cmd_vel: Not a Float64Array!");
                            continue;
                        };
                        if values.len() != 6 {
                            eprintln!(
                                "cmd_vel: Not a Float64Array with 6 elements. It expects a Float64Array with 6 elements: [forward, 0.0, 0.0 , 0.0, 0.0, yaw_rate]"
                            );
                            continue;
                        }
                        let forward = values.value(0);
                        let yaw_rate = values.value(5);
                        let (left_wheel_speed, right_wheel_speed) =
                            diff_drive_controller.compute_wheel_speeds(forward, yaw_rate);
                        // Send float array to joints_speed_cmd output
                        let speed_array = Float64Array::from(vec![left_wheel_speed, right_wheel_speed]);
                        node.send_output(output_joints_speed.clone(), metadata.parameters, speed_array)?;
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
