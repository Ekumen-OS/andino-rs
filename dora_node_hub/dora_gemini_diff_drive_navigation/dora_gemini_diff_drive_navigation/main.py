"""Main entry point for the dora_gemini_diff_drive_navigation package."""

import asyncio
import os
from typing import Dict

import cv2 as cv
import numpy as np
import pyarrow as pa
from dora import Node

from dora_gemini_diff_drive_navigation.controller import DiffDriveGeminiControl


def zero_twist() -> np.ndarray:
    """Create a zero twist array."""
    return np.array([0.0, 0.0, 0.0, 0.0, 0.0, 0.0], dtype=np.float64)


def image_data_to_png(image_data: pa.UInt8Array, metadata: Dict[str, str]) -> bytes:
    """Convert image data from pyarrow UInt8Array to PNG format.

    The metadata should contain the encoding, width, and height of the image.
    For reference, this method can be used to convert images
    sent by https://github.com/dora-rs/dora/tree/main/node-hub/opencv-video-capture dora node.

    Args:
        image_data (pa.UInt8Array): The image data as a pyarrow UInt8Array
        metadata (dict): Metadata containing 'encoding', 'width', and 'height'.

    Returns:
        bytes: The image data in PNG format.

    """
    encoding = metadata["encoding"]
    width = metadata["width"]
    height = metadata["height"]
    if encoding in {"bgr8", "rgb8"}:
        channels = 3
        storage_type = np.uint8
    else:
        raise RuntimeError(f"Unsupported image encoding: {encoding}")

    frame = image_data.to_numpy().astype(storage_type).reshape((height, width, channels))
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
    return frame.tobytes()  # type: ignore[no-any-return]


async def main() -> None:
    """Main function to run the dora_gemini_diff_drive_navigation node."""
    node = Node()

    # Required node configuration
    command = os.getenv("COMMAND", "")
    if not command:
        print("COMMAND environment variable is not set. Using command via input event.")
    # Optional node configuration
    model = os.getenv("MODEL", "gemini-2.5-flash")

    # Initialize the controller with the specified model.
    controller = DiffDriveGeminiControl(model=model)
    last_image = None
    # Initialize cmd_vel with zero velocity.
    cmd_vel = zero_twist()

    while True:
        event = await node.recv_async()
        event_type = event["type"]
        if event_type == "INPUT":
            event_id = event["id"]
            if event_id == "image":
                storage = event["value"]
                metadata = event["metadata"]
                last_image = image_data_to_png(storage, metadata)
            if event_id == "tick":
                if last_image is None:
                    continue
                if command == "":
                    cmd_vel = zero_twist()
                    continue
                image_bytes = last_image

                # For debugging: Dump image into a file for debugging
                # with open("last_image.png", "wb") as f:
                #    f.write(last_image)

                generated_velocities_result = await controller.generate_velocities(
                    command=command, image_bytes=image_bytes
                )
                print(f"Generated velocities: {generated_velocities_result}")
                if command.lower() == "stop" and (np.all(generated_velocities_result == zero_twist())):
                    # To avoid keeping generating velocities in the next iteration when the command is "stop",
                    # we update the command to an empty string.
                    command = ""
                last_image = None
                cmd_vel = generated_velocities_result
            if event_id == "cmd_vel_tick":
                node.send_output(
                    "cmd_vel",
                    data=pa.array(cmd_vel.tolist(), type=pa.float64()),
                    metadata=event["metadata"],
                )
            if event_id == "command":
                value = event["value"]
                # value is a pa.array of type pa.string()
                if not isinstance(value, pa.Array):
                    raise RuntimeError(f"Expected value to be a pyarrow Array, got {type(value)}")
                if value.type != pa.string():
                    raise RuntimeError(f"Expected value to be a pyarrow string Array, got {value.type}")
                value = value.to_pylist()
                if len(value) > 1:
                    raise RuntimeError(f"Expected value to be a single string, got {value.len()} strings")
                command = str(value[0])

                print(f"Received command: {command}")
                if command == "":
                    print("Command is empty. Waiting for next input event.")
                    continue


loop = asyncio.get_event_loop()
loop.run_until_complete(main())
