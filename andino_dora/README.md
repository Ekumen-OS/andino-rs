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

### gemini_navigation.yml

```mermaid
        flowchart TB
  camera["**camera**"]
  dora_andino_hal["**dora_andino_hal**"]
  dora_diff_drive_controller["**dora_diff_drive_controller**"]
  dora_gemini_diff_drive_navigation["**dora_gemini_diff_drive_navigation**"]
  dora_string_publisher_ui[\"**dora_string_publisher_ui**"/]
  rerun-viz[/"**rerun-viz**"\]
subgraph ___dora___ [dora]
  subgraph ___timer_timer___ [timer]
    dora/timer/millis/100[\millis/100/]
  end
end
  dora/timer/millis/100 -- tick --> camera
  dora_diff_drive_controller -- joints_speed_cmd --> dora_andino_hal
  dora/timer/millis/100 -- tick --> dora_andino_hal
  dora_gemini_diff_drive_navigation -- cmd_vel --> dora_diff_drive_controller
  dora/timer/millis/100 -- cmd_vel_tick --> dora_gemini_diff_drive_navigation
  dora_string_publisher_ui -- submitted_string as command --> dora_gemini_diff_drive_navigation
  camera -- image --> dora_gemini_diff_drive_navigation
  dora/timer/millis/100 -- tick --> dora_gemini_diff_drive_navigation
  camera -- image --> rerun-viz
```

Runs a dataflow that uses [Gemini](https://gemini.google.com/app) API for navigating the Andino based on user inputs.

1. Obtain a [Gemini API Key](https://aistudio.google.com/apikey) and add it as environment variable to the correspondent node in the [gemini_navigation.yml](graphs/gemini_navigation.yml) dataflow.

2. Build the dataflow:
```
dora build graphs/gemini_navigation.yml
```

3. Run the dataflow locally:
```
dora run graphs/gemini_navigation.yml
```

4. Run the Rerun server locally (not on the *andino*):
```
rerun --serve --web-viewer --bind <your_ip>
```

5. Use the GUI to input a command to be followed by the Andino.
