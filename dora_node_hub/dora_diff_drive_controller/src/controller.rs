/// A simple differential drive controller for a mobile robot.
///
/// TODO(francocipollone): Implement odometry and other features.
pub struct DiffDriveController {
    /// The distance between the wheels.
    wheel_separation: f64,
    /// The radius of the wheels.
    wheel_radius: f64,
}

impl DiffDriveController {
    pub fn new(wheel_separation: f64, wheel_radius: f64) -> Self {
        DiffDriveController {
            wheel_separation,
            wheel_radius,
        }
    }

    /// Computes the wheel speeds based on the linear and angular velocities.
    ///
    /// # Arguments
    ///
    /// * `linear_speed`: The linear velocity of the robot.
    /// * `angular_speed`: The angular velocity of the robot.
    ///
    /// # Returns
    ///
    /// * A tuple containing the left and right wheel speeds in rad/s.
    pub fn compute_wheel_speeds(&self, linear_speed: f64, angular_speed: f64) -> (f64, f64) {
        let left_wheel_angular_speed = (linear_speed - self.wheel_separation * angular_speed * 0.5) / self.wheel_radius;
        let right_wheel_angular_speed =
            (linear_speed + self.wheel_separation * angular_speed * 0.5) / self.wheel_radius;
        (left_wheel_angular_speed, right_wheel_angular_speed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_wheel_speeds() {
        let controller = DiffDriveController::new(1., 0.1);
        let (left_speed, right_speed) = controller.compute_wheel_speeds(1.0, 0.0);
        assert_eq!(left_speed, 10.0);
        assert_eq!(right_speed, 10.0);
    }
}
