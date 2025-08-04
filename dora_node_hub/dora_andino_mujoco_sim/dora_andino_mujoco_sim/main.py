"""Main entry point for the Andino MuJoCo simulation dora node."""

import os
from importlib import resources

import mujoco
import mujoco.viewer
import numpy as np
import pyarrow as pa
from dora import Node

MUJOCO_ANDINO_CAMARA_NAME = "andino_cam"


def get_scene_path() -> str:
    """Get the path to the MuJoCo scene file."""
    scene_path = resources.files("dora_andino_mujoco_sim").joinpath("assets/scene.xml")
    if not scene_path.is_file():
        raise FileNotFoundError(f"Scene file not found at {scene_path}. Please ensure the assets directory is present.")
    return str(scene_path)


def get_timestep_config() -> float:
    """Get the timestep configuration for the MuJoCo simulation."""
    try:
        timestep = float(os.getenv("TIMESTEP", "0.001"))
    except ValueError:
        raise ValueError from ValueError(f"Invalid TIMESTEP value: {timestep}. Must be a float.")
    return timestep


def convert_rgb_to_bgr(frame_rgb: np.ndarray) -> np.ndarray:
    """Convert an RGB frame to BGR format.

    This is useful for compatibility with OpenCV and other libraries that expect BGR format.

    Args:
        frame_rgb (numpy.ndarray): Input frame in RGB format (H, W, 3).

    Returns:
        numpy.ndarray: Frame converted to BGR format (H, W, 3).

    """
    CHANNEL_DIMENSION = 3
    if frame_rgb.ndim == CHANNEL_DIMENSION and frame_rgb.shape[2] == CHANNEL_DIMENSION:
        return frame_rgb[:, :, ::-1]
    raise ValueError("Input frame must be a 3-channel RGB image.")


def main() -> None:
    """Execute the Andino MuJoCo simulation dora node."""
    try:
        node = Node("Andino MuJoCo Simulation")

        # Load the MuJoCo model from the XML file.
        mj_model = mujoco.MjModel.from_xml_path(get_scene_path())
        mj_model.opt.timestep = get_timestep_config()
        mj_data = mujoco.MjData(mj_model)
        mj_renderer = mujoco.Renderer(mj_model)

        # Andino left and right wheel joint speeds commands.
        left_joint_speed_cmd = 0.0
        right_joint_speed_cmd = 0.0
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
                            mj_data.joint("left_wheel_joint").qpos[0],
                            mj_data.joint("right_wheel_joint").qpos[0],
                        ]

                        wheel_joint_velocities = [
                            mj_data.joint("left_wheel_joint").qvel[0],
                            mj_data.joint("right_wheel_joint").qvel[0],
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
                    if event["id"] == "camera_tick":
                        mj_renderer.update_scene(mj_data, camera=MUJOCO_ANDINO_CAMARA_NAME)
                        frame = convert_rgb_to_bgr(mj_renderer.render())
                        metadata = event["metadata"]
                        metadata["encoding"] = "bgr8"
                        metadata["width"] = int(frame.shape[1])
                        metadata["height"] = int(frame.shape[0])
                        # Send the camera image as a flattened array.
                        # This is compatible with the camera image output format used for the real robot.
                        node.send_output(
                            "camera_image",
                            data=pa.array(frame.ravel(), type=pa.uint8()),
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
