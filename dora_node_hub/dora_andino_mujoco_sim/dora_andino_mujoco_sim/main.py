"""Main entry point for the Andino MuJoCo simulation dora node."""

import pyarrow as pa
from dora import Node
import mujoco
import mujoco.viewer


def get_scene_path():
    """Get the path to the MuJoCo scene file."""
    import os
    root_dir = os.path.dirname(os.path.abspath(__file__))
    scene_path = os.path.join(root_dir, "assets/scene.xml")
    if not os.path.exists(scene_path):
        raise FileNotFoundError(
            f"Scene file not found at {scene_path}. "
            "Please ensure the assets directory is present."
        )
    return scene_path

def main():
    """Execute the Andino MuJoCo simulation dora node."""
    try:

      node = Node("Andino MuJoCo Simulation")

      mj_model = mujoco.MjModel.from_xml_path(get_scene_path())
      mj_model.opt.timestep = 1e-3  # 1ms
      mj_data = mujoco.MjData(mj_model)

      # Andino left and right wheel joint speeds commands.
      left_joint_speed_cmd = 0.
      right_joint_speed_cmd = 0.
      with mujoco.viewer.launch_passive(mj_model, mj_data) as viewer:

        for event in node:
            if event["type"] == "INPUT":
                if event["id"] == "tick":
                    # Pass the joint speeds to the MuJoCo data structure.
                    mj_data.actuator("left-motor").ctrl[0] = left_joint_speed_cmd
                    mj_data.actuator("right-motor").ctrl[0] = right_joint_speed_cmd

                    # Advance the simulation
                    mujoco.mj_step(mj_model, mj_data)
                    viewer.sync()

                    # Send position and velocity data or any other form of feedback from MuJoCo
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
