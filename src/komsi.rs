/// Represents the different errors that can occur in the KOMSI protocol.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KomsiError {
    /// An invalid command character was encountered.
    InvalidCommand(char),
    /// An invalid value was encountered.
    InvalidValue,
    /// An invalid date/time format was encountered.
    InvalidDateTime,
    /// The command is unknown.
    UnknownCommand,
}

/// Represents a date and time in the KOMSI protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KomsiDateTime {
    /// Year (e.g., 2024)
    pub year: u16,
    /// Month (1-12)
    pub month: u8,
    /// Day (1-31)
    pub day: u8,
    /// Hour (0-23)
    pub hour: u8,
    /// Minute (0-59)
    pub min: u8,
    /// Second (0-59)
    pub sec: u8,
}

// --- TRAITS FOR TYPE-SAFE CONVERSION ---

/// Trait for deserializing a type from a raw value and digits.
pub trait FromRaw: Sized {
    /// Deserializes the type from a raw `u64` value and a slice of digits.
    fn from_raw(value: u64, digits: &[u8]) -> Result<Self, KomsiError>;
}

/// Trait for serializing a type to a raw buffer.
pub trait ToRaw {
    /// Serializes the type into the provided buffer and returns the number of bytes written.
    fn to_raw(&self, buf: &mut [u8]) -> usize;
}

// --- IMPLEMENTATIONS OF HELPER TRAITS ---

impl FromRaw for bool {
    fn from_raw(v: u64, _: &[u8]) -> Result<Self, KomsiError> {
        Ok(v != 0)
    }
}
impl ToRaw for bool {
    fn to_raw(&self, buf: &mut [u8]) -> usize {
        if buf.len() >= 1 {
            buf[0] = if *self { b'1' } else { b'0' };
            1
        } else {
            0
        }
    }
}

// Helper macro for integer implementations
macro_rules! impl_raw_for_int {
    ($($t:ty),*) => { $(
        impl FromRaw for $t {
            fn from_raw(v: u64, _: &[u8]) -> Result<Self, KomsiError> { Ok(v as $t) }
        }
        impl ToRaw for $t {
            fn to_raw(&self, buf: &mut [u8]) -> usize { write_u64_to_buf(*self as u64, buf) }
        }
    )* };
}
impl_raw_for_int!(u8, u16, u32, u64);

impl FromRaw for KomsiDateTime {
    fn from_raw(_: u64, digits: &[u8]) -> Result<Self, KomsiError> {
        parse_datetime(digits)
    }
}
impl ToRaw for KomsiDateTime {
    fn to_raw(&self, buf: &mut [u8]) -> usize {
        if buf.len() < 14 {
            return 0;
        }
        write_fixed_u16(self.year, &mut buf[0..4]);
        write_fixed_u8(self.month, &mut buf[4..6]);
        write_fixed_u8(self.day, &mut buf[6..8]);
        write_fixed_u8(self.hour, &mut buf[8..10]);
        write_fixed_u8(self.min, &mut buf[10..12]);
        write_fixed_u8(self.sec, &mut buf[12..14]);
        14
    }
}

// --- THE MAIN MACRO ---
macro_rules! define_komsi_commands {
    ($($name:ident = $char:expr => $type:ty),* $(,)?) => {
        /// Represents a KOMSI command.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum KomsiCommand {
            $($name($type)),*
        }

        impl KomsiCommand {
            /// Deserializes a command from a character and digits.
            pub fn from_parts(cmd_char: char, digits: &[u8]) -> Result<Self, KomsiError> {
                let value_u64 = parse_u64(digits).unwrap_or(0);
                match cmd_char as u8 {
                    $(
                        $char => {
                            let val = <$type as FromRaw>::from_raw(value_u64, digits)?;
                            Ok(Self::$name(val))
                        }
                    ),* // This comma separates the generated arms
                    _ => Err(KomsiError::InvalidCommand(cmd_char)),
                }
            }

            /// Serializes the command into the buffer.
            /// Returns the number of bytes written.
            pub fn to_packet(&self, buf: &mut [u8]) -> usize {
                if buf.is_empty() { return 0; }
                match self {
                    $(
                        Self::$name(val) => {
                            buf[0] = $char; // Set the command character at the beginning
                            1 + val.to_raw(&mut buf[1..])
                        }
                    ),* // Also here the separating comma for the macro
                }
            }
        }
    };
}

// --- DEFINITION OF ALL COMMANDS (Central place, Single Source of Truth) ---

define_komsi_commands! {
    Ignition = b'A' => bool,
    Engine = b'B' => bool,
    PassengerDoorsOpen = b'C' => bool,
    Indicator = b'D' => u8,
    FixingBrake = b'E' => bool,
    WarningLights = b'F' => bool,
    MainLights = b'G' => bool,
    FrontDoor = b'H' => bool,
    SecondDoor = b'I' => bool,
    ThirdDoor = b'J' => bool,
    StopRequest = b'K' => bool,
    StopBrake = b'L' => bool,
    HighBeam = b'M' => bool,
    BatteryLight = b'N' => bool,
    SimulatorType = b'O' => u8,
    DoorEnable = b'P' => bool,
    Odometer = b'o' => u64,
    DateTime = b'r' => KomsiDateTime,
    MaxSpeed = b's' => u32,
    RPM = b't' => u32,
    Pressure = b'u' => u32,
    Temperature = b'v' => u32,
    Oil = b'w' => u32,
    Fuel = b'x' => u8,
    Speed = b'y' => u32,
    Water = b'z' => u32
}

