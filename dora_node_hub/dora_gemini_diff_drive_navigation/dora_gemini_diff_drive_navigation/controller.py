import json
import os
import time

import numpy as np
from google import genai
from google.genai import types


def create_twist_array(linear_vel: float, angular_vel: float) -> np.ndarray:
    """Create a numpy array representing a Twist message.

    Args:
        linear_vel (float): Linear velocity in m/s.
        angular_vel (float): Angular velocity in rad/s.

    Returns:
        numpy.ndarray: A 6-element array where the first three elements are linear velocities
                       (x, y, z) and the last three elements are angular velocities (roll, pitch, yaw).

    """
    return np.array(
        [
            linear_vel,  # Linear velocity in x (m/s)
            0.0,  # Linear velocity in y (m/s)
            0.0,  # Linear velocity in z (m/s)
            0.0,  # Angular velocity in x (rad/s)
            0.0,  # Angular velocity in y (rad/s)
            angular_vel,  # Angular velocity in z (rad/s)
        ],
        dtype=np.float64,
    )


class DiffDriveGeminiControl:
    """A controller for a differential drive robot using Gemini API."""

    # TODO(francocipollone): Allow to set diff drive robot parameters (wheel radius, wheel separation) via constructor.
    #                        Same thing for the camera parameters. For now, these parameters are hardcoded.
    # TODO(francocipollone): Provide a way to extend the system instruction via constructor.
    SYSTEM_INSTRUCTION = R"""
                You are a differential drive robot controller (WHEEL_RADIUS: 0.0315 [m] and WHEEL_SEPARATION: 0.137 [m])
                with a camera pointed forward.
                You receive a user command and a camera image.
                You have to determine the linear and angular velocities
                to navigate as the user commands.
                The linear velocity is the forward speed and the angular velocity is the turn rate.
                The camera is used to understand the environment, so if you want to point to a target,
                you can use the image to determine the direction and speed.
                It is useful to have the target in the center of the image.
                If you see all black, it means you are crashing into a wall.
                Output your response as a JSON object with 'linear_velocity' (m/s)
                and 'angular_velocity' (rad/s) fields.
                Linear velocity should be between -0.5 and 0.5 m/s.
                Angular velocity should be between -1.0 and 1.0 rad/s.
                For example: {'linear_velocity': 0.5, 'angular_velocity': 0.1}
                Use simple navigation, like moving forward, turning left or right,
                Information about the camera's intrinsics:
                    - Lens: f=3.04 mm, f/2.0
                    - Angle of View: 62.2 x 48.8 degrees
                    - Resolution: 640 x 480 pixels
"""

    def __init__(self, model: str = "gemini-2.5-flash"):
        """Initialize the DiffDriveGeminiControl with the specified model."""
        API_KEY = os.getenv("GEMINI_API_KEY", "")  # Leave empty if Canvas provides it at runtime
        if not API_KEY:
            raise ValueError("GEMINI_API_KEY environment variable is not set.")
        print(f"Using Gemini model: {model}")
        print(f"Using System Instruction: {self.SYSTEM_INSTRUCTION}")
        self.client = genai.Client(api_key=API_KEY)
        self.model = model

    async def generate_velocities(self, command: str, image_bytes: bytes) -> np.ndarray:
        """Generate linear and angular velocities based on the command and image.

        Args:
            command (str): The user command describing the desired action.
            image_bytes (bytes): The camera image in PNG format as bytes.

        Returns:
            numpy.ndarray: A 6-element numpy array representing the Twist message,
                           where the first three elements are linear velocities (x, y, z)
                           and the last three elements are angular velocities (roll, pitch, yaw).
                           In a diff drive robot, typically only the x and z components are used.

        """
        contents = [
            types.Content(
                role="user",
                parts=[
                    types.Part.from_text(text=command),
                    types.Part.from_bytes(data=image_bytes, mime_type="image/png"),
                ],
            )
        ]
        config = types.GenerateContentConfig(
            response_mime_type="application/json",
            response_json_schema={
                "type": "OBJECT",
                "properties": {
                    "linear_velocity": {"type": "NUMBER"},
                    "angular_velocity": {"type": "NUMBER"},
                },
                "required": ["linear_velocity", "angular_velocity"],
            },
            system_instruction=self.SYSTEM_INSTRUCTION,
            temperature=0.2,
        )
        now = time.time()
        response = await self.client.aio.models.generate_content(model=self.model, contents=contents, config=config)
        if response.candidates and response.candidates[0].content and response.candidates[0].content.parts:
            response_text = response.candidates[0].content.parts[0].text
            print(f"Response from Gemini in {time.time() - now}: {response_text}")
            try:
                velocity_data = json.loads(response_text)
                linear_vel = velocity_data.get("linear_velocity", 0.0)
                angular_vel = velocity_data.get("angular_velocity", 0.0)
                return create_twist_array(linear_vel, angular_vel)
            except json.JSONDecodeError as e:
                raise ValueError(f"Invalid JSON response from Gemini: {response_text}") from e
        else:
            raise ValueError("No valid response from Gemini API.") from None
