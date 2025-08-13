# dora_andino_mujoco_sim

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
  - viewer_tick: Timer to update viewer of MuJoCo.
  - model_tick: Timer to step MuJoCo model. Pair with `TIMESTEP` env var.
  - joints_speed_cmd: Joints speed commands for the left and right wheel. [rad/s]

### outputs
  - wheel_joint_positions: Feedback info about left and right wheel joint position. [rads]
  - wheel_joint_velocities: Feedback info about left and right wheel joint velocity. [rads/s]

### envs
  - TIMESTEP: Time step for the MuJoCo simulation.

## Examples

```yml
nodes:
  # Loads andino simulation in MuJoCo. This replaces the andino HAL with a simulated version.
  # If we were to replace this node with dora_andino_hal node, it should be able to run the dataflow.
  - id: dora_andino_mujoco_sim
    build: uv pip install -e .
    path: dora_andino_mujoco_sim
    inputs:
      viewer_tick: dora/timer/millis/17 # ~60 Hz
      model_tick: dora/timer/millis/1 # ~1000Hz
      joints_speed_cmd: dora_diff_drive_controller/joints_speed_cmd
    outputs:
      - wheel_joint_positions # [left, right, timestamp]
      - wheel_joint_velocities # [left, right, timestamp]
    env:
      TIMESTEP: 0.001 # [s]
```

## License

dora_andino_mujoco_sim's code are released under the MIT License
