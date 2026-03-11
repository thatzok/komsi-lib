use crate::komsi::KomsiCommand;

/// Trait for logging state changes.
#[cfg(feature = "std")]
pub trait VehicleLogger {
    /// Logs a message.
    fn log(&self, msg: String);
}

/// Represents the state of a vehicle.
///
/// This struct holds various properties of a vehicle, such as speed, engine status,
/// and light statuses. It can be used to track changes and generate KOMSI commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VehicleState {
    /// Ignition status (0 = Off, 1 = On)
    pub ignition: bool,
    /// Engine status (0 = Off, 1 = On)
    pub engine: bool,
    /// Passenger doors status (0 = Closed, 1 = Open)
    pub doors: bool,
    /// Current speed
    pub speed: u32,
    /// Maximum speed
    pub maxspeed: u32,
    /// Fuel level
    pub fuel: u8,
    /// Indicator status (0 = Off, 1 = Left, 2 = Right, 3 = Both)
    pub indicator: u8,
    /// Fixing brake / Parking brake status (0 = Released, 1 = Applied)
    pub fixing_brake: bool,
    /// Warning lights status (0 = Off, 1 = On)
    pub lights_warning: bool,
    /// Main lights status (0 = Off, 1 = On)
    pub lights_main: bool,
    /// Front door lights status
    pub lights_front_door: bool,
    /// Second door lights status
    pub lights_second_door: bool,
    /// Third door lights status
    pub lights_third_door: bool,
    /// Fourth door lights status
    pub lights_fourth_door: bool,
    /// Stop request lights status
    pub lights_stop_request: bool,
    /// Stop brake lights status
    pub lights_stop_brake: bool,
    /// High beam lights status
    pub lights_high_beam: bool,
    /// Battery charging light status
    pub battery_light: bool,
    /// Gear selector position
    pub gear_selector: u8,
    /// Door clearance status
    pub door_clearance: bool,
    /// Current date and time
    pub datetime: crate::komsi::KomsiDateTime,
    /// Total distance in meters.
    pub total_distance: u64,
    /// Total distance in kilometers.
    pub total_distance_km: u64,
}

impl Default for VehicleState {
    fn default() -> Self {
        Self {
            ignition: false,
            engine: false,
            doors: false,
            speed: 0,
            indicator: 0,
            fixing_brake: false,
            lights_warning: false,
            lights_main: false,
            lights_front_door: false,
            lights_second_door: false,
            lights_third_door: false,
            lights_fourth_door: false,
            lights_stop_request: false,
            maxspeed: 0,
            lights_high_beam: false,
            fuel: 0,
            lights_stop_brake: false,
            battery_light: false,
            door_clearance: false,
            gear_selector: 0,
            datetime: crate::komsi::KomsiDateTime {
                year: 2000,
                month: 1,
                day: 1,
                hour: 0,
                min: 0,
                sec: 0,
            },
            total_distance: 0,
            total_distance_km: 0,
        }
    }
}

impl VehicleState {
    /// Creates a new `VehicleState` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Prints the current state to the console.
    #[cfg(feature = "std")]
    pub fn print(&self) {
        print!("ignition:{} ", self.ignition);
        print!("engine:{} ", self.engine);
        print!("indicator:{} ", self.indicator);
        print!("fuel:{} ", self.fuel);
        print!("warn:{} ", self.lights_warning);
        print!("lights:{} ", self.lights_main);
        print!("high-beam:{} ", self.lights_high_beam);
        print!("stop:{} ", self.lights_stop_request);
        print!("fixing-brake:{} ", self.fixing_brake);
        print!("stop-brake:{} ", self.lights_stop_brake);
        print!("doors:{} ", self.doors);
        print!("door1:{} ", self.lights_front_door);
        print!("door2:{} ", self.lights_second_door);
        print!("door3:{} ", self.lights_third_door);
        print!("door4:{} ", self.lights_fourth_door);
        print!("speed:{} ", self.speed);
        print!("max-speed:{} ", self.maxspeed);
        print!("battery-light:{} ", self.battery_light);
        print!("door-clearance:{} ", self.door_clearance);
        print!("gear-selector:{} ", self.gear_selector);
        print!("datetime:{:?} ", self.datetime);
        println!(" ");
    }

