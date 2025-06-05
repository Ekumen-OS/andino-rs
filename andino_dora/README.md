# andino_dora

Dora integration of Andino platform

## Graphs

### dataflow.yml

Teleoperate andino robot with the keyboard.

```mermaid
        flowchart TB
  dora_andino_hal["**dora_andino_hal**"]
  dora_diff_drive_controller["**dora_diff_drive_controller**"]
  dora_keyboard[\"**dora_keyboard**"/]
  dora_teleop_keyboard["**dora_teleop_keyboard**"]
subgraph ___dora___ [dora]
  subgraph ___timer_timer___ [timer]
    dora/timer/millis/100[\millis/100/]
  end
end
  dora_diff_drive_controller -- joints_speed_cmd --> dora_andino_hal
  dora/timer/millis/100 -- tick --> dora_andino_hal
  dora_teleop_keyboard -- cmd_vel --> dora_diff_drive_controller
  dora_keyboard -- char as key --> dora_teleop_keyboard
```

Build the `andino_dora`'s dataflow
```
dora build graphs/dataflow.yml
```

Run the dataflow locally:
```
dora run graphs/dataflow.yml
```

### object_detection.yml

Runs a dataflow to run object detection algorithm.

```mermaid
        flowchart TB
  camera["**camera**"]
  object-detection["**object-detection**"]
  rerun-viz[/"**rerun-viz**"\]
subgraph ___dora___ [dora]
  subgraph ___timer_timer___ [timer]
    dora/timer/millis/100[\millis/100/]
  end
end
  dora/timer/millis/100 -- tick --> camera
  camera -- image --> object-detection
  object-detection -- bbox as boxes2d --> rerun-viz
  camera -- image --> rerun-viz
```

Build the dataflow:
```
dora build graphs/object_detection.yml
```

Run the dataflow locally:
```
dora run graphs/object_detection.yml
```

Run the Rerun server locally (not on the andino):
```
rerun --serve --web-viewer --bind <your_ip>
```

### sim_teleoperation.yml

TODO
