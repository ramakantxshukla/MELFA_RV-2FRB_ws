#!/usr/bin/env bash
# MELFA ROS 2 Environment Setup Script
# Usage: source ~/melfa_ws/setup_melfa_env.sh

if [ -f "/opt/ros/humble/setup.bash" ]; then
    source /opt/ros/humble/setup.bash
else
    echo "[ERROR] /opt/ros/humble/setup.bash not found!"
    return 1 2>/dev/null || exit 1
fi

if [ -f "$HOME/melfa_ros2/install/setup.bash" ]; then
    source "$HOME/melfa_ros2/install/setup.bash"
    echo "[INFO] MELFA ROS 2 workspace sourced ($HOME/melfa_ros2/install/setup.bash)"
else
    echo "[WARNING] $HOME/melfa_ros2/install/setup.bash not found. Run 'colcon build' first."
fi

export ROS_DOMAIN_ID=42
echo "[INFO] ROS_DOMAIN_ID set to $ROS_DOMAIN_ID"
