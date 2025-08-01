# dora_gemini_diff_drive_navigation

Relies on Google's LLM model: [Gemini](https://gemini.google.com/) for navigating a differential drive robot.

## Getting started

- Install it with uv:

```bash
uv venv -p 3.11 --seed
uv pip install -e .
```

## Contribution Guide

- Format with [ruff](https://docs.astral.sh/ruff/):

```bash
uv pip install ruff
uv run ruff check . --fix
```

- Lint with ruff:

```bash
uv run ruff check .
```

- Test with [pytest](https://github.com/pytest-dev/pytest)

```bash
uv pip install pytest
uv run pytest . # Test
```

## YAML Specification

### inputs
  - `tick`: Rate at which the control loop is attempted to be run.
  - `cmd_vel_tick`: Rate at which the command velocity should be published.
  - `image`: Camera image to be used as input for the prompt.
  - `command`: Command to be prompted to Gemini's API.

### outputs
  - `cmd_vel`: Velocity command to be forwarded to a differential drive controller.

### envs
  - `COMMAND`: Optional static command. You can use this command if no *command* is set as *input*. If both (*COMMAND* env and *command* input) are set, this would act as initial command and then be replaced by the input.
  - `MODEL`: One of the valid Gemini models. (E.g: gemini-2.5-flash). Visit [ai.google.dev](https://ai.google.dev/gemini-api/docs) for more info.
  - `GEMINI_API_KEY`: Gemini API Key to be used for interacting with the API.

## Examples

```yml
  - id: dora_gemini_diff_drive_navigation
    build: uv pip install -e ../../dora_node_hub/dora_gemini_diff_drive_navigation
    path: dora_gemini_diff_drive_navigation
    inputs:
      tick: dora/timer/millis/100
      cmd_vel_tick: dora/timer/millis/100
      image: camera_image
      command: dora_string_publisher_ui/submitted_string
    outputs:
      - cmd_vel # [linear_vel, 0, 0, 0, 0, angular_vel]
    env:
      # COMMAND: # You can set up a static command if desired instead of passing the command as an input.
      MODEL: "gemini-2.5-flash"
      # Replace with your actual Gemini API key
      GEMINI_API_KEY: ""

  - id: dora_string_publisher_ui
    build: cargo build -p dora_string_publisher_ui
    path: ../../target/debug/dora_string_publisher_ui
    inputs:
    outputs:
      - submitted_string # [string]

  # Differential drive controller node.
  # This node takes the input command velocity (cmd_vel) [linear and angular velocity] and converts it to joint speed commands [rad/s] for the left and right wheels.
  - id: dora_diff_drive_controller
    build: cargo build -p dora_diff_drive_controller
    path: ../../target/debug/dora_diff_drive_controller
    inputs:
      cmd_vel: dora_gemini_diff_drive_navigation/cmd_vel
    outputs:
      - joints_speed_cmd # [left, right]
    env:
      WHEEL_RADIUS: 0.0315 # [m]
      WHEEL_SEPARATION: 0.137 # [m]
```
