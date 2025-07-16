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

### mujoco_sim_gemini_navigation.yml

```mermaid
        flowchart TB
  dora_andino_mujoco_sim["**dora_andino_mujoco_sim**"]
  dora_diff_drive_controller["**dora_diff_drive_controller**"]
  dora_gemini_diff_drive_navigation["**dora_gemini_diff_drive_navigation**"]
  dora_string_publisher_ui[\"**dora_string_publisher_ui**"/]
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
  dora_gemini_diff_drive_navigation -- cmd_vel --> dora_diff_drive_controller
  dora/timer/millis/100 -- cmd_vel_tick --> dora_gemini_diff_drive_navigation
  dora_string_publisher_ui -- submitted_string as command --> dora_gemini_diff_drive_navigation
  dora_andino_mujoco_sim -- camera_image as image --> dora_gemini_diff_drive_navigation
  dora/timer/millis/100 -- tick --> dora_gemini_diff_drive_navigation
  dora_andino_mujoco_sim -- camera_image as image --> rerun-viz
```

Runs a dataflow that uses [Google's Gemini](https://gemini.google.com/app) API to provide navigation capabilities to Andino.

1. Obtain a [Gemini API Key](https://aistudio.google.com/apikey) and add it as an environment variable to the corresponding node in the [mujoco_sim_gemini_navigation.yml](graphs/mujoco_sim_gemini_navigation.yml) dataflow.

2. Build the dataflow:
```
dora build graphs/mujoco_sim_gemini_navigation.yml
```

3. Run the dataflow locally:
```
dora run graphs/mujoco_sim_gemini_navigation.yml
```

4. Use the GUI to issue a command for Andino to execute.
