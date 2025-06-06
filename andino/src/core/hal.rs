use thiserror::Error;

use crate::core::comm::{HwSerialConnection, HwSerialConnectionError};

use crate::core::sensors::{Wheel, WheelState};

/// Error type for hardware abstraction layer (HAL) operations.
#[derive(Debug, Error)]
pub enum HalError {
    #[error(transparent)]
    /// Error communicating with the hardware.
    HardwareCommunicationError(#[from] HwSerialConnectionError),
}

/// Configuration for the hardware abstraction layer (HAL).
#[derive(Debug)]
pub struct HalConfig {
    /// The serial device to connect to (e.g., "/dev/ttyUSB0").
    pub serial_device: String,
    /// The baud rate for the serial connection.
    pub baud_rate: u32,
    /// The timeout for the serial connection in milliseconds.
    pub timeout: u64,
    /// The number of ticks per revolution of the motor.
    pub motor_ticks_per_revolution: u64,
}

/// Hardware abstraction layer (HAL) for the robot.
///
/// It abstracts the details of the hardware communication and provides methods to control
/// the robot's motors and read sensor values.
#[derive(Debug)]
pub struct Hal {
    /// The serial connection to the hardware.
    hw_serial_connection: HwSerialConnection,
    /// Right wheel instance.
    right_wheel: Wheel,
    /// Left wheel instance.
    left_wheel: Wheel,
}

/// The state of the hardware abstraction layer (HAL).
#[derive(Debug)]
pub struct HalState {
    /// The state of the right wheel.
    pub right_wheel_state: WheelState,
    /// The state of the left wheel.
    pub left_wheel_state: WheelState,
}

impl Hal {
    /// Creates a new instance of the hardware abstraction layer (HAL).
    ///
    /// # Arguments
    ///  - `hal_config` - The configuration for the HAL.
    ///
    /// # Returns
    ///  - `Ok(Hal)` - A new instance of the HAL.
    /// - `Err(HalError)` - An error if the HAL fails to initialize.
    pub fn new(hal_config: &HalConfig) -> Result<Self, HalError> {
        let hw_serial_connection =
            HwSerialConnection::new(&hal_config.serial_device, hal_config.baud_rate, hal_config.timeout)?;
        Ok(Hal {
            hw_serial_connection,
            right_wheel: Wheel::new(hal_config.motor_ticks_per_revolution),
            left_wheel: Wheel::new(hal_config.motor_ticks_per_revolution),
        })
    }

    /// Reads sensor values and updates the state of the sensors in the HAL.
    ///
    /// This method is called periodically to update the state of the sensors.
    /// Consider using a timer or a loop to call this method at regular intervals as
    /// there are calculations that depend on the time elapsed since the last update.
    ///
    /// # Arguments
    ///
    /// * `delta_time` - The time elapsed since the last update in seconds.
    ///
    /// # Returns
    ///
    /// * `Ok(HalState)` - The state of the HAL after the update.
    /// * `Err(HalError)` - An error if the update fails.
    pub fn poll_state(&mut self, delta_time: f64) -> Result<HalState, HalError> {
        // Poll the state of the wheels and update their state.
        let wheels_state = self.update_wheels_state(delta_time)?;
        // Compose the HAL state.
        let hal_state = HalState {
            right_wheel_state: wheels_state.0,
            left_wheel_state: wheels_state.1,
        };
        Ok(hal_state)
    }

    /// Sets the speed of the motors in rads per second.
    ///
    /// # Arguments
    ///
    /// * `left_speed` - The speed of the left motor in rads per second.
    /// * `right_speed` - The speed of the right motor in rads per second.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the command was sent successfully.
    /// * `Err(HalError)` - An error if the command fails.
    pub fn set_motor_speed(&mut self, left_speed: f64, right_speed: f64) -> Result<(), HalError> {
        // Convert the speed from rads/sec to ticks/sec using the rads per tick (rpt) of the motor:
        // ticks/sec = rads/sec * ticks/rads
        let left_value_target = (left_speed * self.left_wheel.ticks_per_rad() as f64).round() as i64;
        let right_value_target = (right_speed * self.right_wheel.ticks_per_rad() as f64).round() as i64;
        log::trace!(
            "Sending command to set motor speed[ticks per second]: left: {} right: {}",
            left_value_target,
            right_value_target
        );
        self.hw_serial_connection
            .send_command(super::comm::SerialCommands::SetMotorValues {
                left: left_value_target,
                right: right_value_target,
            })?;

        Ok(())
    }

    /// Updates the state of the wheels by reading the encoder values from the hardware.
    ///
    /// # Arguments
    /// * `delta_time` - The time elapsed since the last update in seconds.
    ///
    /// # Returns
    /// * `Ok((WheelState, WheelState))` - The state of the left and right wheels after the update.
    /// * `Err(HalError)` - An error if the update fails.
    fn update_wheels_state(
        &mut self,
        delta_time: f64,
    ) -> Result<(super::sensors::WheelState, super::sensors::WheelState), HalError> {
        let response = self
            .hw_serial_connection
            .send_command(super::comm::SerialCommands::ReadEncoderValues)?;
        if let super::comm::SerialResponse::EncoderValues { left, right } = response {
            Ok((
                self.left_wheel.update(left, delta_time).clone(),
                self.right_wheel.update(right, delta_time).clone(),
            ))
        } else {
            Err(HalError::HardwareCommunicationError(
                HwSerialConnectionError::WrongResponseError {
                    error: "Invalid response to a ReadEncoderValues from hardware".to_string(),
                },
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hal_new_failing() {
        let hal_config = HalConfig {
            serial_device: String::from("/hope/invalid/path"),
            baud_rate: 57600,
            timeout: 3000,
            motor_ticks_per_revolution: 360,
        };
        let hal = Hal::new(&hal_config);
        assert!(hal.is_err());
        if let Err(err) = hal {
            assert!(matches!(err, HalError::HardwareCommunicationError(_)));
        }
    }
}
