#!/bin/bash

IMAGE_NAME=gtk-markdown:latest
USER_UID=$(id -u)
WAYLAND_DISPLAY=${WAYLAND_DISPLAY:-wayland-0}
XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-/run/user/$USER_UID}
DBUS_SOCKET=/run/user/$USER_UID/bus

# Save original permissions and ownership of the D-Bus socket
orig_perms=$(stat -c "%a" $DBUS_SOCKET)
orig_owner=$(stat -c "%u" $DBUS_SOCKET)
orig_group=$(stat -c "%g" $DBUS_SOCKET)

# Function to restore original permissions and ownership
restore_perms() {
    echo "Restoring original permissions on D-Bus socket..."
    chmod $orig_perms $DBUS_SOCKET
    chown $orig_owner:$orig_group $DBUS_SOCKET
}
# Ensure restoration on script exit or interruption
trap restore_perms EXIT

# Change permissions to allow root access inside container
chmod 666 $DBUS_SOCKET

# Run the Docker container with proper environment and volumes
docker run --rm -it --privileged \
  --cap-add=SYS_ADMIN \
  --device /dev/dri \
  --security-opt seccomp=unconfined \
  --env WAYLAND_DISPLAY=$WAYLAND_DISPLAY \
  --env XDG_RUNTIME_DIR=$XDG_RUNTIME_DIR \
  --env DISPLAY=$DISPLAY \
  --env DBUS_SESSION_BUS_ADDRESS=unix:path=$DBUS_SOCKET \
  --volume "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" \
  --volume $DBUS_SOCKET:$DBUS_SOCKET \
  --volume "$PWD":/app \
  --volume /etc/machine-id:/etc/machine-id:ro \
  --network host \
  $IMAGE_NAME \
  bash

# Permissions will be restored automatically by the trap on exit
