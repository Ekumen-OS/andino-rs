# dora_string_publisher_ui

Dora node for publishing a string text using a GUI based on [eframe](https://docs.rs/eframe/latest/eframe/).

## Getting started

- Install it with cargo:

```bash
cargo build --package dora_string_publisher_ui
```

## YAML Specification

### inputs
  *None*
### outputs
  - ***submitted_string***: Text entered using the GUI.

### envs
  *None*

## Examples

```yml
nodes:
  - id: dora_string_publisher_ui
    build: cargo build -p dora_string_publisher_ui
    path: ../target/debug/dora_string_publisher_ui
    inputs:
    outputs:
      - submitted_string # [StringArray]

```
