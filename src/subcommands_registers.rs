use bitfield_introspect::bitfield_introspect;
use bitfield_struct::bitfield;

/// Saved Permanent Failure Status A
#[bitfield_introspect(model = ti_bq_subcommand, subcommand = 0x0098[0])]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFStatusA {
    #[bits(1, access = RO)]
    #[value(false = "Copper Deposition Permanent Fail did not occur")]
    #[value(true = "Copper Deposition Permanent Fail occurred")]
    pub cudep: bool,

    #[bits(1, access = RO)]
    #[value(false = "Safety FET Overtemperature Permanent Fail did not occur")]
    #[value(true = "Safety FET Overtemperature Permanent Fail occurred")]
    pub sotf: bool,

    #[bits(1)]
    _reserved_5: bool,

    #[bits(1, access = RO)]
    #[value(false = "Safety Cell Overtemperature Permanent Fail did not occur")]
    #[value(true = "Safety Cell Overtemperature Permanent Fail occurred")]
    pub sot: bool,

    #[bits(1, access = RO)]
    #[value(false = "Safety Overcurrent in Discharge Permanent Fail did not occur")]
    #[value(true = "Safety Overcurrent in Discharge Permanent Fail occurred")]
    pub socd: bool,

    #[bits(1, access = RO)]
    #[value(false = "Safety Overcurrent in Charge Permanent Fail did not occur")]
    #[value(true = "Safety Overcurrent in Charge Permanent Fail occurred")]
    pub socc: bool,

    #[bits(1, access = RO)]
    #[value(false = "Safety Cell Overvoltage Permanent Fail did not occur")]
    pub sov: bool,

    #[bits(1, access = RO)]
    #[value(false = "Safety Cell Undervoltage Permanent Fail did not occur")]
    #[value(true = "Safety Cell Undervoltage Permanent Fail occurred")]
    pub suv: bool,
}

/// Saved Permanent Failure Status B
#[bitfield_introspect(model = ti_bq_subcommand, subcommand = 0x0098[1])]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFStatusB {
    #[bits(1, access = RO)]
    #[value(false = "Latched Short Circuit in Discharge Permanent Fail did not occur")]
    #[value(true = "Latched Short Circuit in Discharge Permanent Fail occurred")]
    pub scdl: bool,

    #[bits(2)]
    _reserved_6_5: u8,

    #[bits(1, access = RO)]
    #[value(false = "Voltage Imbalance in Active Permanent Fail did not occur")]
    #[value(true = "Voltage Imbalance in Active Permanent Fail occurred")]
    pub vima: bool,

    #[bits(1, access = RO)]
    #[value(false = "Voltage Imbalance in Relax Permanent Fail did not occur")]
    #[value(true = "Voltage Imbalance in Relax Permanent Fail occurred")]
    pub vimr: bool,

    #[bits(1, access = RO)]
    #[value(false = "Secondary Protector Permanent Fail did not occur")]
    #[value(true = "Secondary Protector Permanent Fail occurred")]
    pub second_lvl: bool,

    #[bits(1, access = RO)]
    #[value(false = "DSG FET Fail Permanent Fail did not occur")]
    #[value(true = "DSG FET Fail Permanent Fail occurred")]
    pub dfetf: bool,

    #[bits(1, access = RO)]
    #[value(false = "CHG FET Fail Permanent Fail did not occur")]
    #[value(true = "CHG FET Fail Permanent Fail occurred")]
    pub cfetf: bool,
}

