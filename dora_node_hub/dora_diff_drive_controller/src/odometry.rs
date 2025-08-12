use crate::pose_2d::Pose2D;

fn normalize_angle(angle: f64) -> f64 {
    let result = (angle + std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
    if result <= 0.0 {
        result + std::f64::consts::PI
    } else {
        result - std::f64::consts::PI
    }
}
/// Odometry for a diff drive mobile robot.
pub struct DiffDriveOdometry {
    /// Current pose:
    pub current_pose: Pose2D,

    /// Current velocity:
    pub linear: f64, // [m/s]
    pub angular: f64, // [rad/s]

    /// The distance between the wheels.
    wheel_separation: f64, // [m]
    /// The radius of the wheels.
    wheel_radius: f64, // [m]

    /// Previous data for odometry calculations.
    previous_time: f64, // [s]
    previous_left_wheel_position: f64,  // [rad]
    previous_right_wheel_position: f64, // [rad]
}

impl DiffDriveOdometry {
    pub fn new(wheel_separation: f64, wheel_radius: f64) -> Self {
        DiffDriveOdometry {
            wheel_separation,
            wheel_radius,
            current_pose: Pose2D {
                x: 0.0,
                y: 0.0,
                heading: 0.0,
            },
            linear: 0.0,
            angular: 0.0,
            previous_time: -1.0,
            previous_left_wheel_position: 0.0,
            previous_right_wheel_position: 0.0,
        }
    }

    pub fn update(&mut self, left_wheel_position: f64, right_wheel_position: f64, timestamp: f64) {
        if self.previous_time == -1.0 {
            // First update, just set the previous data
            self.previous_time = timestamp;
            self.previous_left_wheel_position = left_wheel_position;
            self.previous_right_wheel_position = right_wheel_position;
            return;
        }

        let dt = timestamp - self.previous_time;
        if dt < 0.01 {
            return; // Ignore updates that are too close together
        }

        // Calculate the change in position of each wheel
        let left_wheel_diff = left_wheel_position - self.previous_left_wheel_position;
        let right_wheel_diff = right_wheel_position - self.previous_right_wheel_position;

        // Obtain velocity.
        // Note that there is no division by dt as it would be canceled out by the multiplication by dt when updating the pose.
        self.linear = (left_wheel_diff + right_wheel_diff) * self.wheel_radius / 2.0;
        self.angular = (right_wheel_diff - left_wheel_diff) * self.wheel_radius / self.wheel_separation;

        let direction: f64 = self.current_pose.heading + self.angular;

        self.current_pose.x += self.linear * direction.cos();
        self.current_pose.y += self.linear * direction.sin();
        self.current_pose.heading += self.angular;
        // Normalize heading to the range [-pi, pi]
        self.current_pose.heading = normalize_angle(self.current_pose.heading);

        // Update previous data
        self.previous_time = timestamp;
        self.previous_left_wheel_position = left_wheel_position;
        self.previous_right_wheel_position = right_wheel_position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn assert_f64_eq(a: f64, b: f64, epsilon: f64) {
        assert!(
            (a - b).abs() < epsilon,
            "Values {} and {} are not within epsilon {}",
            a,
            b,
            epsilon
        );
    }

    #[test]
    fn test_update_wheels_position_linear() {
        let wheel_separation = 1.0; // [m]
        let wheel_radius = 0.5; // [m]
        let mut odometry = DiffDriveOdometry::new(wheel_separation, wheel_radius);
        odometry.update(0.0, 0.0, 0.0);
        // Test with both wheels moving forward
        let wheels_new_position = 2.0 * PI; // [rad]
        odometry.update(wheels_new_position, wheels_new_position, 1.0);

        assert_eq!(odometry.current_pose.x, wheel_radius * wheels_new_position);
        assert_eq!(odometry.current_pose.y, 0.0);
        assert_eq!(odometry.current_pose.heading, 0.0);

        assert_eq!(odometry.linear, PI);
        assert_eq!(odometry.angular, 0.0);
    }

    #[test]
    fn test_update_wheels_position_angular() {
        let wheel_separation = 1.0; // [m]
        let wheel_radius = 0.5; // [m]
        let mut odometry = DiffDriveOdometry::new(wheel_separation, wheel_radius);
        odometry.update(0.0, 0.0, 0.0);
        // Test with both wheels moving forward
        let left_wheel_new_position = -PI; // [rad]
        let right_wheel_new_position = PI; // [rad]
        odometry.update(left_wheel_new_position, right_wheel_new_position, 1.0);

        assert_eq!(odometry.current_pose.x, 0.0);
        assert_eq!(odometry.current_pose.y, 0.0);
        assert_f64_eq(odometry.current_pose.heading, PI, 1e-10);

        assert_eq!(odometry.linear, 0.0);
        assert_f64_eq(odometry.angular, PI, 1e-10);
    }
}
