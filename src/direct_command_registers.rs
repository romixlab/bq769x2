use bitfield_introspect::bitfield_introspect;
use bitfield_struct::bitfield;

/// Device status bits
#[bitfield_introspect(model = ti_bq_command, command = 0x00)]
#[bitfield(u16, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct ControlStatus {
    #[bits(13)]
    _reserved_15_3: u16,

    /// This bit indicates whether the device is in DEEPSLEEP mode.
    #[bits(1, access = RO)]
    #[value(false = "Device is not in DEEPSLEEP mode.")]
    #[value(true = "Device is in DEEPSLEEP mode.")]
    pub deep_sleep: bool,

    /// This bit is set when the Load Detect function has timed out and checking has stopped.
    #[bits(1, access = RO)]
    #[value(false = "Load Detect function has not timed out or is inactive.")]
    #[value(true = "Load Detection function timed out and was deactivated.")]
    pub ld_timeout: bool,

    /// This bit indicates whether the Load Detect pullup was active during the previous LDpin voltage measurement.
    #[bits(1, access = RO)]
    #[value(false = "LD pullup was not active during the previous LD pin measurement.")]
    #[value(true = "LD pullup was active during the previous LD pin measurement.")]
    pub ld_on: bool,
}

/// Provides individual alert signals when enabled safety alerts have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x02)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct SafetyAlertA {
    /// Short Circuit in Discharge Protection
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub scd: bool,

    /// Overcurrent in Discharge 2nd Tier Protection
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub ocd2: bool,

    /// Overcurrent in Discharge 1st Tier Protection
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub ocd1: bool,

    /// Overcurrent in Charge Protection
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub occ: bool,

    /// Cell Overvoltage Protection
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub cov: bool,

    /// Cell Undervoltage Protection
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub cuv: bool,

    #[bits(2)]
    _reserved_1_0: u8,
}

/// Provides individual fault signals when enabled safety faults have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x03)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct SafetyStatusA {
    /// Short Circuit in Discharge Protection
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub scd: bool,

    /// Overcurrent in Discharge 2nd Tier Protection
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub ocd2: bool,

    /// Overcurrent in Discharge 1st Tier Protection
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub ocd1: bool,

    /// Overcurrent in Charge Protection
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub occ: bool,

    /// Cell Overvoltage Protection
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub cov: bool,

    /// Cell Undervoltage Protection
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub cuv: bool,

    #[bits(2)]
    _reserved_1_0: u8,
}

/// Provides individual alert signals when enabled safety alerts have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x04)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct SafetyAlertB {
    /// FET Overtemperature
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub otf: bool,

    /// Internal Overtemperature
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub otint: bool,

    /// Overtemperature in Discharge
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub otd: bool,

    /// Overtemperature in Charge
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub otc: bool,

    #[bits(1)]
    _reserved_3: bool,

    /// Internal Undertemperature
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub utint: bool,

    /// Undertemperature in Discharge
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub utd: bool,

    /// Undertemperature in Charge
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub utc: bool,
}

/// Bitfield
#[bitfield_introspect(model = ti_bq_command, command = 0x05)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct SafetyStatusB {
    /// FET Overtemperature
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub otf: bool,

    /// Internal Overtemperature
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub otint: bool,

    /// Overtemperature in Discharge
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub otd: bool,

    /// Overtemperature in Charge
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub otc: bool,

    #[bits(1)]
    _reserved_3: bool,

    /// Internal Undertemperature
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub utint: bool,

    /// Undertemperature in Discharge
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub utd: bool,

    /// Undertemperature in Charge
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub utc: bool,
}

/// Provides individual alert signals when enabled safety alerts have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x06)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct SafetyAlertC {
    /// Overcurrent in Discharge 3rd Tier Protection
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub ocd3: bool,

    /// Short Circuit in Discharge Latch
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub scdl: bool,

    /// Overcurrent in Discharge Latch
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub ocdl: bool,

    /// Cell Overvoltage Latch
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub covl: bool,

    /// Precharge Timeout Suspend
    #[bits(1, access = RO)]
    #[value(false = "Precharge timeout protection is not suspended.")]
    #[value(true = "Precharge timeout protection is suspended.")]
    pub ptos: bool,

    #[bits(3)]
    _reserved_2_0: u8,
}