/// Saved Permanent Failure Status C
#[bitfield_introspect(model = ti_bq_subcommand, subcommand = 0x0098[2])]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFStatusC {
    #[bits(1, access = RO)]
    #[value(false = "Commanded Permanent Fail did not occur")]
    #[value(true = "Commanded Permanent Fail occurred")]
    pub cmdf: bool,

    #[bits(1, access = RO)]
    #[value(false = "Protection Comparator MUX Permanent Fail did not occur")]
    #[value(true = "Protection Comparator MUX Permanent Fail occurred")]
    pub hwmx: bool,

    #[bits(1, access = RO)]
    #[value(false = "VSS Permanent Fail did not occur")]
    #[value(true = "VSS Permanent Fail occurred")]
    pub vssf: bool,

    #[bits(1, access = RO)]
    #[value(false = "VREF Permanent Fail did not occur")]
    #[value(true = "VREF Permanent Fail occurred")]
    pub vref: bool,

    #[bits(1, access = RO)]
    #[value(false = "Low Frequency Oscillator Monitor Permanent Fail did not occur")]
    #[value(true = "Low Frequency Oscillator Monitor Permanent Fail occurred")]
    pub lfof: bool,

    #[bits(1, access = RO)]
    #[value(false = "Instruction ROM Permanent Fail did not occur")]
    #[value(true = "Instruction ROM Permanent Fail occurred")]
    pub irmf: bool,

    #[bits(1, access = RO)]
    #[value(false = "Data ROM Permanent Fail did not occur")]
    #[value(true = "Data ROM Permanent Fail occurred")]
    pub drmf: bool,

    #[bits(1, access = RO)]
    #[value(false = "OTP Memory Permanent Fail did not occur")]
    #[value(true = "OTP Memory Permanent Fail occurred")]
    pub otpf: bool,
}

/// Saved Permanent Failure Status D
#[bitfield_introspect(model = ti_bq_subcommand, subcommand = 0x0098[3])]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct PFStatusD {
    #[bits(7)]
    _reserved_7_1: u8,

    #[bits(1, access = RO)]
    #[value(false = "Top-of-Stack Permanent Fail did not occur")]
    #[value(true = "Top-of-Stack Permanent Fail occurred")]
    pub tosf: bool,
}

/// Provides flags for use during manufacturing
#[bitfield_introspect(model = ti_bq_subcommand, subcommand = 0x0057[0])]
#[bitfield(u16, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct ManufacturingStatus {
    #[bits(8)]
    _reserved_15_8: u8,
    
    /// This bit enables or disables writes to OTP during normal operation. The device can programbits in OTP when a PF occurs or when the fuse is blown to retain state after a full reset. Italso can program MANU_DATA upon request (if in FULLACCESS mode). This bit enablesthe device to program this runtime data to OTP. Programming will only occur when the stackvoltage and temperature are within allowed limits. If this bit is not set, programming may stillbe done in CONFIG_UPDATE mode.
    #[bits(1, access = RO)]
    #[value(false = "Device will not program OTP during normal operation")]
    #[value(true = "Device may program OTP during normal operation")]
    pub otpw_en: bool,

    /// This bit enables or disables Permanent Failure checks. Clearing this bit prevents PermanentFailure from triggering which is useful during manufacturing.
    #[bits(1, access = RO)]
    #[value(false = "Permanent Failure checks are disabled")]
    #[value(true = "Permanent Failure checks are enabled")]
    pub pf_en: bool,

    /// This bit indicates whether the PDSG FET is enabled in FET Test Mode. This bit is controlledusing the PDSGTEST() subcommand.
    #[bits(1, access = RO)]
    #[value(false = "PDSG FET is not enabled in FET Test Mode")]
    #[value(true = "PDSG FET is enabled in FET Test Mode")]
    pub pdsg_test: bool,

    /// This bit enables or disables FET Test mode. In FET Test mode, the FET states arecontrolled by the FET Test subcommands. This is typically used during manufacturing to testFET circuitry. Note that safety checks still may force FETs off (or for body diode protection,on) in FET Test mode.unless FET Test subcommands instruct it to do sosubcommands
    #[bits(1, access = RO)]
    #[value(false = "Normal FET control is disabled. FET Test mode is enabled. Device will not turn on FETs")]
    #[value(true = "Normal FET control is enabled. FET Test mode is disabled. Device will ignore FET Test")]
    pub fet_en: bool,

    #[bits(1)]
    _reserved_3: bool,

    /// This bit indicates whether the DSG FET is enabled in FET Test Mode. This bit is controlledusing the DSGTEST() subcommand.
    #[bits(1, access = RO)]
    #[value(false = "DSG FET is not enabled in FET Test Mode")]
    #[value(true = "DSG FET is enabled in FET Test Mode")]
    pub dsg_test: bool,

    /// This bit indicates whether the CHG FET is enabled in FET Test Mode. This bit is controlledusing the CHGTEST() subcommand.
    #[bits(1, access = RO)]
    #[value(false = "CHG FET is not enabled in FET Test Mode")]
    #[value(true = "CHG FET is enabled in FET Test Mode")]
    pub chg_test: bool,

    /// This bit indicates whether the PCHG FET is enabled in FET Test Mode. This bit is controlledusing the PCHGTEST() subcommand.
    #[bits(1, access = RO)]
    #[value(false = "PCHG FET is not enabled in FET Test Mode")]
    #[value(true = "PCHG FET is enabled in FET Test Mode")]
    pub pchg_test: bool,
}

