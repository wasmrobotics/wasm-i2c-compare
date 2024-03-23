#! /bin/bash/
set -e
# grant permissions to mounted rust volume
chown vscode:vscode /rust-volume

# create /.cargo/config.toml in root folder
mkdir ~vscode/.cargo/
touch ~vscode/.cargo/config.toml
cat << EOF > ~vscode/.cargo/config.toml
[build]
target-dir = "/rust-volume/target"
EOF

chown vscode:vscode ~vscode/.cargo

# this avoids a fatal: detected dubious ownership in repository" error when performing git ops in the workspace
# see https://github.com/microsoft/vscode-remote-release/issues/8656 for some color
# and https://www.kenmuse.com/blog/avoiding-dubious-ownership-in-dev-containers/
git config --global --add safe.directory $1

# /usr/local/cargo/bin/cargo install cargo-component
