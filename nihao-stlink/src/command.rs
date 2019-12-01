use crate::consts::*;
use std::io;

// todo: we should use overlapped api (async await)

pub(crate) fn command(handle: &nihao_usb::Handle, cmd0: u8, cmd1: u8, resp_len: usize) -> io::Result<Vec<u8>> {
    let mut s = vec![0u8; STLINK_CMD_SIZE_V2];
    s[0] = cmd0;
    s[1] = cmd1;
    let mut r = vec![0u8; resp_len];
    handle.write_pipe(STLINK_TX_EP, &s)?;
    handle.read_pipe(STLINK_RX_EP, &mut r)?;
    Ok(r)
}

pub(crate) fn debug_command(handle: &nihao_usb::Handle, cmd0: u8, cmd1: u8, resp_len: usize) -> io::Result<Vec<u8>> {
    let mut s = vec![0u8; STLINK_CMD_SIZE_V2];
    s[0] = STLINK_DEBUG_COMMAND;
    s[1] = cmd0;
    s[2] = cmd1;
    let mut r = vec![0u8; resp_len];
    handle.write_pipe(STLINK_TX_EP, &s)?;
    handle.read_pipe(STLINK_RX_EP, &mut r)?;
    Ok(r)
}
