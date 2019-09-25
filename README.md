# nihao

[Under development] Rust powered flash programmer and on chip debugger for embedded devices.

This is a library project, further GUI module should be separated into another project repository.

It's under heavy development, but this library have its advantages compared to OpenOCD. 
`nihao` is stand-alone thus does not depend on library written in the C programming language 
(instead it uses `winapi` and other OS-specific application interface crates). 
`nihao` has the ability to flash two (and more) chips at the same time using overlapped structures,
thus it would be unnecessary to unplug and replug to flash your multi-device communicating programs
into two separate MCU chips. 

`nihao` consists of two modules by now. `nihao-usb` is a stand-alone USB library aimed to support
basic control transfers and data exchanges with support of overlapped transportations. 
`nihao-stlink` wraps ST-Link dongles as abstract SWD and JTAG debuggers, allowing read, modify and 
erase target STM32 and STM8 chips.

By now, only limited support of Windows system and ST-Link programmers are finished.
However, all contributions are welcomed! Please fire an issue or submit your pull request if you want to contribute.
