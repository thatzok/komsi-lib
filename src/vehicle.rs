use crate::komsi::build_komsi_command;
use crate::komsi::build_komsi_command_eol;
use crate::komsi::build_komsi_command_u8;
use crate::komsi::KomsiCommandKind;

/// Trait for logging state changes.
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
    pub ignition: u8,
    /// Engine status (0 = Off, 1 = On)
    pub engine: u8,
    /// Passenger doors status (0 = Closed, 1 = Open)
    pub doors: u8,
    /// Current speed
    pub speed: u32,
    /// Maximum speed
    pub maxspeed: u32,
    /// Fuel level
    pub fuel: u32,
    /// Indicator status (0 = Off, 1 = Left, 2 = Right, 3 = Both)
    pub indicator: u8,
    /// Fixing brake / Parking brake status (0 = Released, 1 = Applied)
    pub fixing_brake: u8,
    /// Warning lights status (0 = Off, 1 = On)
    pub lights_warning: u8,
    /// Main lights status (0 = Off, 1 = On)
    pub lights_main: u8,
    /// Front door lights status
    pub lights_front_door: u8,
    /// Second door lights status
    pub lights_second_door: u8,
    /// Third door lights status
    pub lights_third_door: u8,
    /// Fourth door lights status
    pub lights_fourth_door: u8,
    /// Stop request lights status
    pub lights_stop_request: u8,
    /// Stop brake lights status
    pub lights_stop_brake: u8,
    /// High beam lights status
    pub lights_high_beam: u8,
    /// Battery charging light status
    pub battery_light: u8,
    /// Gear selector position
    pub gear_selector: u8,
    /// Door enable status
    pub door_enable: u8,
}

impl Default for VehicleState {
    fn default() -> Self {
        Self {
            ignition: 0,
            engine: 0,
            doors: 0,
            speed: 0,
            indicator: 0,
            fixing_brake: 0,
            lights_warning: 0,
            lights_main: 0,
            lights_front_door: 0,
            lights_second_door: 0,
            lights_third_door: 0,
            lights_fourth_door: 0,
            lights_stop_request: 0,
            maxspeed: 0,
            lights_high_beam: 0,
            fuel: 0,
            lights_stop_brake: 0,
            battery_light: 0,
            door_enable: 0,
            gear_selector:0,
        }
    }
}

