# andino_dora_sim

Dora integration of Andino Simulation

## Graphs

### mujoco_teleoperation.yml

Runs a dataflow for running the andino MuJoCo simulation and teleoperate the robot.

```mermaid
        flowchart TB
  dora_andino_mujoco_sim["**dora_andino_mujoco_sim**"]
  dora_diff_drive_controller["**dora_diff_drive_controller**"]
  dora_keyboard[\"**dora_keyboard**"/]
  dora_teleop_keyboard["**dora_teleop_keyboard**"]
subgraph ___dora___ [dora]
  subgraph ___timer_timer___ [timer]
    dora/timer/millis/1[\millis/1/]
  end
end
  dora_diff_drive_controller -- joints_speed_cmd --> dora_andino_mujoco_sim
  dora/timer/millis/1 -- tick --> dora_andino_mujoco_sim
  dora_teleop_keyboard -- cmd_vel --> dora_diff_drive_controller
  dora_keyboard -- char as key --> dora_teleop_keyboard
```

Build the dataflow:
```
dora build graphs/mujoco_teleoperation.yml
```

Run the dataflow locally:
```
dora run graphs/mujoco_teleoperation.yml
```