/// Bitfield
#[bitfield_introspect(model = ti_bq_command, command = 0x07)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct SafetyStatusC {
    /// Overcurrent in Discharge 3rd Tier Protection
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub ocd3: bool,

    /// Short Circuit in Discharge Latch
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub scdl: bool,

    /// Overcurrent in Discharge Latch
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub ocdl: bool,

    /// Cell Overvoltage Latch
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub covl: bool,

    #[bits(1)]
    _reserved_3: bool,
    /// Precharge Timeout
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub pto: bool,

    /// Host Watchdog Fault
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub hwdf: bool,

    #[bits(1)]
    _reserved_0: bool,
}

/// Provides individual alert signals when enabled safety alerts have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x0A)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFAlertA {
    /// Copper Deposition Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub cudep: bool,

    /// Safety Overtemperature FET Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub sotf: bool,

    #[bits(1)]
    _reserved_5: bool,
    /// Safety Overtemperature Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub sot: bool,

    /// Safety Overcurrent in Discharge Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub socd: bool,

    /// Safety Overcurrent in Charge Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub socc: bool,

    /// Safety Cell Overvoltage Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub sov: bool,

    /// Safety Cell Undervoltage Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub suv: bool,
}

/// Provides individual fault signals when enabled Permanent Fail faults have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x0B)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFStatusA {
    /// Copper Deposition Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub cudep: bool,

    /// Safety Overtemperature FET Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub sotf: bool,

    #[bits(1)]
    _reserved_5: bool,

    /// Safety Overtemperature Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub sot: bool,

    /// Safety Overcurrent in Discharge Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub socd: bool,

    /// Safety Overcurrent in Charge Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub socc: bool,

    /// Safety Cell Overvoltage Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub sov: bool,

    /// Safety Cell Undervoltage Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub suv: bool,
}

/// Provides individual alert signals when enabled safety alerts have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x0C)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFAlertB {
    /// Short Circuit in Discharge Latch Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub scdl: bool,

    #[bits(2)]
    _reserved_6_5: u8,

    /// Voltage Imbalance Active Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub vima: bool,

    /// Voltage Imbalance at Rest Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub vimr: bool,

    /// Second Level Protector Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub second_lvl: bool,

    /// Discharge FET Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub dfetf: bool,

    /// Charge FET Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub cfetf: bool,
}

/// Provides individual fault signals when enabled Permanent Fail faults have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x0D)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFStatusB {
    /// Short Circuit in Discharge Latch Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub scdl: bool,

    #[bits(2)]
    _reserved_6_5: u8,

    /// Voltage Imbalance Active Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub vima: bool,

    /// Voltage Imbalance at Rest Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub vimr: bool,

    /// Second Level Protector Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub second_lvl: bool,

    /// Discharge FET Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub dfetf: bool,

    /// Charge FET Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub cfetf: bool,
}

/// Provides individual alert signals when enabled safety alerts have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x0E)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFAlertC {
    #[bits(1)]
    _reserved_7: bool,

    /// Hardware Mux Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub hwmx: bool,

    /// Internal VSS Measurement Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub vssf: bool,

    /// Internal Voltage Reference Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub vref: bool,

    /// Internal LFO Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub lfof: bool,

    #[bits(3)]
    _reserved_2_0: u8,
}

/// Provides individual fault signals when enabled Permanent Fail faults have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x0F)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFStatusC {
    /// Commanded Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub cmdf: bool,

    /// Hardware Mux Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub hwmx: bool,

    /// Internal VSS Measurement Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub vssf: bool,

    /// Internal Voltage Reference Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub vref: bool,

    /// Internal LFO Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub lfof: bool,

    /// Instruction ROM Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub irmf: bool,

    /// Data ROM Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub drmf: bool,

    /// OTP Memory Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub otpf: bool,
}

/// Provides individual alert signals when enabled Permanent Fail alerts have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x10)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFAlertD {
    #[bits(7)]
    _reserved_7_1: u8,

    /// Top of Stack vs Cell Sum Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Alert is not triggered.")]
    #[value(true = "Alert is triggered.")]
    pub tosf: bool,
}

/// Provides individual fault signals when enabled Permanent Fail faults have triggered.
#[bitfield_introspect(model = ti_bq_command, command = 0x11)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFStatusD {
    #[bits(7)]
    _reserved_7_1: u8,

    /// Top of Stack vs Cell Sum Permanent Fail
    #[bits(1, access = RO)]
    #[value(false = "Fault is not triggered.")]
    #[value(true = "Fault is triggered.")]
    pub tosf: bool,
}

