#!/usr/bin/env sh
set -eu

REPO="https://github.com/Kure2323/floord.git"
SERVICE_PATH="$HOME/.config/systemd/user"

# Check and install Rust/Cargo
if ! command -v cargo >/dev/null 2>&1; then
    echo "Cargo not found. Installing Rust..."

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

echo "Using Cargo: $(cargo --version)"

# Download and compile binary
echo "Downloading and compiling floord..."
cargo install --git "$REPO" --locked --force floord

# Creating floord-daemon.service
echo "
[Unit]
Description=Floor daemon File organizer
[Service]
ExecStart=$HOME/.cargo/bin/floord --delay 300 --daemon
Restart=on-failure
[Install]
WantedBy=default.target
" > "$SERVICE_PATH/floord-daemon.service"

# Systemd
echo "Initializing systemd service..."
systemctl --user daemon-reload
systemctl --user enable --now floor-daemon

echo "Floord installed and initialized successfully!"
