#!/bin/bash

# Nome dell'immagine Docker
IMAGE_NAME=gtk-markdown:latest

# UID dell'utente (serve per Wayland e D-Bus)
USER_UID=$(id -u)
WAYLAND_DISPLAY=${WAYLAND_DISPLAY:-wayland-0}
XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-/run/user/$USER_UID}

# Esegui il container
docker run --rm -it \
  --env WAYLAND_DISPLAY=$WAYLAND_DISPLAY \
  --env XDG_RUNTIME_DIR=$XDG_RUNTIME_DIR \
  --volume $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY \
  --volume "$PWD":/app \
  --network host \
  $IMAGE_NAME \
  bash