impl VehicleState {
    /// Creates a new `VehicleState` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Prints the current state to the console.
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
        print!("door-enable:{} ", self.door_enable);
        print!("gear-selector:{} ", self.gear_selector);
        println!(" ");
    }

    /// Compares the current state with a new state and returns a buffer of KOMSI commands.
    ///
    /// If `force` is true, all fields will be included in the command buffer regardless of changes.
    /// An optional `logger` can be provided to log each change.
    pub fn compare(
        &self,
        new: &VehicleState,
        force: bool,
        logger: Option<&dyn VehicleLogger>,
    ) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; 0];

        self.handle_u8_field_change(
            self.ignition,
            new.ignition,
            "ignition",
            KomsiCommandKind::Ignition,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.engine,
            new.engine,
            "engine",
            KomsiCommandKind::Engine,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.doors,
            new.doors,
            "doors",
            KomsiCommandKind::PassengerDoorsOpen,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.fixing_brake,
            new.fixing_brake,
            "fixing_brake",
            KomsiCommandKind::FixingBrake,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.indicator,
            new.indicator,
            "indicator",
            KomsiCommandKind::Indicator,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.lights_warning,
            new.lights_warning,
            "lights_warning",
            KomsiCommandKind::LightsWarning,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.lights_main,
            new.lights_main,
            "lights_main",
            KomsiCommandKind::LightsMain,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.lights_stop_request,
            new.lights_stop_request,
            "lights_stop_request",
            KomsiCommandKind::LightsStopRequest,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.lights_stop_brake,
            new.lights_stop_brake,
            "lights_stop_brake",
            KomsiCommandKind::LightsStopBrake,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.lights_front_door,
            new.lights_front_door,
            "lights_front_door",
            KomsiCommandKind::LightsFrontDoor,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.lights_second_door,
            new.lights_second_door,
            "lights_second_door",
            KomsiCommandKind::LightsSecondDoor,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.lights_third_door,
            new.lights_third_door,
            "lights_third_door",
            KomsiCommandKind::LightsThirdDoor,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.lights_high_beam,
            new.lights_high_beam,
            "lights_high_beam",
            KomsiCommandKind::LightsHighBeam,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u32_field_change(
            self.fuel,
            new.fuel,
            "fuel",
            KomsiCommandKind::Fuel,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u32_field_change(
            self.speed,
            new.speed,
            "speed",
            KomsiCommandKind::Speed,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u32_field_change(
            self.maxspeed,
            new.maxspeed,
            "maxspeed",
            KomsiCommandKind::MaxSpeed,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.battery_light,
            new.battery_light,
            "battery_light",
            KomsiCommandKind::BatteryLight,
            logger,
            force,
            &mut buffer,
        );

        self.handle_u8_field_change(
            self.door_enable,
            new.door_enable,
            "door_enable",
            KomsiCommandKind::DoorEnable,
            logger,
            force,
            &mut buffer,
        );

        // TODO GearSelector, door4 if this will become a KOMSI-protocol entry sometime

        // zeilenende hinzu, wenn buffer nicht leer
        if buffer.len() > 0 {
            let mut b = build_komsi_command_eol();
            buffer.append(&mut b);
        }

        buffer
    }

    /// Helper function for handling u8 field changes.
    fn handle_u8_field_change(
        &self,
        old_value: u8,
        new_value: u8,
        field_name: &str,
        command_kind: KomsiCommandKind,
        logger: Option<&dyn VehicleLogger>,
        force: bool,
        buffer: &mut Vec<u8>,
    ) {
        if (old_value != new_value) || force {
            if let Some(l) = logger {
                l.log(format!("{}: {} -> {} ", field_name, old_value, new_value));
            }
            let mut b = build_komsi_command_u8(command_kind, new_value);
            buffer.append(&mut b);
        }
    }

    /// Helper function for handling u32 field changes.
    fn handle_u32_field_change(
        &self,
        old_value: u32,
        new_value: u32,
        field_name: &str,
        command_kind: KomsiCommandKind,
        logger: Option<&dyn VehicleLogger>,
        force: bool,
        buffer: &mut Vec<u8>,
    ) {
        if (old_value != new_value) || force {
            if let Some(l) = logger {
                l.log(format!("{}:  {} -> {} ", field_name, old_value, new_value));
            }
            let mut b = build_komsi_command(command_kind, new_value);
            buffer.append(&mut b);
        }
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
        assert_eq!(state.ignition, 0);
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
        new.ignition = 1;
        new.speed = 50;
        
        let buffer = old.compare(&new, false, None);
        // Ignition(65) + '1' (49) + Speed(121) + '50' (53, 48) + EOL(10)
        let expected = vec![65, 49, 121, 53, 48, 10];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_compare_force() {
        let old = VehicleState::new();
        let new = VehicleState::new();
        let buffer = old.compare(&new, true, None);
        // When force is true, all fields (except maybe door4 which is TODO) are added.
        // It should definitely not be empty.
        assert!(!buffer.is_empty());
        assert_eq!(buffer.last(), Some(&10)); // Should end with EOL
    }

    #[test]
    fn test_compare_with_logger() {
        let old = VehicleState::new();
        let mut new = VehicleState::new();
        new.ignition = 1;
        
        let logs = Arc::new(Mutex::new(Vec::new()));
        let logger = TestLogger { logs: Arc::clone(&logs) };
        
        let _ = old.compare(&new, false, Some(&logger));
        
        let logs_locked = logs.lock().unwrap();
        assert_eq!(logs_locked.len(), 1);
        assert!(logs_locked[0].contains("ignition: 0 -> 1"));
    }
}

