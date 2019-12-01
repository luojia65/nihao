use std::io;
use crate::consts::*;
use core::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Version {
    pub stlink_version: u8,
    pub jtag_api: JtagApi,
    pub jtag: u8,
    pub swim: u8,
    pub msd: u8,
    pub bridge: u8,
    pub vid: u16,
    pub pid: u16,
    pub has_trace: bool,
    pub has_get_last_rwstatus2: bool,
    pub has_swd_set_freq: bool,
    pub has_jtag_set_freq: bool,
    pub has_mem_16bit: bool,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("V{}", self.stlink_version);
        if self.jtag > 0 || self.msd == 0 {
            s += &format!("J{}", self.jtag)
        }
        if self.msd > 0 {
            s += &format!("M{}", self.msd)
        }
        if self.bridge > 0 {
            s += &format!("B{}", self.bridge)
        }
        if self.swim > 0 || self.msd == 0 {
            s += &format!("S{}", self.swim)
        }
        f.write_fmt(format_args!(
            "ST-Link {} (API {}) VID:PID {:04X}:{:04X}", 
            s,
            self.jtag_api,
            self.vid,
            self.pid
        ))
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum JtagApi {
    V1,
    V2,
    V3
}

impl fmt::Display for JtagApi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            JtagApi::V1 => "v1",
            JtagApi::V2 => "v2",
            JtagApi::V3 => "v3",
        })
    }
}

pub(crate) fn read_handle(handle: &nihao_usb::Handle<'_>) -> io::Result<Version> {
    let mut buf_recv = vec![0u8; 6];
    handle.write_pipe(STLINK_TX_EP, STLINK_GET_VERSION)?;
    let _len = handle.read_pipe(STLINK_RX_EP, &mut buf_recv)?;
    let version = u16::from_be_bytes([buf_recv[0], buf_recv[1]]);
    let v = (version >> 12) & 0x0f;
    let x = (version >> 6) & 0x3f;
    let y = version & 0x3f;
    // println!("{:?} {:?} {:?}", v, x, y);
    let vid = u16::from_le_bytes([buf_recv[2], buf_recv[3]]);
    let pid = u16::from_le_bytes([buf_recv[4], buf_recv[5]]);
    // println!("{:?} {:?}", vid, pid);
    let (msd, swim, jtag) = if pid == STLINK_V2_1_PID || pid == STLINK_V2_1_NO_MSD_PID {
        if (x <= 22 && y == 7) || (x >= 25 && y >= 7 && y <= 12) {
            (x, y, 0)
        } else {
            (y, 0, x)
        }
    } else {
        (0, y, x)
    };
    let bridge = 0;
    if v == 3 && x == 0 && y == 0 {
        // todo: write, verify and test STLink v3 special command
        // here bridge may change
    }
    // println!("{:?}", (msd, swim, jtag));
    let jtag_api = match v /*STLink version*/ {
        1 if jtag >= 11 => JtagApi::V2,
        1 if jtag < 11 => JtagApi::V1,
        2 => JtagApi::V2,
        3 => JtagApi::V3,
        _ => unimplemented!("version not in 1, 2 or 3")
    };
    // API for trace from J13, or from STLink v3
    let has_trace = (v == 2 && jtag >= 13) || v == 3;
    // prefer new R/W status read command from J15 or v3
    let has_get_last_rwstatus2 = (v == 2 && jtag >= 15) || v == 3;
    // set SWD frequency from J22
    let has_swd_set_freq = v == 2 && jtag >= 22;
    // set JTAG frequency from J24
    let has_jtag_set_freq = v == 2 && jtag >= 24;
    // read/write memory at 16 bit from J26
    let has_mem_16bit = (v == 2 && jtag >= 26) || v == 3;
    
    Ok(Version { 
        stlink_version: v as u8, 
        jtag_api,
        jtag: jtag as u8,
        swim: swim as u8,
        msd: msd as u8,
        bridge,
        vid,
        pid,
        has_trace,
        has_get_last_rwstatus2,
        has_swd_set_freq,
        has_jtag_set_freq,
        has_mem_16bit,
    })
}
