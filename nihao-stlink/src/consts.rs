pub const ENDPOINT_IN: u8 = 0x80;
pub const ENDPOINT_OUT: u8 = 0x00;
pub const STLINK_RX_EP: u8 = 1 | ENDPOINT_IN;
pub const STLINK_TX_EP: u8 = 2 | ENDPOINT_OUT;
pub const STLINK_TRACE_EP: u8 = 3 | ENDPOINT_IN;
 
pub const STLINK_GET_VERSION: &'static [u8] = &[0xF1];
pub const STLINK_GET_TARGET_VOLTAGE: &'static [u8] = &[0xF7];

pub const STLINK_VID: u16 = 0x0483;
pub const STLINK_V1_PID: u16 = 0x3744;
pub const STLINK_V2_PID: u16 = 0x3748;
pub const STLINK_V2_1_PID: u16 = 0x374B;
pub const STLINK_V2_1_NO_MSD_PID: u16 = 0x3752;
pub const STLINK_V3_USBLOADER_PID: u16 = 0x374D;
pub const STLINK_V3E_PID: u16 = 0x374E;
pub const STLINK_V3S_PID: u16 = 0x374F;
pub const STLINK_V3_2VCP_PID: u16 = 0x3753;
