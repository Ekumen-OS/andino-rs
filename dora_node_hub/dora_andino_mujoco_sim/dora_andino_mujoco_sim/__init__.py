"""Package for Dora Andino Mujoco Simulation."""

import os

# Define the path to the README file relative to the package directory
readme_path = os.path.join(os.path.dirname(os.path.dirname(__file__)), "README.md")

# Read the content of the README file
try:
    with open(readme_path, encoding="utf-8") as f:
        __doc__ = f.read()
except FileNotFoundError:
    raise FileNotFoundError(
        f"README file not found at {readme_path}. "
        "Please ensure the README.md file is present in the package directory."
    )
