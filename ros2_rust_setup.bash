#!/bin/bash

# If not already, move to the directory of this script
cd "$(dirname "${BASH_SOURCE[0]}")"

# Install the Rust toolchain, the ROS2 Rust client library, and import all necessary source code to build ROS2 packages using Rust.
sudo apt update -y && sudo apt install -y curl build-essential gcc-multilib git libclang-dev python3-pip python3-vcstool
sudo curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
source ~/.cargo/env && cargo install --debug cargo-ament-build
echo "source ~/.cargo/env" >> ~/.bashrc
python3 -m pip install git+https://github.com/colcon/colcon-cargo.git git+https://github.com/colcon/colcon-ros-cargo.git
vcs import .. < ros2_rust.repos
