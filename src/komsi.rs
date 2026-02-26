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
    /// total distance traveled
    Odometer = 113,
    /// actual Date and Time
    DateTime = 114,
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

#[derive(Debug)]
pub enum KomsiError {
    CommandReadError,
    InvalidCommand(char),
    InvalidValue,
    InvalidDateTime,
}

#[derive(Debug, Clone, Copy)]
pub struct KomsiDateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}

#[derive(Debug)]
pub enum KomsiCommand {
    Ignition(bool),           // A
    Engine(bool),             // B
    PassengerDoorsOpen(bool), // C
    Indicator(u8),            // D
    FixingBrake(bool),        // E
    WarningLights(bool),      // F
    MainLights(bool),         // G
    FrontDoor(bool),          // H
    SecondDoor(bool),         // I
    ThirdDoor(bool),          // J
    StopRequest(bool),        // K
    StopBrake(bool),          // L
    HighBeam(bool),           // M
    BatteryLight(bool),       // N
    SimulatorType(u8),        // O
    DoorEnable(bool),         // P
    Odometer(u64),            // o
    DateTime(KomsiDateTime),  // r
    MaxSpeed(u32),            // s
    RPM(u32),                 // t
    Pressure(u32),            // u
    Temperature(u32),         // v
    Oil(u32),                 // w
    Fuel(u8),                 // x
    Speed(u32),               // y
    Water(u32),               // z
}

impl KomsiCommand {
    pub fn from_parts(cmd_char: char, digits: &[u8]) -> Result<Self, KomsiError> {
        // default value 0 if we did not receive a value
        let value_u64 = if digits.is_empty() {
            0
        } else {
            parse_u64(digits)?
        };

        match cmd_char {
            'A' => Ok(KomsiCommand::Ignition(value_u64 != 0)),
            'B' => Ok(KomsiCommand::Engine(value_u64 != 0)),
            'C' => Ok(KomsiCommand::PassengerDoorsOpen(value_u64 != 0)),
            'D' => Ok(KomsiCommand::Indicator(value_u64 as u8)),
            'E' => Ok(KomsiCommand::FixingBrake(value_u64 != 0)),
            'F' => Ok(KomsiCommand::WarningLights(value_u64 != 0)),
            'G' => Ok(KomsiCommand::MainLights(value_u64 != 0)),
            'H' => Ok(KomsiCommand::FrontDoor(value_u64 != 0)),
            'I' => Ok(KomsiCommand::SecondDoor(value_u64 != 0)),
            'J' => Ok(KomsiCommand::ThirdDoor(value_u64 != 0)),
            'K' => Ok(KomsiCommand::StopRequest(value_u64 != 0)),
            'L' => Ok(KomsiCommand::StopBrake(value_u64 != 0)),
            'M' => Ok(KomsiCommand::HighBeam(value_u64 != 0)),
            'N' => Ok(KomsiCommand::BatteryLight(value_u64 != 0)),
            'O' => Ok(KomsiCommand::SimulatorType(value_u64 as u8)),
            'P' => Ok(KomsiCommand::DoorEnable(value_u64 != 0)),
            'o' => Ok(KomsiCommand::Odometer(value_u64)),
            'r' => Ok(KomsiCommand::DateTime(parse_datetime(digits)?)),
            's' => Ok(KomsiCommand::MaxSpeed(value_u64 as u32)),
            't' => Ok(KomsiCommand::RPM(value_u64 as u32)),
            'u' => Ok(KomsiCommand::Pressure(value_u64 as u32)),
            'v' => Ok(KomsiCommand::Temperature(value_u64 as u32)),
            'w' => Ok(KomsiCommand::Oil(value_u64 as u32)),
            'x' => Ok(KomsiCommand::Fuel(value_u64 as u8)),
            'y' => Ok(KomsiCommand::Speed(value_u64 as u32)),
            'z' => Ok(KomsiCommand::Water(value_u64 as u32)),
            _ => Err(KomsiError::InvalidCommand(cmd_char)),
        }
    }
}

// Helperfunctions for Parsing

fn parse_u64(digits: &[u8]) -> Result<u64, KomsiError> {
    let mut res: u64 = 0;
    for &d in digits {
        let digit = d.checked_sub(b'0').ok_or(KomsiError::InvalidValue)? as u64;
        if digit > 9 {
            return Err(KomsiError::InvalidValue);
        }
        res = res.saturating_mul(10).saturating_add(digit);
    }
    Ok(res)
}

fn parse_datetime(digits: &[u8]) -> Result<KomsiDateTime, KomsiError> {
    if digits.len() != 14 {
        return Err(KomsiError::InvalidDateTime);
    }
    Ok(KomsiDateTime {
        year: parse_slice_u16(&digits[0..4])?,
        month: parse_slice_u8(&digits[4..6])?,
        day: parse_slice_u8(&digits[6..8])?,
        hour: parse_slice_u8(&digits[8..10])?,
        min: parse_slice_u8(&digits[10..12])?,
        sec: parse_slice_u8(&digits[12..14])?,
    })
}

fn parse_slice_u8(slice: &[u8]) -> Result<u8, KomsiError> {
    let mut res: u8 = 0;
    for &d in slice {
        let digit = d.checked_sub(b'0').ok_or(KomsiError::InvalidValue)?;
        res = res
            .checked_mul(10)
            .ok_or(KomsiError::InvalidValue)?
            .checked_add(digit)
            .ok_or(KomsiError::InvalidValue)?;
    }
    Ok(res)
}

fn parse_slice_u16(slice: &[u8]) -> Result<u16, KomsiError> {
    let mut res: u16 = 0;
    for &d in slice {
        let digit = d.checked_sub(b'0').ok_or(KomsiError::InvalidValue)? as u16;
        res = res
            .checked_mul(10)
            .ok_or(KomsiError::InvalidValue)?
            .checked_add(digit)
            .ok_or(KomsiError::InvalidValue)?;
    }
    Ok(res)
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
