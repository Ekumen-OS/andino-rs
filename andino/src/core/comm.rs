// ***************************************************************************
// About
// ***************************************************************************
//
//! Module for communication with the underlying hardware via serial port.
//! For further details regarding the hardware take a look at the
//! firmware code at <https://github.com/Ekumen-OS/andino/tree/humble/andino_firmware>

use thiserror::Error;

/// Error type for the serial connection.
#[derive(Debug, Error, PartialEq)]
pub enum HwSerialConnectionError {
    #[error("Serial port connection error: {error}")]
    /// File system error.
    SerialPortConnectionError { error: String },
    #[error("Serial port read error: {error}")]
    /// File system error.
    WrongResponseError { error: String },
}

impl From<std::io::Error> for HwSerialConnectionError {
    fn from(source: std::io::Error) -> Self {
        HwSerialConnectionError::SerialPortConnectionError {
            error: source.to_string(),
        }
    }
}

/// Enum representing the commands that can be sent to the serial connection.
#[derive(Debug)]
pub enum SerialCommands {
    /// Command to read encoder values.
    ReadEncoderValues,
    /// Command to set motor values. (encoder ticks per second)
    SetMotorValues { left: i64, right: i64 },
    /// Command to modify PID values of the motor controller.
    SetPIDValues { kp: f32, ki: f32, kd: f32, ko: f32 },
}

/// Enum representing the response from the serial connection.
#[derive(Debug)]
pub enum SerialResponse {
    /// Response containing the encoder values.
    EncoderValues { left: i64, right: i64 },
    /// Response containing a message
    Other { message: String },
}

/// Abstracts the serial connection to the underlying hardware.
/// This struct is used to send commands to the hardware and receive responses.
/// It uses the `serialport` crate to handle the serial communication.
#[derive(Debug)]
pub struct HwSerialConnection {
    serial_port: Box<dyn serialport::SerialPort>,
}

impl HwSerialConnection {
    /// Creates a new instance of `HwSerialConnection`.
    ///
    /// # Arguments
    ///
    /// * `serial_device` - The name of the serial device to connect to.
    /// * `baud_rate` - The baud rate for the serial connection.
    /// * `timeout` - The timeout for the serial connection in milliseconds.
    ///
    /// # Returns
    ///
    /// * `Ok(HwSerialConnection)` - A new instance of `HwSerialConnection`.
    /// * `Err(HwSerialConnectionError)` - An error if the connection fails.
    ///
    pub fn new(serial_device: impl AsRef<str>, baud_rate: u32, timeout: u64) -> Result<Self, HwSerialConnectionError> {
        let serial_port = serialport::new(serial_device.as_ref(), baud_rate)
            // Set the serial port parameters
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .data_bits(serialport::DataBits::Eight)
            .flow_control(serialport::FlowControl::None)
            .timeout(std::time::Duration::from_millis(timeout))
            // Open the serial port
            .open()
            .map_err(|e| HwSerialConnectionError::SerialPortConnectionError { error: e.to_string() })?;
        log::trace!("Serial port opened: {}", serial_device.as_ref());
        Ok(HwSerialConnection { serial_port })
    }

    /// Sends a command to the serial connection and returns the raw response.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to send to the serial connection.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The response from the serial connection.
    /// * `Err(HwSerialConnectionError)` - An error if the command fails.
    ///
    pub fn send_command(&mut self, command: SerialCommands) -> Result<SerialResponse, HwSerialConnectionError> {
        let command_str = HwSerialConnection::prepare_command_to_send(&command);
        log::trace!("Sending command: {}", command_str);
        // Send the command to the serial port
        self.serial_port.write_all(command_str.as_bytes())?;

        let mut response_buffer = vec![0; 32];
        log::trace!("Reading response from serial port");
        let n = self.serial_port.read(&mut response_buffer)?;
        let response_str = String::from_utf8_lossy(&response_buffer[..n]).to_string();
        log::trace!("Received response: {}", response_str);
        HwSerialConnection::parse_response(&command, response_str)
    }

    /// Prepares the command to be sent to the serial connection.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to send.
    ///
    /// # Returns
    ///
    /// * `String` - The command string ready to be sent to the serial connection.
    pub(crate) fn prepare_command_to_send(command: &SerialCommands) -> String {
        let command_str = match command {
            SerialCommands::ReadEncoderValues => "e".to_string(),
            SerialCommands::SetMotorValues { left, right } => format!("m {} {}", left, right),
            SerialCommands::SetPIDValues { kp, ki, kd, ko } => format!("u {}:{}:{}:{}", kp, ki, kd, ko),
        }
        // Add carriage return to the message.
        + "\r";
        log::trace!("Preparing command to send: {}", command_str);
        command_str
    }

