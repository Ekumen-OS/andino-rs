"""Main entry point for the dora_gemini_diff_drive_navigation package."""

import asyncio

from .main import main

if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
