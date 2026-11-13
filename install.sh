#!/usr/bin/env sh
set -eu

RELEASE_BIN="https://github.com/polete-bermejo/floord/releases/latest/download/floord"
BINARY="floord"
BINARY_PATH="/usr/local/bin/$BINARY"
SERVICE_PATH="$HOME/.config/systemd/user"
TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT

# Download binary
echo "Downloading Floord..."
curl -fsSL "$RELEASE_BIN" -o "$TMP"

# Install binary
echo "Installing Floord..."
sudo install -m 755 "$TMP" "$BINARY_PATH"
# Creating floord-daemon.service
echo "
[Unit]
Description=Floor daemon File organizer
[Service]
ExecStart=$BINARY_PATH --daemon --organizer --cleanup --allow-deletion
Restart=on-failure
[Install]
WantedBy=default.target
" > "$SERVICE_PATH/floor-daemon.service"

# Systemd
echo "Initializing systemd service..."
mkdir -p "$SERVICE_PATH"
systemctl --user daemon-reload
systemctl --user enable --now floor-daemon
systemctl --user restart --now floor-daemon

echo "Floord installed and initialized successfully!"