    /// Parses the response from the serial connection.
    ///
    /// # Arguments
    ///
    /// * `command` - The command that was sent to the serial connection. It is used to determine how to parse the response based on the command type.
    /// * `response` - The response from the serial connection.
    ///
    /// # Returns
    ///
    /// * `Ok(SerialResponse)` - The parsed response from the serial connection.
    /// * `Err(HwSerialConnectionError)` - An error if the response is invalid.
    pub(crate) fn parse_response(
        command: &SerialCommands,
        response: String,
    ) -> Result<SerialResponse, HwSerialConnectionError> {
        // Verify the response is not empty
        if response.is_empty() {
            return Err(HwSerialConnectionError::WrongResponseError {
                error: "Empty response from serial port".to_string(),
            });
        }
        use itertools::Itertools;
        match command {
            SerialCommands::ReadEncoderValues => {
                let splitted_response: Option<(&str, &str)> = response.split_whitespace().collect_tuple();
                let values = if let Some(values) = splitted_response {
                    values
                } else {
                    return Err(HwSerialConnectionError::WrongResponseError {
                        error: "Invalid response format for encoder values: ".to_string() + response.as_str(),
                    });
                };
                let left = values
                    .0
                    .parse::<i64>()
                    .map_err(|e| HwSerialConnectionError::WrongResponseError { error: e.to_string() })?;
                let right = values
                    .1
                    .parse::<i64>()
                    .map_err(|e| HwSerialConnectionError::WrongResponseError { error: e.to_string() })?;
                Ok(SerialResponse::EncoderValues { left, right })
            }
            _ => Ok(SerialResponse::Other { message: response }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HwSerialConnection;
    use super::HwSerialConnectionError;
    use super::SerialCommands;
    use super::SerialResponse;

    #[test]
    fn test_prepare_command_to_send_read_encoders() {
        let command = SerialCommands::ReadEncoderValues;
        let command_str = HwSerialConnection::prepare_command_to_send(&command);
        assert_eq!(command_str, "e\r");
    }

    #[test]
    fn test_prepare_command_to_send_set_motor_values() {
        let command = SerialCommands::SetMotorValues { left: 100, right: 200 };
        let command_str = HwSerialConnection::prepare_command_to_send(&command);
        assert_eq!(command_str, "m 100 200\r");
    }

    #[test]
    fn test_prepare_command_to_send_set_pid_values() {
        let command = SerialCommands::SetPIDValues {
            kp: 1.0,
            ki: 2.0,
            kd: 3.0,
            ko: 4.0,
        };
        let command_str = HwSerialConnection::prepare_command_to_send(&command);
        assert_eq!(command_str, "u 1:2:3:4\r");
    }

    #[test]
    fn test_parse_response_encoders() {
        let response = "123 456".to_string();
        let command = SerialCommands::ReadEncoderValues;
        let parsed_response = HwSerialConnection::parse_response(&command, response).unwrap();
        match parsed_response {
            SerialResponse::EncoderValues { left, right } => {
                assert_eq!(left, 123);
                assert_eq!(right, 456);
            }
            _ => panic!("Expected EncoderValues response"),
        }
    }
    #[test]
    fn test_parse_response_encoders_error() {
        let response = "123 456 789".to_string();
        let command = SerialCommands::ReadEncoderValues;
        let parsed_response = HwSerialConnection::parse_response(&command, response.clone());
        assert!(parsed_response.is_err());
        assert_eq!(
            parsed_response.unwrap_err(),
            HwSerialConnectionError::WrongResponseError {
                error: "Invalid response format for encoder values: ".to_string() + response.as_str()
            }
        );
    }
    #[test]
    fn test_parse_response_other() {
        let response = "Random Msg".to_string();
        let command = SerialCommands::SetMotorValues { left: 0, right: 0 };
        let parsed_response = HwSerialConnection::parse_response(&command, response).unwrap();
        match parsed_response {
            SerialResponse::Other { message } => {
                assert_eq!(message, "Random Msg");
            }
            _ => panic!("Expected Other response"),
        }
    }
}
