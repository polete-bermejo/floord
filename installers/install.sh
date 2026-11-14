
#!/usr/bin/env sh
set -eu

RELEASE_BIN="https://github.com/polete-bermejo/floord/releases/latest/download/floord-linux"
BINARY="floord"
BINARY_PATH="/usr/local/bin/$BINARY"
SERVICE_PATH="$HOME/.config/systemd/user"
SERVICE_FILE="$SERVICE_PATH/floor-daemon.service"

TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT

# DOWNLOAD BINARY
echo "Downloading Floord..."
curl -fsSL "$RELEASE_BIN" -o "$TMP"

# STOP PREVIOUS SERVICE
if systemctl --user is-active --quiet floor-daemon.service; then
    echo "Stopping existing Floord service..."
    systemctl --user stop floor-daemon.service
fi

# INSTALL BINARY
echo "Installing Floord..."
sudo install -m 755 "$TMP" "$BINARY_PATH"

# CREATE SYSTEMD DIRECTORY
mkdir -p "$SERVICE_PATH"

# CREATE SYSTEMD SERVICE
echo "Creating Floord systemd service..."

cat > "$SERVICE_FILE" <<EOF
[Unit]
Description=Floord file organizer
After=default.target

[Service]
ExecStart=$BINARY_PATH --daemon --organizer --cleanup --allow-deletion
Restart=on-failure
RestartSec=10

[Install]
WantedBy=default.target
EOF

# INITIALIZE SYSTEMD
echo "Initializing systemd service..."

systemctl --user daemon-reload
systemctl --user enable --now floor-daemon.service

echo ""
echo "Floord installed and initialized successfully!"
echo "Binary: $BINARY_PATH"
echo "Service: floor-daemon.service"