/// Allows host control of individual FET drivers
#[bitfield_introspect(model = ti_bq_subcommand, subcommand = 0x0097[0])]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct FETControl {
    #[bits(4)]
    _reserved_7_4: u8,

    #[bits(1, access = WO)]
    #[value(false = "PCHG FET is allowed to turn on if other conditions are met")]
    #[value(true = "PCHG FET driver is forced off")]
    pub pchg_off: bool,

    #[bits(1, access = WO)]
    #[value(false = "CHG FET is allowed to turn on if other conditions are met")]
    #[value(true = "CHG FET driver is forced off")]
    pub chg_off: bool,

    #[bits(1, access = WO)]
    #[value(false = "PDSG FET is allowed to turn on if other conditions are met")]
    #[value(true = "PDSG FET driver is forced off")]
    pub pdsg_off: bool,

    #[bits(1, access = WO)]
    #[value(false = "DSG FET is allowed to turn on if other conditions are met")]
    #[value(true = "DSG FET driver is forced off")]
    pub dsg_off: bool,
}

/// Bitfield
#[bitfield_introspect(model = ti_bq_subcommand, subcommand = 0x00A0[0])]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct OTPWriteCheckResult {
    /// This bit is set whenever programming conditions are met. None of the other bits will be setwhen this bit is set.
    #[bits(1, access = RO)]
    #[value(false = "OTP programming not allowed")]
    #[value(true = "OTP programming ok")]
    pub ok: bool,

    #[bits(1)]
    _reserved_6: bool,

    /// The device is not in FULLACCESS and CONFIG_UPDATE mode, or the OTP Lock bit hasbeen set to prevent further modification.
    #[bits(1, access = RO)]
    #[value(false = "OTP not locked")]
    #[value(true = "OTP locked")]
    pub lock: bool,

    /// OTP signature cannot be written (indicating that the signature has already been written toomany times).
    #[bits(1, access = RO)]
    #[value(false = "OTP signature can be programmed")]
    #[value(true = "OTP signature could not be programmed")]
    pub nosig: bool,

    /// Could not program data into OTP (indicating data has been programmed too many times, noXOR bits left). When this bit is set, the following bytes will indicate the address of the firstfailing parameter.
    #[bits(1, access = RO)]
    #[value(false = "Data can be programmed into OTP")]
    #[value(true = "Data could not be programmed into OTP")]
    pub nodata: bool,

    /// The measured internal temperature is above the allowed OTP programming temperaturerange.
    #[bits(1, access = RO)]
    #[value(false = "Temperature is within the allowed range")]
    #[value(true = "Temperature is too high to program OTP")]
    pub ht: bool,

    /// The measured stack voltage is below the allowed OTP programming voltage.
    #[bits(1, access = RO)]
    #[value(false = "The measured stack voltage is above the minimum OTP programming voltage")]
    #[value(true = "The measured stack voltage is below the minimum OTP programming voltage")]
    pub lv: bool,

    /// The measured stack voltage is above the allowed OTP programming voltage.
    #[bits(1, access = RO)]
    #[value(false = "The measured stack voltage is below the maximum OTP programming voltage")]
    #[value(true = "The measured stack voltage is above the maximum OTP programming voltage")]
    pub hv: bool,

}

