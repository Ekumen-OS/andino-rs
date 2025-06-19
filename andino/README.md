# andino

Rust-based Hardware Abstraction Layer (HAL) for [andino](https://github.com/Ekumen-OS/andino).

## Build

```
cargo build
```

## Pre-requisites

 - You need a built andino! Refer to [andino_hardware](https://github.com/Ekumen-OS/andino/tree/humble/andino_hardware) to setup everything.

 - You can partially test the HAL by using only an Arduino Nano, correctly loaded with the firmware. See [andino_firmware](htthttps://github.com/Ekumen-OS/andino/tree/humble/andino_firmware) for further reference.

## Examples

 - *01_available_serial_ports*: Verify the available serial ports.

    ```
    cargo run --example 01_available_serial_ports
    ```

 - *02_hardware_serial_connection*: Communicate with the underlying hardware via serial port.

    ```
    cargo run --example 02_hardware_serial_connection
    ```

 - *03_hal_interface*: CLI for using hal interface to communicate with underlying hardware. This allows teleoperation of the robot.

    ```
    cargo run --example 03_hal_interface
    ```