    /// Compares the current state with a new state and returns a buffer of KOMSI commands.
    ///
    /// If `force` is true, all fields will be included in the command buffer regardless of changes.
    /// An optional `logger` can be provided to log each change.
    #[cfg(feature = "std")]
    pub fn compare(
        &self,
        new: &VehicleState,
        force: bool,
        logger: Option<&dyn VehicleLogger>,
    ) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; 0];

        if self.ignition != new.ignition || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "ignition", self.ignition as u8, new.ignition as u8
                ));
            }
            let cmd = KomsiCommand::Ignition(new.ignition);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.engine != new.engine || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "engine", self.engine as u8, new.engine as u8
                ));
            }
            let cmd = KomsiCommand::Engine(new.engine);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.doors != new.doors || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "doors", self.doors as u8, new.doors as u8
                ));
            }
            let cmd = KomsiCommand::PassengerDoorsOpen(new.doors);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.fixing_brake != new.fixing_brake || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "fixing_brake", self.fixing_brake as u8, new.fixing_brake as u8
                ));
            }
            let cmd = KomsiCommand::FixingBrake(new.fixing_brake);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.indicator != new.indicator || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "indicator", self.indicator, new.indicator
                ));
            }
            let cmd = KomsiCommand::Indicator(new.indicator);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.lights_warning != new.lights_warning || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "lights_warning", self.lights_warning as u8, new.lights_warning as u8
                ));
            }
            let cmd = KomsiCommand::WarningLights(new.lights_warning);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.lights_main != new.lights_main || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "lights_main", self.lights_main as u8, new.lights_main as u8
                ));
            }
            let cmd = KomsiCommand::MainLights(new.lights_main);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.lights_stop_request != new.lights_stop_request || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "lights_stop_request",
                    self.lights_stop_request as u8,
                    new.lights_stop_request as u8
                ));
            }
            let cmd = KomsiCommand::StopRequest(new.lights_stop_request);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.lights_stop_brake != new.lights_stop_brake || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "lights_stop_brake", self.lights_stop_brake as u8, new.lights_stop_brake as u8
                ));
            }
            let cmd = KomsiCommand::StopBrake(new.lights_stop_brake);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.lights_front_door != new.lights_front_door || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "lights_front_door", self.lights_front_door as u8, new.lights_front_door as u8
                ));
            }
            let cmd = KomsiCommand::FrontDoor(new.lights_front_door);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.lights_second_door != new.lights_second_door || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "lights_second_door",
                    self.lights_second_door as u8,
                    new.lights_second_door as u8
                ));
            }
            let cmd = KomsiCommand::SecondDoor(new.lights_second_door);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.lights_third_door != new.lights_third_door || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "lights_third_door", self.lights_third_door as u8, new.lights_third_door as u8
                ));
            }
            let cmd = KomsiCommand::ThirdDoor(new.lights_third_door);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.lights_high_beam != new.lights_high_beam || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "lights_high_beam", self.lights_high_beam as u8, new.lights_high_beam as u8
                ));
            }
            let cmd = KomsiCommand::HighBeam(new.lights_high_beam);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.fuel != new.fuel || force {
            if let Some(l) = logger {
                l.log(format!("{}: {} -> {} ", "fuel", self.fuel, new.fuel));
            }
            let cmd = KomsiCommand::Fuel(new.fuel);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.speed != new.speed || force {
            if let Some(l) = logger {
                l.log(format!("{}: {} -> {} ", "speed", self.speed, new.speed));
            }
            let cmd = KomsiCommand::Speed(new.speed);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.maxspeed != new.maxspeed || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "maxspeed", self.maxspeed, new.maxspeed
                ));
            }
            let cmd = KomsiCommand::MaxSpeed(new.maxspeed);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.battery_light != new.battery_light || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "battery_light", self.battery_light as u8, new.battery_light as u8
                ));
            }
            let cmd = KomsiCommand::BatteryLight(new.battery_light);
            buffer.extend_from_slice(&cmd.build());
        }

        if self.door_clearance != new.door_clearance || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "door_enable", self.door_clearance as u8, new.door_clearance as u8
                ));
            }
            let cmd = KomsiCommand::DoorClearance(new.door_clearance);
            buffer.extend_from_slice(&cmd.build());
        }

        // we send only the total_distance if total_distance_km is changing
        // we do not want to send to many messages
        // the meters should be counted and increased in the client
        if self.total_distance_km != new.total_distance_km || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {} -> {} ",
                    "odometer", self.total_distance_km as u8, new.total_distance_km as u8
                ));
            }
            let cmd = KomsiCommand::Odometer(new.total_distance);
            buffer.extend_from_slice(&cmd.build());
        }

        // we send only the datetime if the minute value is changing
        // we do not want to send too many messages 
        // the time should be increased in the client every second
        if self.datetime.min != new.datetime.min || force {
            if let Some(l) = logger {
                l.log(format!(
                    "{}: {}:{}:{} -> {}:{}:{} ",
                    "datetime",
                    self.datetime.hour,
                    self.datetime.min,
                    self.datetime.sec,
                    new.datetime.hour,
                    new.datetime.min,
                    new.datetime.sec,
                ));
            }
            let cmd = KomsiCommand::DateTime(new.datetime);
            buffer.extend_from_slice(&cmd.build());
        }

        // TODO GearSelector, door4 if this will become a KOMSI-protocol entry sometime

        // add end of line if buffer is not empty
        if buffer.len() > 0 {
            let mut b = KomsiCommand::build_eol();
            buffer.append(&mut b);
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct TestLogger {
        logs: Arc<Mutex<Vec<String>>>,
    }

    impl VehicleLogger for TestLogger {
        fn log(&self, msg: String) {
            self.logs.lock().unwrap().push(msg);
        }
    }

    #[test]
    fn test_vehicle_state_new() {
        let state = VehicleState::new();
        assert_eq!(state.ignition, false);
        assert_eq!(state.speed, 0);
    }

    #[test]
    fn test_compare_no_change() {
        let old = VehicleState::new();
        let new = VehicleState::new();
        let buffer = old.compare(&new, false, None);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_compare_with_changes() {
        let old = VehicleState::new();
        let mut new = VehicleState::new();
        new.ignition = true;
        new.speed = 50;

        let buffer = old.compare(&new, false, None);
        let expected = vec![65, 49, 121, 53, 48, 10];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_compare_force() {
        let old = VehicleState::new();
        let new = VehicleState::new();
        let buffer = old.compare(&new, true, None);
        assert!(!buffer.is_empty());
        assert_eq!(buffer.last(), Some(&10)); // Should end with Linefeed
    }

    #[test]
    fn test_multi_value_line_roundtrip() {
        let mut new = VehicleState::new();
        new.ignition = true;
        new.speed = 85;
        new.fuel = 42;

        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&KomsiCommand::Ignition(new.ignition).build());
        buffer.extend_from_slice(&KomsiCommand::Speed(new.speed).build());
        buffer.extend_from_slice(&KomsiCommand::Odometer(123456).build());
        buffer.extend_from_slice(&KomsiCommand::Fuel(new.fuel).build());
        buffer.extend_from_slice(&KomsiCommand::build_eol());

        // check if buffer ends with Linefeed
        assert_eq!(buffer.last(), Some(&10));

        // decode buffer an check values
        let mut i = 0;
        let mut decoded_commands = Vec::new();
        while i < buffer.len() && buffer[i] != 10 {
            let cmd_char = buffer[i] as char;
            i += 1;
            let start = i;
            while i < buffer.len()
                && buffer[i] != 10
                && !(buffer[i] >= b'A' && buffer[i] <= b'Z')
                && !(buffer[i] >= b'a' && buffer[i] <= b'z')
            {
                i += 1;
            }
            let digits = &buffer[start..i];
            let cmd = KomsiCommand::from_parts(cmd_char, digits).unwrap();
            decoded_commands.push(cmd);
        }

        assert_eq!(decoded_commands.len(), 4);
        assert_eq!(decoded_commands[0], KomsiCommand::Ignition(true));
        assert_eq!(decoded_commands[1], KomsiCommand::Speed(85));
        assert_eq!(decoded_commands[2], KomsiCommand::Odometer(123456));
        assert_eq!(decoded_commands[3], KomsiCommand::Fuel(42));
    }

    #[test]
    fn test_compare_logging_multiple_types() {
        let mut old = VehicleState::new();
        let mut new = VehicleState::new();

        // Boolean (ignition)
        old.ignition = false;
        new.ignition = true;

        // u8 (fuel)
        old.fuel = 10;
        new.fuel = 20;

        // u32 (speed)
        old.speed = 0;
        new.speed = 55;

        let logs = Arc::new(Mutex::new(Vec::new()));
        let logger = TestLogger {
            logs: Arc::clone(&logs),
        };

        let _buffer = old.compare(&new, false, Some(&logger));

        let captured_logs = logs.lock().unwrap();

        // Check if all expected changes are logged
        assert!(captured_logs.iter().any(|s| s.contains("ignition: 0 -> 1")));
        assert!(captured_logs.iter().any(|s| s.contains("fuel: 10 -> 20")));
        assert!(captured_logs.iter().any(|s| s.contains("speed: 0 -> 55")));
    }
}