/// Flags related to battery status
#[bitfield_introspect(model = ti_bq_command, command = 0x12)]
#[bitfield(u16, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct BatteryStatus {
    /// This bit indicates whether or not the device is presently in SLEEP mode.
    #[bits(1, access = RO)]
    #[value(false = "Device is not in SLEEP mode.")]
    #[value(true = "Device is in SLEEP mode.")]
    pub sleep: bool,

    #[bits(1)]
    _reserved_14: bool,

    /// This bit is set when shutdown is pending because the command was received or theRST_SHUT pin was asserted for at least one second.
    #[bits(1, access = RO)]
    #[value(false = "Shutdown due to command or pin is not pending.")]
    #[value(true = "Shutdown due to command or pin is pending.")]
    pub sd_cmd: bool,

    /// This bit indicates whether or not an enabled Permanent Fail fault has triggered.
    #[bits(1, access = RO)]
    #[value(false = "No Permanent Fail fault has triggered.")]
    #[value(true = "At least one Permanent Fail fault has triggered.")]
    pub pf: bool,

    /// This bit indicates whether or not an enabled safety fault is triggered.
    #[bits(1, access = RO)]
    #[value(false = "No safety fault is triggered.")]
    #[value(true = "At least one enabled safety fault is triggered.")]
    pub ss: bool,

    /// This bit reports the most recently observed state of the FUSE pin and is updated everysecond.
    #[bits(1, access = RO)]
    #[value(false = "FUSE pin was not asserted by device or secondary protector at last sample.")]
    #[value(true = "FUSE pin was asserted by device or secondary protector at last sample.")]
    pub fuse: bool,

    /// These bits indicate the present security state of the device. 
    #[bits(2, access = RO)]
    sec: SecurityState,

    /// This bit indicates whether voltage and temperature conditions are valid for OTPprogramming. During normal operation, this bit will always be set if Manufacturing Status()[OTPW] is clear. When entering CONFIG_UPDATE mode, conditions will be checked andthis bit will reflect whether or not programming is allowed (Manufacturing Status()[OTPW]does not apply in CONFIG_UPDATE mode). Once in CONFIG_UPDATE mode, this bit willnot change state since no new measurements are being taken.
    #[bits(1, access = RO)]
    #[value(false = "OTP writes are allowed.")]
    #[value(true = "Writes to OTP are blocked.")]
    pub otpb: bool,

    /// This bit indicates whether or not some data is waiting to be written to OTP during normaloperation. This can occur when, for example, configured to Permanent Fail information toOTP. This bit may remain set until conditions for OTP programming are met and all data isprogrammed. This bit is not set during OTP programming from CONFIG_UPDATE mode.
    #[bits(1, access = RO)]
    #[value(false = "No writes to OTP are pending.")]
    #[value(true = "Writes to OTP are pending.")]
    pub otpw: bool,

    /// This bit indicates while cell open-wire checks are occurring. When the feature is disabled,this bit will not set. When the feature is enabled, this bit will set periodically as the checksare performed.
    #[bits(1, access = RO)]
    #[value(false = "Device is not actively performing a cell open-wire check.")]
    #[value(true = "Device is actively performing a cell open-wire check.")]
    pub cow_chk: bool,

    /// This bit indicates whether or not the previous device reset was caused by the internalwatchdog timer. This is not related to the Host Watchdog protection.
    #[bits(1, access = RO)]
    #[value(false = "Previous reset was normal.")]
    #[value(true = "Previous reset was caused by the watchdog timer.")]
    pub wd: bool,

    /// This bit is set when the device fully resets. It is cleared upon exit of CONFIG_UPDATEmode. It can be used by the host to determine if any RAM configuration changes were lostdue to a reset.RAM settings is required.
    #[bits(1, access = RO)]
    #[value(false = "Full reset has not occurred since last exit of CONFIG_UPDATE mode.")]
    #[value(true = "Full reset has occurred since last exit of CONFIG_UPDATE and reconfiguration of any")]
    pub por: bool,

    /// This bit indicates whether or not SLEEP mode is allowed based on configuration andcommands. The Settings:Configuration:Power Config[SLEEP] bit sets the default stateof this bit. The host may send commands to enable or disable SLEEP mode based onsystem requirements. When this bit is set, the device may transition to SLEEP mode whenother SLEEP criteria are met.
    #[bits(1, access = RO)]
    #[value(false = "SLEEP mode is disabled by the host.")]
    #[value(true = "SLEEP mode is allowed when other SLEEP conditions are met.")]
    pub sleep_en: bool,

    /// This bit indicates whether or not the device is in PRECHARGE mode. In PRECHARGEmode, the PCHG FET is turned on instead of the CHG FET.
    #[bits(1, access = RO)]
    #[value(false = "Device is not in PRECHARGE mode.")]
    #[value(true = "Device is in PRECHARGE mode.")]
    pub pchg_mode: bool,

    /// This bit indicates whether or not the device is in CONFIG_UPDATE mode. It will be set afterthe SET_CFGUPDATE command is received and fully processed. Configuration settingsmay be changed only while this bit is set.
    #[bits(1, access = RO)]
    #[value(false = "Device is not in CONFIG_UPDATE mode.")]
    #[value(true = "Device is in CONFIG_UPDATE mode.")]
    pub cfgupdate: bool,
}

