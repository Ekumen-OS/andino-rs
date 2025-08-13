use dora_node_api::{DoraNode, Event, arrow::array::Float64Array, dora_core::config::DataId};

pub fn main() -> eyre::Result<()> {
    println!("Initializing Andino HAL interface...");

    // Configuration from environment variables
    let serial_device = std::env::var("SERIAL_DEVICE").unwrap_or_else(|_| "/dev/ttyUSB0".to_string());
    let baud_rate = std::env::var("BAUD_RATE")
        .unwrap_or_else(|_| "57600".to_string())
        .parse::<u32>()
        .unwrap_or(57600);
    let timeout = std::env::var("TIMEOUT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u64>()
        .unwrap_or(3000);
    let motor_ticks_per_revolution = std::env::var("MOTOR_TICKS_PER_REVOLUTION")
        .unwrap_or_else(|_| "700".to_string())
        .parse::<u64>()
        .unwrap_or(700);

    let hal_config = andino::core::hal::HalConfig {
        serial_device,
        baud_rate,
        timeout,
        motor_ticks_per_revolution,
    };
    println!("HalConfig: {:?}", &hal_config);

    let mut andino_hal = andino::core::hal::Hal::new(&hal_config)?;

    // TODO(francocipollone): Remove this sleep
    std::thread::sleep(std::time::Duration::from_secs(3));

    let output_wheel_joint_positions = DataId::from("wheel_joint_positions".to_owned());
    let output_wheel_joint_velocities = DataId::from("wheel_joint_velocities".to_owned());

    let (mut node, mut events) = DoraNode::init_from_env()?;

    let mut andino_hal_state;
    let mut last_timestamp = Option::None;
    while let Some(event) = events.recv() {
        match event {
            Event::Stop(_) => {
                println!("Received stop event");
                break;
            }
            Event::Input { id, data, metadata } => {
                match id.as_str() {
                    "tick" => {
                        if last_timestamp.is_none() {
                            last_timestamp = Some(metadata.timestamp());
                        }
                        let delta_time = metadata
                            .timestamp()
                            .get_diff_duration(&last_timestamp.unwrap())
                            .as_secs_f64();
                        andino_hal_state = andino_hal.poll_state(delta_time)?;

                        // Publish wheel joint positions
                        let wheel_joint_positions_data = Float64Array::from(vec![
                            andino_hal_state.left_wheel_state.position,
                            andino_hal_state.right_wheel_state.position,
                            metadata.timestamp().get_time().to_duration().as_secs_f64(),
                        ]);
                        node.send_output(
                            output_wheel_joint_positions.clone(),
                            metadata.parameters.clone(),
                            wheel_joint_positions_data,
                        )?;
                        // Publish wheel joint velocities
                        let wheel_joint_velocities_data = Float64Array::from(vec![
                            andino_hal_state.left_wheel_state.velocity,
                            andino_hal_state.right_wheel_state.velocity,
                            metadata.timestamp().get_time().to_duration().as_secs_f64(),
                        ]);
                        node.send_output(
                            output_wheel_joint_velocities.clone(),
                            metadata.parameters.clone(),
                            wheel_joint_velocities_data,
                        )?;
                    }
                    "joints_speed_cmd" => {
                        let values = if let Some(float_array) = data.as_any().downcast_ref::<Float64Array>() {
                            float_array
                        } else {
                            eprintln!("Not a Float64Array!");
                            continue;
                        };
                        if values.len() != 2 {
                            eprintln!("Expected 2 elements in the list, got: {:?}", data);
                            continue;
                        }
                        let left_speed = values.value(0);
                        let right_speed = values.value(1);
                        andino_hal.set_motor_speed(left_speed, right_speed)?;
                    }
                    _ => {
                        println!("Unexpected input id: {:?}", id);
                    }
                }
                last_timestamp = Some(metadata.timestamp());
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
