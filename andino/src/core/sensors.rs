/// Abstract representation of the wheels.
///
/// It provides a way to calculate the angular velocity(rad/s) and position(rad) of the wheels
/// based on the encoder ticks.
#[derive(Debug)]
pub struct Wheel {
    /// The number of ticks per revolution of the wheel.
    ticks_per_revolution: u64,
    /// The number of encoder ticks per radian.
    ticks_per_rad: u64,
    /// The last ticks count from the encoder.
    last_ticks_count: i64,
    /// The current wheel state.
    state: WheelState,
}

/// The state of the wheel.
#[derive(Clone, Debug)]
pub struct WheelState {
    /// The current speed of the wheel in rads per second.
    pub velocity: f64,
    /// The current position of the wheel in rads.
    pub position: f64,
}

impl Wheel {
    /// Creates a new wheel with the given ticks per revolution.
    pub fn new(ticks_per_revolution: u64) -> Self {
        // Formula: ticks_per_rad = ticks_per_revolution / (2 * PI)
        let ticks_per_rad = (ticks_per_revolution as f64 / (2.0 * std::f64::consts::PI)).round() as u64;
        Self {
            ticks_per_rad,
            ticks_per_revolution,
            last_ticks_count: 0,
            state: WheelState {
                velocity: 0.0,
                position: 0.0,
            },
        }
    }

    /// Gets the current state of the wheel.
    pub fn get_state(&self) -> &WheelState {
        &self.state
    }

    /// Encoder ticks per radian unit.
    pub fn ticks_per_rad(&self) -> u64 {
        self.ticks_per_rad
    }

    /// Updates the wheel state based on the given ticks and delta time.
    ///
    /// This method should be called periodically to update the state of the wheel.
    /// Consider using a timer or a loop to call this method at regular intervals as
    /// there are calculations that depend on the time elapsed since the last update.
    pub fn update(&mut self, ticks: i64, delta_time: f64) -> &WheelState {
        self.update_velocity(ticks, delta_time);
        self.update_position(ticks);
        &self.state
    }

    // Update the position of the wheel based on the ticks count.
    fn update_position(&mut self, ticks: i64) {
        self.state.position = (ticks as f64 / self.ticks_per_revolution as f64) * (2.0 * std::f64::consts::PI);
    }

    // Update the angular velocity of the wheel based on the ticks count and delta time.
    fn update_velocity(&mut self, ticks: i64, delta_time: f64) {
        let delta_ticks = ticks - self.last_ticks_count;
        self.state.velocity =
            (delta_ticks as f64 / self.ticks_per_revolution as f64) * (2.0 * std::f64::consts::PI / delta_time);
        self.last_ticks_count = ticks;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wheel_update() {
        let mut wheel = Wheel::new(1000);
        // Initial ticks count is 0.
        let state = wheel.update(500, 1.0);
        assert_eq!(state.position, std::f64::consts::PI);
        assert_eq!(state.velocity, std::f64::consts::PI);

        let state = wheel.update(1000, 1.0);
        assert_eq!(state.position, 2. * std::f64::consts::PI);
        assert_eq!(state.velocity, std::f64::consts::PI);

        let state = wheel.update(500, 1.0);
        assert_eq!(state.position, std::f64::consts::PI);
        assert_eq!(state.velocity, -std::f64::consts::PI);
    }

    #[test]
    fn test_wheel_ticks_per_rad() {
        let wheel = Wheel::new(1000);
        assert_eq!(wheel.ticks_per_rad(), 159);
    }
}