/// Security state of the device
#[derive(Debug, PartialEq, Eq)]
//#[bitfield_introspect]
#[repr(u8)]
pub enum SecurityState {
    NotInitialized = 0,
    /// When in FULLACCESS mode, unrestricted read and write access is allowed and all commands are accepted.
    /// Even in FULLACCESS mode, changes to device configuration should only be changed while also in CONFIG_UPDATE mode.
    FullAccess = 1,
    /// When in UNSEALED mode, device configuration may normally be read and may be written while in CONFIG_UPDATE mode.
    Unsealed = 2,
    /// When in SEALED mode, device configuration may not be read or written and some commands are restricted.
    Sealed = 3
}

impl SecurityState {
    // const fn into_bits(self) -> u8 {
    //     self as u8
    // }
    
    const fn from_bits(bits: u8) -> SecurityState {
        match bits {
            0 => SecurityState::NotInitialized,
            1 => SecurityState::FullAccess,
            2 => SecurityState::Sealed,
            3 => SecurityState::Sealed,
            _ => unreachable!()
        }
    } 
}

/// Latched signal used to assert the ALERT pin. Write a bit high to clear the latch.
#[bitfield_introspect(model = ti_bq_command, command = 0x62)]
#[bitfield(u16, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct AlarmStatus {
    /// This bit is set when a bit is set in Safety Status B() or Safety Status C().
    #[bits(1, access = RO)]
    pub ssbc: bool,

    /// This bit is set when a bit is set in Safety Status A().
    #[bits(1, access = RO)]
    pub ssa: bool,

    /// This bit is set when an enabled Permanent Fail fault triggers.
    #[bits(1, access = RO)]
    pub pf: bool,

    /// This bit is set when a safety alert is triggered that is also enabled in thecorresponding Settings:Alarm:SF Alert Mask A, Settings:Alarm:SF Alert Mask B, orSettings:Alarm:SF Alert Mask C register.
    #[bits(1, access = RO)]
    pub msk_sfalert: bool,

    /// This bit is set when a Permanent Fail alert is triggered that is also enabled in thecorresponding Settings:Alarm:PF Alert Mask A, Settings:Alarm:PF Alert Mask B,Settings:Alarm:PF Alert Mask C, or Settings:Alarm:PF Alert Mask D register.
    #[bits(1, access = RO)]
    pub msk_pfalert: bool,

    /// Initialization started (sets quickly after device powers up).
    #[bits(1, access = RO)]
    pub initstart: bool,

    /// Initialization completed (sets after the device has powered and completed one measurementscan).
    #[bits(1, access = RO)]
    pub initcomp: bool,

    #[bits(1)]
    _reserved_8: bool,

    /// Full Voltage Scan Complete. The necessary multiple ADC scans have been completed tocollect the full voltag emeasurement loop data (including cell voltages, pin or thermistorvoltages, etc). This bit sets each time a full scan completes (when enabled).
    #[bits(1, access = RO)]
    pub fullscan: bool,

    /// This bit is set when the CHG FET is off.
    #[bits(1, access = RO)]
    pub xchg: bool,

    /// This bit is set when the DSG FET is off.
    #[bits(1, access = RO)]
    pub xdsg: bool,

    /// Stack voltage is below Power:Shutdown:Shutdown Stack Voltage.
    #[bits(1, access = RO)]
    pub shutv: bool,

    /// FUSE Pin Driven. FUSE pin is being driven by either the device or the secondary protector.
    #[bits(1, access = RO)]
    pub fuse: bool,

    /// This bit is set when cell balancing is active.
    #[bits(1, access = RO)]
    pub cb: bool,

    /// Voltage ADC Scan Complete. A single ADC scan is complete (cell voltages are measuredon each scan). This bit sets each time a scan completes (when enabled).
    #[bits(1, access = RO)]
    pub adscan: bool,

    /// This bit is set when the device is wakened from SLEEP mode.
    #[bits(1, access = RO)]
    pub wake: bool,
}

