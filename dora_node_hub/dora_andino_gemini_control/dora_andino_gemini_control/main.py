"""TODO: Add docstring."""

import asyncio
import os
import threading

import cv2 as cv
import numpy as np
import pyarrow as pa
from dora import Node

from dora_andino_gemini_control.controller import DiffDriveGeminiControl


def start_event_loop(loop: asyncio.AbstractEventLoop) -> None:
    """Starts the asyncio event loop in the background thread."""
    asyncio.set_event_loop(loop)
    loop.run_forever()


def zero_twist() -> np.ndarray:
    """Create a zero twist array."""
    return np.array([0.0, 0.0, 0.0, 0.0, 0.0, 0.0], dtype=np.float64)


def main() -> None:
    """TODO: Add docstring."""
    node = Node()

    # Required node configuration
    command = os.getenv("COMMAND", "")
    if not command:
        print("COMMAND environment variable is not set. Using command via input event.")
    # Optional node configuration
    model = os.getenv("MODEL", "gemini-2.0-flash-lite")

    # Initialize the controller with the specified model.
    controller = DiffDriveGeminiControl(model=model)
    last_image = None
    # Initialize cmd_vel with zero velocity.
    cmd_vel = zero_twist()

    # Setup for the background event loop thread
    loop = asyncio.new_event_loop()
    thread = threading.Thread(target=start_event_loop, args=(loop,), daemon=True)
    thread.start()

    generated_velocities_future = None

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
                if command == "":
                    cmd_vel = zero_twist()
                    continue
                image_bytes = last_image.tobytes()

                # For debugging: Dump image into a file for debugging
                # with open("last_image.png", "wb") as f:
                #    f.write(last_image)

                velocities = None
                if (
                    generated_velocities_future is not None
                    and generated_velocities_future.done()
                ):
                    # Velocities generation has finished. Retrieving result...
                    try:
                        velocities = generated_velocities_future.result()
                    except Exception as e:
                        print(f"❌ ERROR from velocities generation: {e}")
                    generated_velocities_future = None
                    print(f"Velocities generation future done: {velocities}")
                    if command.lower() == "stop" and (
                        np.all(velocities == zero_twist())
                    ):
                        # To avoid keeping generating velocities in the next iteration when the command is "stop",
                        # we update the command to an empty string.
                        command = ""

                elif generated_velocities_future is None:
                    # Velocities generation has not been run yet. Scheduling it now.
                    generated_velocities_future = asyncio.run_coroutine_threadsafe(
                        controller.generate_velocities(
                            command=command, image_bytes=image_bytes
                        ),
                        loop,
                    )
                    continue
                else:
                    # Velocities generation is still in progress. Continue...
                    continue

                last_image = None
                cmd_vel = velocities
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
                    raise RuntimeError(
                        f"Expected value to be a pyarrow Array, got {type(value)}"
                    )
                if value.type != pa.string():
                    raise RuntimeError(
                        f"Expected value to be a pyarrow string Array, got {value.type}"
                    )
                value = value.to_pylist()
                if len(value) > 1:
                    raise RuntimeError(
                        f"Expected value to be a single string, got {value.len()} strings"
                    )
                command = str(value[0])

                print(f"Received command: {command}")
                if command == "":
                    print("Command is empty. Waiting for next input event.")
                    continue


if __name__ == "__main__":
    main()
