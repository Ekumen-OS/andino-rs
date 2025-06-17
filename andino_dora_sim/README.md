# andino_dora_sim

Dora integration of Andino Simulation

## Graphs

### mujoco_sim.yml

<p align="center">
  <img src="docs/mujoco_sim.png"/>
</p>

Runs a dataflow for running the andino MuJoCo simulation along with:
 - keyboard teleoperation of the robot.
 - object detection using YOLOv8.
 -  `rerun` visualization.

```mermaid
        flowchart TB
  dora_andino_mujoco_sim["**dora_andino_mujoco_sim**"]
  dora_diff_drive_controller["**dora_diff_drive_controller**"]
  dora_keyboard[\"**dora_keyboard**"/]
  dora_teleop_keyboard["**dora_teleop_keyboard**"]
  object-detection["**object-detection**"]
  rerun-viz[/"**rerun-viz**"\]
subgraph ___dora___ [dora]
  subgraph ___timer_timer___ [timer]
    dora/timer/millis/1[\millis/1/]
    dora/timer/millis/17[\millis/17/]
    dora/timer/millis/100[\millis/100/]
  end
end
  dora/timer/millis/100 -- camera_tick --> dora_andino_mujoco_sim
  dora_diff_drive_controller -- joints_speed_cmd --> dora_andino_mujoco_sim
  dora/timer/millis/1 -- model_tick --> dora_andino_mujoco_sim
  dora/timer/millis/17 -- viewer_tick --> dora_andino_mujoco_sim
  dora_teleop_keyboard -- cmd_vel --> dora_diff_drive_controller
  dora_keyboard -- char as key --> dora_teleop_keyboard
  dora_andino_mujoco_sim -- camera_image as image --> object-detection
  object-detection -- bbox as boxes2d --> rerun-viz
  dora_andino_mujoco_sim -- camera_image as image --> rerun-viz
```

Build the dataflow:
```
dora build graphs/mujoco_sim.yml
```

Run the dataflow locally:
```
dora run graphs/mujoco_sim.yml
```
