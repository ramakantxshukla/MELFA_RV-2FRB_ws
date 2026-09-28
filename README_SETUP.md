# Mitsubishi MELFA RV-2FRB Robot — ROS 2 Setup Guide

**Robot**: Mitsubishi MELFA RV-2FRB (6-axis articulated arm, 2 kg payload)  
**Controller**: Mitsubishi CR800-02VD (CR800 D-series)  
**OS**: Ubuntu 22.04.5 LTS  
**ROS 2**: Humble Hawksbill  
**Date**: September 2026

---

## Table of Contents

1. [What This Setup Does](#what-this-setup-does)
2. [System Specifications](#system-specifications)
3. [Quick Start](#quick-start)
4. [Network Configuration](#network-configuration)
5. [Driver Installation](#driver-installation)
6. [Teach Pendant Setup](#teach-pendant-setup)
7. [Launching the Driver](#launching-the-driver)
8. [Problems Encountered and How They Were Fixed](#problems-encountered-and-how-they-were-fixed)
9. [Port Reference](#port-reference)
10. [Shutdown and Restart Procedures](#shutdown-and-restart-procedures)

---

## What This Setup Does

This workspace lets your Ubuntu PC control the Mitsubishi MELFA RV-2FRB robot arm over Ethernet using ROS 2.

The PC sends joint position commands to the robot controller 286 times per second over a UDP connection. The robot controller sends back the actual joint angles of the arm in real time. This is called **Real Time External Control (MXT)**.

You can then use ROS 2 tools like MoveIt to plan and execute arm movements, read joint states, and build applications.

---

## System Specifications

| Item | Value |
|---|---|
| Operating System | Ubuntu 22.04.5 LTS (x86_64) |
| Kernel | Linux 6.8.0-138-generic |
| CPU | AMD Ryzen 5 3550H (8 threads) |
| RAM | 13 GB |
| ROS 2 Distribution | Humble Hawksbill |
| ROS Domain ID | 42 |
| Workspace Path | `~/melfa_ros2` |
| Driver Repository | https://github.com/Mitsubishi-Electric-Asia/melfa_ros2_driver |
| Driver Version | Tag `v1.1.1`, commit `a2ab6e8` |
| Robot Model | Mitsubishi MELFA RV-2FRB |
| Controller | Mitsubishi CR800-02VD (D-series) |
| Controller IP | `192.168.0.20` |
| PC Ethernet IP | `192.168.0.100` |
| Communication Protocol | UDP, port 10000 |
| Control Frequency | 286 Hz |

---

## Quick Start

Open a terminal and source the environment:

```bash
source ~/melfa_ros2/setup_melfa_env.sh
```

### Run software simulation (no robot needed):
```bash
ros2 launch melfa_bringup rv2fr_control.launch.py \
  controller_type:=D \
  use_fake_hardware:=true \
  start_rviz:=false
```

### Connect to real robot:
1. Plug in the Ethernet cable (PC USB adapter `enx00e04e9114ff` → robot station LAN socket).
2. Run the pendant program (see [Teach Pendant Setup](#teach-pendant-setup)).
3. Then launch the hardware controller:

```bash
ros2 launch melfa_bringup rv2fr_control.launch.py \
  controller_type:=D \
  use_fake_hardware:=false \
  robot_ip:=192.168.0.20 \
  robot_port:=10000 \
  start_rviz:=false
```

### Connect with MoveIt for motion planning:

**IMPORTANT:** You need to run **TWO** launch files simultaneously:

1. **First terminal** - Launch the hardware controller (must be running first):
```bash
source ~/melfa_ros2/setup_melfa_env.sh
ros2 launch melfa_bringup rv2fr_control.launch.py \
  controller_type:=D \
  use_fake_hardware:=false \
  robot_ip:=192.168.0.20 \
  robot_port:=10000 \
  start_rviz:=false
```

2. **Second terminal** - Launch MoveIt with RViz (after controller is connected):
```bash
source ~/melfa_ros2/setup_melfa_env.sh
ros2 launch melfa_rv2fr_moveit_config rv2fr_moveit.launch.py \
  controller_type:=D \
  use_fake_hardware:=false \
  robot_ip:=192.168.0.20
```

The MoveIt launch file does NOT include the hardware interface, so both launches are required for full functionality.

---

## Network Configuration

### Physical Connection

The Ethernet cable connects the PC's **USB Ethernet adapter `enx00e04e9114ff`** to the LAN socket on the robot training station. The station's internal cabling connects to the CR800-02VD controller LAN port.

### IP Addresses

| Device | Interface | IP Address | Subnet |
|---|---|---|---|
| PC | `enx00e04e9114ff` | `192.168.0.100` | `/24` |
| Robot Controller | — | `192.168.0.20` | `/24` |

### NetworkManager Profile

A dedicated profile called **`melfa-robot`** is configured:

```
Connection Name  : melfa-robot
Interface        : enx00e04e9114ff (USB Ethernet adapter)
IPv4 Method      : Manual (Static)
IP Address       : 192.168.0.100/24
Gateway          : None (disabled)
DNS              : None (disabled)
Never Default    : Yes (Wi-Fi keeps internet access)
Auto-connect     : Yes (activates when cable is plugged in)
```

The `Never Default` setting is important — it stops the Ethernet robot connection from overriding the Wi-Fi internet route.

### Connectivity Verification

Ping the controller to check the link is alive:
```bash
ping -c 4 192.168.0.20
```

Expected output:
```
4 packets transmitted, 4 received, 0% packet loss
rtt min/avg/max/mdev = 0.585/1.080/1.799/0.520 ms
```

The MAC address `58:52:8a:84:f1:f7` of the controller was verified against Mitsubishi Electric's OUI prefix (`58:52:8a`), confirming it is genuine Mitsubishi hardware.

---

## Driver Installation

### 1. Clone the Repository

```bash
mkdir -p ~/melfa_ros2/src
git clone https://github.com/Mitsubishi-Electric-Asia/melfa_ros2_driver.git \
  ~/melfa_ros2/src/melfa_ros2_driver
git -C ~/melfa_ros2/src/melfa_ros2_driver checkout v1.1.1
```

### 2. Install Missing Dependencies (requires sudo)

```bash
sudo apt update && sudo apt install -y \
  ros-humble-moveit \
  ros-humble-moveit-servo \
  ros-humble-moveit-chomp-optimizer-adapter \
  ros-humble-moveit-planners-chomp \
  ros-humble-chomp-motion-planner \
  ros-humble-pilz-industrial-motion-planner \
  ros-humble-warehouse-ros-sqlite \
  ros-humble-ign-ros2-control \
  ros-humble-ros-gz-bridge \
  ros-humble-ros-gz-sim \
  ros-humble-joint-state-publisher \
  ros-humble-joint-state-publisher-gui
```

### 3. Build the Workspace

```bash
source /opt/ros/humble/setup.bash
cd ~/melfa_ros2
colcon build --cmake-args -DCMAKE_BUILD_TYPE=Release --parallel-workers 4
```

### 4. Source the Workspace

```bash
source ~/melfa_ros2/setup_melfa_env.sh
```

---

## Teach Pendant Setup

### Program to Enter

On the teach pendant, navigate to your program editor and enter:

```basic
1 Servo On
2 Open "ENET:192.168.0.100" As #1
3 Mxt 1,1,10
4 End
```

**Line explanations:**

| Line | Command | Meaning |
|---|---|---|
| 1 | `Servo On` | Power on the robot servos |
| 2 | `Open "ENET:192.168.0.100" As #1` | Open an Ethernet socket to the PC (192.168.0.100) and assign it as File #1 |
| 3 | `Mxt 1,1,10` | Start real-time external control: use File #1, accept Joint commands, 10 ms filter |
| 4 | `End` | Program ends after Mxt exits |

### How to Run

1. Set the mode switch to **AUTO** (top right of controller).
2. Enable the teach pendant (**ENABLE** switch ON).
3. Press **[SERVO]** while holding the deadman switch (back of pendant).
4. Press **[START]** (or [F1] on the softkey row).
5. The screen will show `STATUS:RUN` and `STEP:00003` — this means `Mxt` is executing and the controller is waiting for the PC.

### What You Should See on the Pendant

| Status | Meaning |
|---|---|
| `STATUS:READY` + `STEP:00001` | Program loaded, not yet started |
| `STATUS:RUN` + `STEP:00003` | **Mxt is active** — controller waiting for PC |
| `STATUS:RUN` → ERROR appears | Network or program error — check error code |

---

## Port Reference

| Port | Protocol | Direction | Purpose |
|---|---|---|---|
| **10000** | **UDP** | PC → Controller | Real-time external control (MXT protocol) |
| 10001 | UDP | PC → Controller | MXT alternative port (unused in this setup) |

### Why UDP and Not TCP?

The MXT protocol uses **UDP** (not TCP) because:
- UDP has lower latency — essential for 286 Hz real-time control.
- TCP's handshake and retry overhead would break the timing.
- The driver handles its own packet loss detection internally.

### Is There a Firewall Issue?

On Ubuntu, the firewall (`ufw`) may block incoming UDP packets from the controller. Since the driver works by sending packets to the controller first and then receiving replies, and the default Ubuntu `ufw` policy allows outbound connections, this typically works without adding any rules.

If communication fails (driver logs `ERROR: Unable to connect`), run:

```bash
sudo ufw allow in on enx00e04e9114ff proto udp to any port 10000
sudo ufw allow in on enx00e04e9114ff proto udp from 192.168.0.20
```

You can check the firewall status with:

```bash
sudo ufw status
```

---

## Problems Encountered and How They Were Fixed

### Problem 1 — USB Ethernet Adapter Was Not Connected

**What happened:**  
The setup plan was to use a dedicated USB Ethernet adapter (`enx00e04e9114ff`) for the robot connection. When we started the setup, this adapter was not seen by the system at all.

**Why it happened:**  
The USB adapter was not physically plugged into the PC.

**How it was fixed:**  
Instead of the USB adapter, the PC's **built-in Ethernet port** (`eno1`) was used. The NetworkManager profile was updated from `enx00e04e9114ff` to `eno1`. When the Ethernet cable was plugged into `eno1` and connected to the robot station LAN socket, the link came up immediately with the static IP `192.168.0.100`.

---

### Problem 2 — `xacro` Was Not Installed

**What happened:**  
When launching the driver with:
```
ros2 launch melfa_bringup rv2fr_control.launch.py ...
```
The launch immediately failed with the error:
```
executable '[TextSubstitution]' not found on the PATH
```

**Why it happened:**  
The `xacro` tool (used to process robot description files) was not installed as an apt package. The `ros-humble-xacro` package was listed as a dependency but was not present on the system.

**How it was fixed:**  
Without sudo access available at the time, we cloned the `xacro` package directly into the workspace and built it as part of the workspace:

```bash
git clone https://github.com/ros/xacro.git -b ros2 ~/melfa_ws/src/xacro
colcon build --packages-select xacro
```

This made `xacro` available through the workspace `install/setup.bash` without needing any system-level installation.

---

### Problem 3 — MoveIt and Gazebo Packages Not Installed

**What happened:**  
`rosdep check` reported 14 missing system packages including MoveIt, CHOMP planner, Pilz planner, `ros_gz_sim`, `ros_gz_bridge`, and `ign-ros2-control`.

**Why it happened:**  
These are large optional packages that are not installed by default with ROS 2 Humble. The driver depends on them for MoveIt motion planning and Gazebo simulation support.

**How it was fixed:**  
The complete list of missing packages was identified and the `sudo apt install` command was prepared. The user was asked to run this manually in a terminal with their password. The Gazebo packages (`ros_gz_sim`, `ros_gz_bridge`) are only used when `use_sim:=true` — for real hardware and fake hardware operation they are loaded conditionally and do not block the launch.

---

### Problem 4 — `rosdep` Was Not Initialised

**What happened:**  
Running `rosdep check` or `rosdep install` immediately failed with:
```
ERROR: your rosdep installation has not been initialized yet.
```

**Why it happened:**  
`rosdep init` writes to `/etc/ros/rosdep/sources.list.d/` which requires root access. This had never been run on this machine.

**How it was fixed:**  
Instead of using `sudo rosdep init`, a user-space rosdep database was set up under `~/.config/ros/rosdep/sources.list.d/` and `rosdep update` was run with the `ROSDEP_SOURCE_PATH` environment variable pointing to it. This allowed rosdep to update its package cache and check dependencies without needing root.

---

### Problem 5 — Launch File Tries to Load Gazebo Even in Non-Sim Mode

**What happened:**  
Looking at the launch file code, `FindPackageShare("ros_gz_sim")` is called at the Python import stage — before the `IfCondition(use_sim)` condition is evaluated. This means if `ros_gz_sim` is not installed, the launch fails even when `use_sim:=false`.

**Why it happened:**  
This is a known limitation in how the launch file was written in `v1.1.1`. The Gazebo node definitions and `get_package_share_directory('melfa_bringup')` call happen at file load time, not at runtime.

**How it was fixed:**  
Installing `ros-humble-ros-gz-sim` and `ros-humble-ros-gz-bridge` resolves this, even if you never use Gazebo. The packages just need to be present on the system. These are in the `sudo apt install` list above.

---

### Problem 6 — Teach Pendant Program Was Incorrect

**What happened:**  
The teach pendant was showing a program with extra lines that would cause problems:
- Line 4: `Mov P1` — this would command the robot to move to a stored position automatically.
- Line 5: `Hlt` — this would halt the program unexpectedly.
- Line 2 appeared to be missing the file number `#1` at the end (`Open ... As #` instead of `As #1`).

**Why it happened:**  
The program was either partially entered or edited at an earlier session and not completed correctly. The `Mov P1` and `Hlt` lines look like leftover lines from a previous test program.

**How it was fixed:**  
The correct program was confirmed as:
```basic
1 Servo On
2 Open "ENET:192.168.0.100" As #1
3 Mxt 1,1,10
4 End
```
The extra lines should be deleted and the `Open` line must end with `#1` (the file number must match the first argument to `Mxt`).

> **Important safety note**: The `Mov P1` line is dangerous when combined with external control. It would move the robot to a stored position before entering MXT mode, with no warning. Always review the full program before pressing START.

---

### Problem 7 — Wrong Ethernet Profile Interface Name

**What happened:**  
The `melfa-robot` NetworkManager profile was originally created for interface `enx00e04e9114ff` (the USB adapter). When the cable was connected to `eno1` (the built-in port), the profile did not activate automatically because it was bound to the wrong interface name.

**Why it happened:**  
The pre-configuration assumed the USB adapter would be used, but the physical connection ended up on the built-in port.

**How it was fixed:**  
The profile was updated with one command:
```bash
nmcli con mod melfa-robot connection.interface-name eno1
nmcli con up melfa-robot
```
This immediately brought up `192.168.0.100/24` on `eno1` and the controller was reachable.

---

### Problem 8 — No `Mxt` START button visible on pendant

**What happened:**  
The pendant screen showed `FWD`, `JUMP`, `BWD` instead of a `START` button, making it unclear how to run the program.

**Why it happened:**  
This is the **Program Step Execution** screen of the R32TB/R33TB teach pendant. On this model, `START`/`RUN` are on a different screen — the **OPERATION** screen accessed via the `[MONITOR]` key.

**How it was fixed:**  
Two options were identified:
1. Use `FWD` (step forward through each line, good for testing).
2. Press `[MONITOR]` to navigate to the OPERATION screen where `START`, `CYCLE`, and `RESET` softkeys are visible — which is exactly what the user found and confirmed worked.

---

## Robot Model Compatibility Note

The driver uses the **`rv2fr`** configuration for the **RV-2FRB**. This is correct because:

- The Mitsubishi MELFA naming convention states: `RV` = vertical articulated, `2` = 2 kg payload, `F` = F-series, `R` = standard reach, **`B` = all-axis brake option**.
- The **`B` suffix does not change the kinematics** — arm lengths, joint limits, DH parameters, and coordinate frames are identical to the standard RV-2FR.
- All 6 joint names (`rv2fr_joint_1` through `rv2fr_joint_6`) and 7 STL mesh files apply correctly.

---

## Shutdown and Restart Procedures

### Shutdown (PC side)
```bash
# In the terminal running the driver, press Ctrl+C
# Then verify no nodes are left over:
pkill -f ros2_control_node
```

### Shutdown (Pendant side)
1. Press **[STOP]** on the pendant — this exits `Mxt` and the program ends.
2. Press **[SERVO]** to turn servos off.

### Restart
1. Start the pendant program first (`STATUS:RUN` on `Mxt` line).
2. Then launch the PC driver:
   ```bash
   source ~/melfa_ws/setup_melfa_env.sh
   ros2 launch melfa_bringup rv2fr_control.launch.py \
     controller_type:=D \
     use_fake_hardware:=false \
     robot_ip:=192.168.0.20 \
     robot_port:=10000 \
     start_rviz:=false
   ```

> **Order matters**: Always start the pendant program before the PC driver. If the driver starts first and the controller is not running `Mxt`, it will time out immediately.
