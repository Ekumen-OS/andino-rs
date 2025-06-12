"""Main entry point for the Andino MuJoCo simulation dora node."""

import os
import importlib.resources as resources

import pyarrow as pa
import mujoco
import mujoco.viewer
from dora import Node

def get_scene_path():
    """Get the path to the MuJoCo scene file."""
    scene_path = resources.files("dora_andino_mujoco_sim").joinpath("assets", "scene.xml")
    if not scene_path.is_file():
        raise FileNotFoundError(
            f"Scene file not found at {scene_path}. "
            "Please ensure the assets directory is present."
        )
    return str(scene_path)

def get_timestep_config():
    """Get the timestep configuration for the MuJoCo simulation."""
    timestep = os.getenv("TIMESTEP", "0.001")
    try:
        timestep = float(timestep)
    except ValueError:
        raise ValueError(f"Invalid TIMESTEP value: {timestep}. Must be a float.")
    return timestep

def main():
    """Execute the Andino MuJoCo simulation dora node."""
    try:

      node = Node("Andino MuJoCo Simulation")

      # Load the MuJoCo model from the XML file.
      mj_model = mujoco.MjModel.from_xml_path(get_scene_path())
      mj_model.opt.timestep = get_timestep_config()
      mj_data = mujoco.MjData(mj_model)

      # Andino left and right wheel joint speeds commands.
      left_joint_speed_cmd = 0.
      right_joint_speed_cmd = 0.
      with mujoco.viewer.launch_passive(mj_model, mj_data) as viewer:

        for event in node:
            if event["type"] == "INPUT":
                if event["id"] == "viewer_tick":
                    # Handle viewer tick events to update the viewer.
                    viewer.sync()
                if event["id"] == "model_tick":
                    # Pass the joint speeds to the MuJoCo data structure.
                    mj_data.actuator("left-motor").ctrl[0] = left_joint_speed_cmd
                    mj_data.actuator("right-motor").ctrl[0] = right_joint_speed_cmd

                    # Advance the simulation
                    mujoco.mj_step(mj_model, mj_data)

                    # Send position and velocity data or any other form of feedback from MuJoCo.
                    # Replicating same behavior as the dora_andino_hal we outputs:
                    # - wheel_joint_positions
                    # - wheel_joint_velocities
                    wheel_joint_positions = [
                        mj_data.qpos[0],
                        mj_data.qpos[1],
                    ]
                    wheel_joint_velocities = [
                        mj_data.qvel[0],
                        mj_data.qvel[1],
                    ]
                    node.send_output(
                        "wheel_joint_positions",
                        data=pa.array(wheel_joint_positions, type=pa.float64()),
                        metadata=event["metadata"],
                    )
                    node.send_output(
                        "wheel_joint_velocities",
                        data=pa.array(wheel_joint_velocities, type=pa.float64()),
                        metadata=event["metadata"],
                    )
                if event["id"] == "joints_speed_cmd":

                    # Extract the joints speed for the joint wheels:
                    # [left_joint_speed, right_joint_speed] in radians per second
                    left_joint_speed_cmd, right_joint_speed_cmd = event["value"].to_pylist()

    except KeyboardInterrupt:
        print("\nExiting simulation...")
    except Exception as e:
        print(f"Simulation error: {e}")
        raise e


if __name__ == "__main__":
    main()