// --- HELPER FUNCTIONS FOR PARSING & FORMATTING ---

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
    let p8 = |s: &[u8]| -> Result<u8, KomsiError> {
        let mut n = 0u8;
        for &d in s {
            n = n
                .checked_mul(10)
                .and_then(|n| n.checked_add(d - b'0'))
                .ok_or(KomsiError::InvalidValue)?;
        }
        Ok(n)
    };
    Ok(KomsiDateTime {
        year: (p8(&digits[0..2])? as u16 * 100) + p8(&digits[2..4])? as u16,
        month: p8(&digits[4..6])?,
        day: p8(&digits[6..8])?,
        hour: p8(&digits[8..10])?,
        min: p8(&digits[10..12])?,
        sec: p8(&digits[12..14])?,
    })
}

fn write_u64_to_buf(mut n: u64, buf: &mut [u8]) -> usize {
    if n == 0 {
        if !buf.is_empty() {
            buf[0] = b'0';
            return 1;
        }
        return 0;
    }
    let mut temp = [0u8; 20];
    let mut i = 0;
    while n > 0 {
        temp[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    let mut written = 0;
    for j in (0..i).rev() {
        if written < buf.len() {
            buf[written] = temp[j];
            written += 1;
        }
    }
    written
}

fn write_fixed_u8(n: u8, buf: &mut [u8]) {
    if buf.len() >= 2 {
        buf[0] = b'0' + (n / 10);
        buf[1] = b'0' + (n % 10);
    }
}

fn write_fixed_u16(n: u16, buf: &mut [u8]) {
    if buf.len() >= 4 {
        write_fixed_u8((n / 100) as u8, &mut buf[0..2]);
        write_fixed_u8((n % 100) as u8, &mut buf[2..4]);
    }
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

/// Creates a KOMSI command buffer from a `KomsiCommand` enum.
///
/// Uses the `to_packet` logic generated by the macro.
pub fn build_komsi_command(cmd: KomsiCommand) -> Vec<u8> {
    // A buffer of 32 bytes is enough for all our types
    // (u64 has max 20 digits, DateTime has 14).
    let mut buffer = [0u8; 32];

    // Call the method generated in the macro
    let len = cmd.to_packet(&mut buffer);

    // Return only the part that was actually written as a Vec
    buffer[..len].to_vec()
}

/// Builds a KOMSI EOL (End Of Line) command buffer.
///
/// This simply contains the EOL byte.
pub fn build_komsi_command_eol() -> Vec<u8> {
    let cmd_u8 = 10u8;
    let buffer: Vec<u8> = vec![cmd_u8];
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_roundtrip() {
        let values = [true, false];
        for &v in &values {
            let mut buf = [0u8; 1];
            let len = v.to_raw(&mut buf);
            assert_eq!(len, 1);
            let raw_val = (buf[0] - b'0') as u64;
            let decoded = bool::from_raw(raw_val, &buf).unwrap();
            assert_eq!(v, decoded);
        }
    }

    #[test]
    fn test_int_roundtrip() {
        let mut buf = [0u8; 32];

        // u8
        let v_u8: u8 = 123;
        let len = v_u8.to_raw(&mut buf);
        let decoded = u8::from_raw(parse_u64(&buf[..len]).unwrap(), &buf[..len]).unwrap();
        assert_eq!(v_u8, decoded);

        // u32
        let v_u32: u32 = 12345678;
        let len = v_u32.to_raw(&mut buf);
        let decoded = u32::from_raw(parse_u64(&buf[..len]).unwrap(), &buf[..len]).unwrap();
        assert_eq!(v_u32, decoded);

        // u64
        let v_u64: u64 = 123456789012345;
        let len = v_u64.to_raw(&mut buf);
        let decoded = u64::from_raw(parse_u64(&buf[..len]).unwrap(), &buf[..len]).unwrap();
        assert_eq!(v_u64, decoded);
    }

    #[test]
    fn test_datetime_roundtrip() {
        let dt = KomsiDateTime {
            year: 2024,
            month: 5,
            day: 12,
            hour: 14,
            min: 30,
            sec: 45,
        };
        let mut buf = [0u8; 14];
        let len = dt.to_raw(&mut buf);
        assert_eq!(len, 14);
        let decoded = KomsiDateTime::from_raw(0, &buf).unwrap();
        assert_eq!(dt, decoded);
    }

    #[test]
    fn test_komsi_command_roundtrip() {
        let commands = vec![
            KomsiCommand::Ignition(true),
            KomsiCommand::Indicator(2),
            KomsiCommand::Speed(100),
            KomsiCommand::Odometer(123456),
            KomsiCommand::DateTime(KomsiDateTime {
                year: 2026,
                month: 2,
                day: 26,
                hour: 16,
                min: 45,
                sec: 0,
            }),
        ];

        for cmd in commands {
            let mut buf = [0u8; 64];
            let len = cmd.to_packet(&mut buf);
            let cmd_char = buf[0] as char;
            let digits = &buf[1..len];
            let decoded = KomsiCommand::from_parts(cmd_char, digits).unwrap();
            assert_eq!(cmd, decoded);
        }
    }

    #[test]
    fn test_build_komsi_command() {
        let cmd = KomsiCommand::Speed(120);
        let buf = build_komsi_command(cmd);
        assert_eq!(buf, b"y120");
    }

    #[test]
    fn test_invalid_parse() {
        assert!(parse_u64(b"12a").is_err());
        assert!(parse_datetime(b"2024010112000").is_err()); // Too short
    }
}