/// Unlatched value of flags which can be selected to be latched (using Alarm
// Enable()) and used to assert the ALERT pin.
#[bitfield_introspect(model = ti_bq_command, command = 0x64)]
#[bitfield(u16, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct AlarmRawStatus {
    /// This bit is set when a bit is set in Safety Status B() or Safety Status C().
    #[bits(1, access = RO)]
    pub ssbc: bool,

    /// This bit is set when a bit is set in Safety Status A().
    #[bits(1, access = RO)]
    pub ssa: bool,

    /// This bit is set when an enabled Permanent Fail fault triggers.
    #[bits(1, access = RO)]
    pub pf: bool,

    /// This bit is set when a safety alert is triggered that is also enabled in thecorresponding Settings:Alarm:SF Alert Mask A, Settings:Alarm:SF Alert Mask B, orSettings:Alarm:SF Alert Mask C register.
    #[bits(1, access = RO)]
    pub msk_sfalert: bool,

    /// This bit is set when a Permanent Fail alert is triggered that is also enabled in thecorresponding Settings:Alarm:PF Alert Mask A, Settings:Alarm:PF Alert Mask B,Settings:Alarm:PF Alert Mask C, or Settings:Alarm:PF Alert Mask D register.
    #[bits(1, access = RO)]
    pub msk_pfalert: bool,

    /// Initialization started (sets quickly after device powers up).
    #[bits(1, access = RO)]
    pub initstart: bool,

    /// Initialization completed (sets after the device has powered and completed one measurementscan).
    #[bits(1, access = RO)]
    pub initcomp: bool,

    #[bits(1)]
    _reserved_8: bool,

    /// Full Voltage Scan Complete. The necessary multiple ADC scans have been completed tocollect the full voltage measurement loop data (including cell voltages, pin or thermistorvoltages, etc). This bit sets after the first full scan completes, then remains set.
    #[bits(1, access = RO)]
    pub fullscan: bool,

    /// This bit is set when the CHG FET is off.
    #[bits(1, access = RO)]
    pub xchg: bool,

    /// This bit is set when the DSG FET is off.
    #[bits(1, access = RO)]
    pub xdsg: bool,

    /// Stack voltage is below Power:Shutdown:Shutdown Stack Voltage.
    #[bits(1, access = RO)]
    pub shutv: bool,

    /// FUSE Pin Driven. FUSE pin is being driven by either the device or the secondary protector.
    #[bits(1, access = RO)]
    pub fuse: bool,

    /// This bit is set when cell balancing is active.
    #[bits(1, access = RO)]
    pub cb: bool,

    /// Voltage ADC Scan Complete. A single ADC scan is complete (cell voltages are measuredon each scan). This bit sets after the first ADC scan completes, then remains set.
    #[bits(1, access = RO)]
    pub adscan: bool,

    /// This bit is set when the device is wakened from SLEEP mode.
    #[bits(1, access = RO)]
    pub wake: bool,
}

