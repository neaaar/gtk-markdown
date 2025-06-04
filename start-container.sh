#!/bin/bash

# Name of the Docker image
IMAGE_NAME=gtk-markdown:latest

# User UID (needed for Wayland and D-Bus)
USER_UID=$(id -u)
WAYLAND_DISPLAY=${WAYLAND_DISPLAY:-wayland-0}
XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-/run/user/$USER_UID}

# Run the container
docker run --rm -it \
  --env WAYLAND_DISPLAY=$WAYLAND_DISPLAY \
  --env XDG_RUNTIME_DIR=$XDG_RUNTIME_DIR \
  --volume $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY \
  --volume "$PWD":/app \
  --network host \
  $IMAGE_NAME \
  bash
