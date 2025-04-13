#!/bin/bash

# Run the TXT_ML editor with X11 backend to avoid Wayland buffer size issues
export WINIT_UNIX_BACKEND=x11
cargo run 