"""TODO: Add docstring."""

import os

import numpy as np
import pyarrow as pa
from dora import Node
import cv2 as cv
import time
from dora_andino_gemini_control.controller import DiffDriveGeminiControl


def main() -> None:
    """TODO: Add docstring."""
    node = Node()

    # Required node configuration
    command = os.getenv("COMMAND", "")
    if not command:
        raise ValueError(
            "COMMAND environment variable is not set. Please provide a command."
        )
    # Optional node configuration
    model = os.getenv("MODEL", "gemini-2.0-flash-lite")

    # Initialize the controller with the specified model
    controller = DiffDriveGeminiControl(model=model)
    last_image = None
    cmd_vel = np.array(
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0], dtype=np.float64
    )  # Initialize cmd_vel with zeros
    for event in node:
        event_type = event["type"]
        if event_type == "INPUT":
            event_id = event["id"]
            if event_id == "image":
                storage = event["value"]
                metadata = event["metadata"]
                encoding = metadata["encoding"]
                width = metadata["width"]
                height = metadata["height"]

                if encoding in {"bgr8", "rgb8"}:
                    channels = 3
                    storage_type = np.uint8
                else:
                    raise RuntimeError(f"Unsupported image encoding: {encoding}")

                frame = (
                    storage.to_numpy()
                    .astype(storage_type)
                    .reshape((height, width, channels))
                )
                if encoding == "bgr8":
                    pass
                elif encoding == "rgb8":
                    frame = frame[:, :, ::-1]  # Convert RGB to BGR
                else:
                    raise RuntimeError(f"Unsupported image encoding: {encoding}")

                # Convert the frame to png
                ret, frame = cv.imencode(".png", frame)
                if not ret:
                    raise RuntimeError("Failed to encode image to PNG format.")
                last_image = frame
            if event_id == "tick":
                if last_image is None:
                    continue
                image_bytes = last_image.tobytes()

                # Dump image into a file for debugging
                with open("last_image.png", "wb") as f:
                    f.write(last_image)

                now = time.time()
                velocities = controller.generate_velocities(
                    command=command, image_bytes=image_bytes
                )
                last_image = None
                print(f"Time taken to generate velocities: {time.time() - now:.4f} seconds")
                print(f"Generated velocities: {velocities}")
                cmd_vel = velocities
            if event_id == "cmd_vel_tick":
                node.send_output(
                    "cmd_vel",
                    data=pa.array(cmd_vel.tolist(), type=pa.float64()),
                    metadata=event["metadata"],
                )


if __name__ == "__main__":
    main()
