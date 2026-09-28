#!/usr/bin/env python3

import math

import rclpy
from rclpy.node import Node

from sensor_msgs.msg import JointState
from trajectory_msgs.msg import JointTrajectory, JointTrajectoryPoint


class JointPController(Node):

    def __init__(self):
        super().__init__('rv2fr_joint_p_controller')

        # ------------------------------------------------------
        # Joint order expected by rv2fr_controller
        # ------------------------------------------------------
        self.joint_names = [
            'rv2fr_joint_1',
            'rv2fr_joint_2',
            'rv2fr_joint_3',
            'rv2fr_joint_4',
            'rv2fr_joint_5',
            'rv2fr_joint_6'
        ]

        # ------------------------------------------------------
        # Target joint positions [rad]
        # Change these values to whatever target you want.
        # ------------------------------------------------------
        self.target_positions = [
            0.0,    # J1
            0.0,    # J2
            0.0,    # J3
            0.0,    # J4
            0.0,    # J5
            0.0     # J6
        ]

        # ------------------------------------------------------
        # P-controller gains
        #
        # Start low. Increase slowly.
        # ------------------------------------------------------
        self.kp = [
            0.1,
            0.1,
            0.1,
            0.1,
            0.1,
            0.1
        ]

        # Maximum position change per control cycle [rad]
        # Prevents large jumps.
        self.max_step = 0.05

        # Joint error tolerance [rad]
        self.tolerance = 0.01

        # Latest joint-state dictionary
        self.current_positions = {}

        # Used to avoid printing target reached continuously
        self.target_reached_reported = False

        # ------------------------------------------------------
        # Subscriber
        # ------------------------------------------------------
        self.joint_state_sub = self.create_subscription(
            JointState,
            '/joint_states',
            self.joint_state_callback,
            10
        )

        # ------------------------------------------------------
        # Publisher
        # ------------------------------------------------------
        self.trajectory_pub = self.create_publisher(
            JointTrajectory,
            '/rv2fr_controller/joint_trajectory',
            10
        )

        # ------------------------------------------------------
        # Control loop
        #
        # 20 Hz
        # ------------------------------------------------------
        control_frequency = 20.0
        self.dt = 1.0 / control_frequency

        self.timer = self.create_timer(
            self.dt,
            self.control_loop
        )

        self.get_logger().info(
            'RV-2FR P joint controller started.'
        )

        self.get_logger().info(
            f'Target = {self.target_positions}'
        )

    # ==========================================================
    # Joint-state callback
    # ==========================================================
    def joint_state_callback(self, msg: JointState):

        for name, position in zip(msg.name, msg.position):

            if name not in self.joint_names:
                continue

            if not math.isfinite(position):
                continue

            self.current_positions[name] = position

    # ==========================================================
    # Main P controller
    # ==========================================================
    def control_loop(self):

        # Wait until we have all six joints
        if not all(
            joint in self.current_positions
            for joint in self.joint_names
        ):
            return

        current = [
            self.current_positions[name]
            for name in self.joint_names
        ]

        command = []
        errors = []

        # ------------------------------------------------------
        # P controller
        # ------------------------------------------------------
        for i in range(6):

            error = (
                self.target_positions[i]
                - current[i]
            )

            errors.append(error)

            # P controller output
            delta_q = self.kp[i] * error

            # Limit position step
            delta_q = max(
                -self.max_step,
                min(self.max_step, delta_q)
            )

            q_command = current[i] + delta_q

            command.append(q_command)

        # ------------------------------------------------------
        # Check whether target has been reached
        # ------------------------------------------------------
        max_error = max(abs(e) for e in errors)

        if max_error < self.tolerance:

            if not self.target_reached_reported:

                self.get_logger().info(
                    'Target reached.'
                )

                self.get_logger().info(
                    f'Current positions: '
                    f'{[round(x, 4) for x in current]}'
                )

                self.target_reached_reported = True

            # Hold exact target
            command = self.target_positions.copy()

        else:

            self.target_reached_reported = False

        # ------------------------------------------------------
        # Create trajectory message
        # ------------------------------------------------------
        trajectory_msg = JointTrajectory()

        trajectory_msg.joint_names = self.joint_names

        point = JointTrajectoryPoint()

        point.positions = command

        # Controller should reach the newly calculated point
        # before approximately the next few control periods.
        point.time_from_start.sec = 0
        point.time_from_start.nanosec = 100_000_000  # 0.1 s

        trajectory_msg.points = [point]

        self.trajectory_pub.publish(trajectory_msg)

        # Optional diagnostic output
        self.get_logger().debug(
            f'Current: {[round(x, 3) for x in current]} | '
            f'Error: {[round(x, 3) for x in errors]} | '
            f'Command: {[round(x, 3) for x in command]}'
        )


def main(args=None):

    rclpy.init(args=args)

    node = JointPController()

    try:
        rclpy.spin(node)

    except KeyboardInterrupt:
        pass

    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
