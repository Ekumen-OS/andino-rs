# dora_andino_mujoco_sim

## Getting started

- Install it with uv:

```bash
uv venv -p 3.10 --seed
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

## Examples

```yml
nodes:
  # Loads andino simulation in MuJoCo. This replaces the andino HAL with a simulated version.
  # If we were to replace this node with dora_andino_hal node, it should be able to run the dataflow.
  - id: dora_andino_mujoco_sim
    build: pip install -e .
    path: dora_andino_mujoco_sim
    inputs:
      tick: dora/timer/millis/10
      joints_speed_cmd: dora_diff_drive_controller/joints_speed_cmd
    outputs:
      - wheel_joint_positions # [left, right]
      - wheel_joint_velocities # [left, right]
```

## License

dora_andino_mujoco_sim's code are released under the MIT License