/// Bitfield
#[bitfield_introspect(model = ti_bq_subcommand, subcommand = 0x00A1[0])]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct OTPWriteResult {
    /// This bit is set whenever programming conditions are met. None of the other bits will be setwhen this one is.
    #[bits(1, access = RO)]
    #[value(false = "OTP programming not allowed")]
    #[value(true = "OTP programming ok")]
    pub ok: bool,

    #[bits(1)]
    _reserved_6: bool,

    /// The device is not in FULLACCESS and CONFIG_UPDATE mode, or the OTP Lock bit hasbeen set to prevent further modification.
    #[bits(1, access = RO)]
    #[value(false = "OTP not locked")]
    #[value(true = "OTP locked")]
    pub lock: bool,

    /// Signature cannot be written (indicating that the signature has already been written too manytimes).
    #[bits(1, access = RO)]
    #[value(false = "OTP signature can be programmed")]
    #[value(true = "OTP signature could not be programmed")]
    pub nosig: bool,

    /// Could not program OTP data (indicating data has been programmed too many times, noXOR bits left). When this bit is set, the following bytes will indicate the address of the firstfailing parameter.
    #[bits(1, access = RO)]
    #[value(false = "Data can be programmed")]
    #[value(true = "Data could not be programmed")]
    pub nodata: bool,

    /// The measured internal temperature is above the allowed OTP programming temperaturerange.
    #[bits(1, access = RO)]
    #[value(false = "Temperature is within the allowed range")]
    #[value(true = "Temperature is too high to program OTP")]
    pub ht: bool,

    /// The measured stack voltage is below the allowed OTP programming voltage.
    #[bits(1, access = RO)]
    #[value(false = "The measured stack voltage is above the minimum OTP programming voltage")]
    #[value(true = "The measured stack voltage is below the minimum OTP programming voltage")]
    pub lv: bool,

    /// The measured stack voltage is above the allowed OTP programming voltage.
    #[bits(1, access = RO)]
    #[value(false = "The measured stack voltage is below the maximum OTP programming voltage")]
    #[value(true = "The measured stack voltage is above the maximum OTP programming voltage")]
    pub hv: bool,

}

/// Voltage regulator settings
#[bitfield_introspect(model = ti_bq_subcommand, subcommand = 0x0098[0])]
#[bitfield(u8, order = Msb, default = false, new = false, defmt = cfg(feature = "defmt"))]
pub struct Reg12ControlRegister {
    /// Selects voltage level for REG2
    #[bits(3, access = RO)]
    reg2_voltage: RegVoltageSetting,
    
    /// Enables or disables REG2
    #[bits(1, access = WO)]
    #[value(false = "REG2 Disabled")]
    #[value(true = "REG2 Enabled")]
    reg2_en: bool,
    
    /// Selects voltage level for REG1
    #[bits(3, access = RO)]
    reg1_voltage: RegVoltageSetting,

    /// Enables or disables REG1
    #[bits(1, access = WO)]
    #[value(false = "REG1 Disabled")]
    #[value(true = "REG1 Enabled")]
    reg1_en: bool
}

/// Selects voltage level for LDOs
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum RegVoltageSetting {
    _1V8 = 3,
    _2V5 = 4,
    _3V0 = 5,
    _3V3 = 6,
    _5V0 = 7
}

impl RegVoltageSetting {
    // const fn into_bits(self) -> u8 {
    //     self as u8
    // }
    
    const fn from_bits(value: u8) -> RegVoltageSetting {
        match value {
            0..=3 => RegVoltageSetting::_1V8, 
            4 => RegVoltageSetting::_2V5,
            5 => RegVoltageSetting::_3V0,
            6 => RegVoltageSetting::_3V3,
            7 => RegVoltageSetting::_5V0,
            _ => unreachable!()
        }
    }
}