/// Mask for Alarm Status(). Can be written to change during operation to change
// which alarm sources are enabled. The default value of this parameter is set by
// Settings:Alarm:Default Alarm Mask.
#[bitfield_introspect(model = ti_bq_command, command = 0x66)]
#[bitfield(u16, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct AlarmEnable {
    /// Setting this bit causes the Alarm Status()[SSBC] to be set and latched when Alarm RawStatus()[SSBC] is asserted.
    #[bits(1, access = RW)]
    pub ssbc: bool,

    /// Setting this bit causes the Alarm Status()[SSA] to be set and latched when Alarm RawStatus()[SSA] is asserted.
    #[bits(1, access = RW)]
    pub ssa: bool,

    /// Setting this bit causes the Alarm Status()[PF] to be set and latched when Alarm RawStatus()[PF] is asserted.
    #[bits(1, access = RW)]
    pub pf: bool,

    /// Setting this bit causes the Alarm Status()[MSK_SFALERT] to be set and latched when AlarmRaw Status()[MSK_SFALERT] is asserted.
    #[bits(1, access = RW)]
    pub msk_sfalert: bool,

    /// Setting this bit causes the Alarm Status()[MSK_PFALERT] to be set and latched when AlarmRaw Status()[MSK_PFALERT] is asserted.
    #[bits(1, access = RW)]
    pub msk_pfalert: bool,

    /// Setting this bit causes the Alarm Status()[INITSTART] to be set and latched when AlarmRaw Status()[INITSTART] is asserted.
    #[bits(1, access = RW)]
    pub initstart: bool,

    /// Setting this bit causes the Alarm Status()[INITCOMP] to be set and latched when Alarm RawStatus()[INITCOMP] is asserted.
    #[bits(1, access = RW)]
    pub initcomp: bool,

    #[bits(1)]
    _reserved_8: bool,

    /// Setting this bit causes the Alarm Status()[FULLSCAN] to be set and latched when AlarmRaw Status()[FULLSCAN] is asserted.
    #[bits(1, access = RW)]
    pub fullscan: bool,

    /// Setting this bit causes the Alarm Status()[XCHG] to be set and latched when Alarm RawStatus()[XCHG] is asserted.
    #[bits(1, access = RW)]
    pub xchg: bool,

    /// Setting this bit causes the Alarm Status()[XDSG] to be set and latched when Alarm RawStatus()[XDSG] is asserted.
    #[bits(1, access = RW)]
    pub xdsg: bool,

    /// Setting this bit causes the Alarm Status()[SHUTV] to be set and latched when Alarm RawStatus()[SHUTV] is asserted.
    #[bits(1, access = RW)]
    pub shutv: bool,

    /// Setting this bit causes the Alarm Status()[FUSE] to be set and latched when Alarm RawStatus()[FUSE] is asserted.
    #[bits(1, access = RW)]
    pub fuse: bool,

    /// Setting this bit causes the Alarm Status()[CB] to be set and latched when Alarm RawStatus()[CB] is asserted.
    #[bits(1, access = RW)]
    pub cb: bool,

    /// Setting this bit causes the Alarm Status()[ADSCAN] to be set and latched when Alarm RawStatus()[ADSCAN] is asserted.
    #[bits(1, access = RW)]
    pub adscan: bool,

    /// Setting this bit causes the Alarm Status()[WAKE] to be set and latched when Alarm RawStatus()[WAKE] is asserted.
    #[bits(1, access = RW)]
    pub wake: bool,
}

/// Provides flags showing status of FETs and ALERT pin.
#[bitfield_introspect(model = ti_bq_command, command = 0x7F)]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct FETStatus {
    #[bits(1)]
    _reserved_7: bool,

    /// Indicates the status of the ALERT pin.
    #[bits(1, access = RO)]
    #[value(false = "The ALERT pin is not asserted.")]
    #[value(true = "The ALERT pin is asserted.")]
    pub alrt_pin: bool,

    /// Indicates the status of the DDSG pin.
    #[bits(1, access = RO)]
    #[value(false = "The DDSG pin is not asserted.")]
    #[value(true = "The DDSG pin is asserted.")]
    pub ddsg_pin: bool,

    /// Indicates the status of the DCHG pin.
    #[bits(1, access = RO)]
    #[value(false = "The DCHG pin is not asserted.")]
    #[value(true = "The DCHG pin is asserted.")]
    pub dchg_pin: bool,

    /// Indicates the status of the PDSG FET.
    #[bits(1, access = RO)]
    #[value(false = "The PDSG FET is off.")]
    #[value(true = "The PDSG FET is on.")]
    pub pdsg_fet: bool,

    /// Indicates the status of the DSG FET.
    #[bits(1, access = RO)]
    #[value(false = "The DSG FET is off.")]
    #[value(true = "The DSG FET is on.")]
    pub dsg_fet: bool,

    /// Indicates the status of the PCHG FET.
    #[bits(1, access = RO)]
    #[value(false = "The PCHG FET is off.")]
    #[value(true = "The PCHG FET is on.")]
    pub pchg_fet: bool,

    /// Indicates the status of the CHG FET.
    #[bits(1, access = RO)]
    #[value(false = "The CHG FET is off.")]
    #[value(true = "The CHG FET is on.")]
    pub chg_fet: bool,
}
