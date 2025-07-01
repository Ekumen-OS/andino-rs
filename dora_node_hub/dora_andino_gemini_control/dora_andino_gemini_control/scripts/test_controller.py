import os

from dora_andino_gemini_control.controller import DiffDriveGeminiControl


def get_camera_image_bytes() -> bytes:
    """Retrieve the camera image bytes from a file."""
    IMAGE_PATH = os.path.dirname(os.path.abspath(__file__)) + "/image.png"
    with open(IMAGE_PATH, "rb") as image_file:
        # Read the image file and encode it to base64
        return image_file.read()


if __name__ == "__main__":
    # Create an instance of the DiffDriveGeminiControl class
    controller = DiffDriveGeminiControl(model="gemini-2.0-flash")

    # Example command to generate velocities
    command = "Turn to the right"

    image_bytes = get_camera_image_bytes()

    linear_velocity, angular_velocity = controller.generate_velocities(
        command, image_bytes
    )

    print(f"Generated velocities: Linear={linear_velocity}, Angular={angular_velocity}")
