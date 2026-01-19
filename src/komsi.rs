/// Represents the different types of commands in the KOMSI protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KomsiCommandKind {
    /// End of command line ("\n")
    EOL = 10,
    /// Ignition status
    Ignition = 65,
    /// Engine status
    Engine = 66,
    /// Passenger doors open status
    PassengerDoorsOpen = 67,
    /// Indicator status
    Indicator = 68,
    /// Fixing brake / Parking brake status
    FixingBrake = 69,
    /// Warning lights status
    LightsWarning = 70,
    /// Main lights status
    LightsMain = 71,
    /// Front door lights status
    LightsFrontDoor = 72,
    /// Second door lights status
    LightsSecondDoor = 73,
    /// Third door lights status
    LightsThirdDoor = 74,
    /// Stop request lights status
    LightsStopRequest = 75,
    /// Stop brake lights status
    LightsStopBrake = 76,
    /// High beam lights status
    LightsHighBeam = 77,
    /// Battery charging light status
    BatteryLight = 78,
    /// Type of simulator
    SimulatorType = 79,
    /// Door enable status
    DoorEnable = 80,
    /// Custom / Reserved command A17
    A17 = 81,
    /// Custom / Reserved command A18
    A18 = 82,
    /// Custom / Reserved command A19
    A19 = 83,
    /// Custom / Reserved command A20
    A20 = 84,
    /// Custom / Reserved command A21
    A21 = 85,
    /// Custom / Reserved command A22
    A22 = 86,
    /// Custom / Reserved command A23
    A23 = 87,
    /// Custom / Reserved command A24
    A24 = 88,
    /// Custom / Reserved command A25
    A25 = 89,
    /// Custom / Reserved command A26
    A26 = 90,

    /// Maximum speed value
    MaxSpeed = 115,
    /// Engine RPM value
    RPM = 116,
    /// Air pressure value
    Pressure = 117,
    /// Temperature value
    Temperature = 118,
    /// Oil level/pressure value
    Oil = 119,
    /// Fuel level value
    Fuel = 120,
    /// Current speed value
    Speed = 121,
    /// Water temperature value
    Water = 122,
}

/// Builds a KOMSI command buffer from a command kind and a u32 value.
///
/// The value is converted to its string representation and appended to the command buffer.
pub fn build_komsi_command(cmd: KomsiCommandKind, wert: u32) -> Vec<u8> {
    let cmd_u8 = cmd as u8;
    let mut buffer: Vec<u8> = vec![cmd_u8];
    let mut s: Vec<u8> = wert.to_string().as_bytes().to_vec();

    buffer.append(&mut s);

    buffer
}

/// Builds a KOMSI command buffer from a command kind and a u8 value.
///
/// The value is converted to its string representation and appended to the command buffer.
pub fn build_komsi_command_u8(cmd: KomsiCommandKind, wert: u8) -> Vec<u8> {
    let cmd_u8 = cmd as u8;
    let mut buffer: Vec<u8> = vec![cmd_u8];
    let mut s: Vec<u8> = wert.to_string().as_bytes().to_vec();

    buffer.append(&mut s);

    buffer
}

/// Builds a KOMSI EOL (End Of Line) command buffer.
///
/// This simply contains the EOL byte.
pub fn build_komsi_command_eol() -> Vec<u8> {
    let cmd_u8 = KomsiCommandKind::EOL as u8;
    let buffer: Vec<u8> = vec![cmd_u8];
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_komsi_command() {
        let cmd = KomsiCommandKind::Speed;
        let value = 100;
        let result = build_komsi_command(cmd, value);
        // Speed (121) + "100" (ASCII: 49, 48, 48)
        assert_eq!(result, vec![121, 49, 48, 48]);
    }

    #[test]
    fn test_build_komsi_command_u8() {
        let cmd = KomsiCommandKind::Ignition;
        let value = 1;
        let result = build_komsi_command_u8(cmd, value);
        // Ignition (65) + "1" (ASCII: 49)
        assert_eq!(result, vec![65, 49]);
    }

    #[test]
    fn test_build_komsi_command_eol() {
        let result = build_komsi_command_eol();
        assert_eq!(result, vec![10]);
    }

    #[test]
    fn test_enum_values() {
        assert_eq!(KomsiCommandKind::EOL as u8, 10);
        assert_eq!(KomsiCommandKind::Ignition as u8, 65);
        assert_eq!(KomsiCommandKind::Speed as u8, 121);
    }
}
