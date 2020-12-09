﻿# PineCone BL602 OpenOCD and Rust Guide

The JTAG Port needs to be remapped to other GPIO Pins. Refer to the article...

["Quick Peek of PineCone BL602 RISC-V Evaluation Board"](https://lupyuen.github.io/articles/pinecone)

[How to remap the JTAG port](https://github.com/lupyuen/bl_iot_sdk/releases/tag/v0.0.4)

## Install and run OpenOCD for Windows

Connect Sipeed JTAG Debugger to PineCone: TMS, TCK, TDI, TDO, GND 

Connect PineCone and Sipeed JTAG Debugger to our computer (Yes we need two USB ports)

Install driver for Sipeed JTAG Debugger...

https://docs.platformio.org/en/latest/plus/debug-tools/sipeed-rv-debugger.html#drivers

Using the Zadig Tool, install the WinUSB Driver for BOTH `Dual RS232 (Interface 0)` and `Dual RS232 (Interface 1)`

Download OpenOCD from...

https://github.com/xpack-dev-tools/openocd-xpack/releases/download/v0.10.0-15/xpack-openocd-0.10.0-15-win32-x64.zip

Run OpenOCD...

```cmd
git clone --recursive https://github.com/sipeed/bl602-pac
git clone --recursive https://github.com/sipeed/bl602-hal
git clone --recursive https://github.com/lupyuen/pinecone-rust
cd pinecone-rust
# TODO: Check openocd.cfg, verify that the FTDI channel is 0: "ftdi_channel 0"
# TODO: Unzip OpenOCD into bl602-rust-guide\xpack-openocd-0.10.0-15
xpack-openocd-0.10.0-15\bin\openocd.exe
```

We should see...

```
C:\pinecone\bl602-rust-guide>..\xpack-openocd-0.10.0-15\bin\openocd.exe
xPack OpenOCD, x86_64 Open On-Chip Debugger 0.10.0+dev (2020-10-13-17:29)
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
Ready for Remote Connections
Info : clock speed 100 kHz
Info : JTAG tap: riscv.cpu tap/device found: 0x20000c05 (mfg: 0x602 (<unknown>), part: 0x0000, ver: 0x2)
Info : datacount=1 progbufsize=2
Info : Disabling abstract command reads from CSRs.
Info : Examined RISC-V core; found 1 harts
Info :  hart 0: XLEN=32, misa=0x40801125
Info : starting gdb server for riscv.cpu.0 on 3333
Info : Listening on port 3333 for gdb connections
Info : JTAG tap: riscv.cpu tap/device found: 0x20000c05 (mfg: 0x602 (<unknown>), part: 0x0000, ver: 0x2)
reset-assert-pre
reset-deassert-post
Info : Disabling abstract command writes to CSRs.
reset-init
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
```

Note: For Sipeed JTAG Debugger, FTDI channel must be 0 in [`openocd.cfg`](openocd.cfg)...

```
ftdi_channel 0
```

## Install and run OpenOCD for macOS

Connect Sipeed JTAG Debugger to PineCone: TMS, TCK, TDI, TDO, GND 

Connect PineCone and Sipeed JTAG Debugger to our computer (Yes we need two USB ports)

Install driver for Sipeed JTAG Debugger...

https://docs.platformio.org/en/latest/plus/debug-tools/sipeed-rv-debugger.html#drivers

Enter...

```bash
git clone --recursive https://github.com/sipeed/bl602-pac
git clone --recursive https://github.com/sipeed/bl602-hal
git clone --recursive https://github.com/lupyuen/pinecone-rust
cd pinecone-rust
rustup target add riscv32imac-unknown-none-elf
cargo build

# Alternatively...
# cargo build --example bl602-gpio-blinky

wget https://github.com/xpack-dev-tools/riscv-none-embed-gcc-xpack/releases/download/v8.3.0-2.3/xpack-riscv-none-embed-gcc-8.3.0-2.3-darwin-x64.tar.gz
tar xvf xpack-riscv-none-embed-gcc-8.3.0-2.3-darwin-x64.tar.gz
ln -s "$PWD/xpack-riscv-none-embed-gcc-8.3.0-2.3/bin/riscv-none-embed-gdb" "$PWD/xpack-riscv-none-embed-gcc-8.3.0-2.3/bin/riscv64-unknown-elf-gdb"

wget https://github.com/xpack-dev-tools/openocd-xpack/releases/download/v0.10.0-14/xpack-openocd-0.10.0-14-darwin-x64.tar.gz
tar -xvf xpack-openocd-0.10.0-14-darwin-x64.tar.gz
xpack-openocd-0.10.0-14/bin/openocd
```

We should see...

```

xPack OpenOCD, x86_64 Open On-Chip Debugger 0.10.0+dev-00378-ge5be992df (2020-06-26-12:31)
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
Ready for Remote Connections
Info : clock speed 100 kHz
Info : JTAG tap: riscv.cpu tap/device found: 0x20000c05 (mfg: 0x602 (<unknown>), part: 0x0000, ver: 0x2)
Info : datacount=1 progbufsize=2
Info : Disabling abstract command reads from CSRs.
Info : Examined RISC-V core; found 1 harts
Info :  hart 0: XLEN=32, misa=0x40801125
Info : starting gdb server for riscv.cpu.0 on 3333
Info : Listening on port 3333 for gdb connections
Info : JTAG tap: riscv.cpu tap/device found: 0x20000c05 (mfg: 0x602 (<unknown>), part: 0x0000, ver: 0x2)
reset-assert-pre
reset-deassert-post
Info : Disabling abstract command writes to CSRs.
reset-init
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
```

If the Sipeed JTAG Debugger is not detected, we will see in OpenOCD...

```
Error: no device found
Error: unable to open ftdi device with vid 0403, pid 6010, description '*', serial '*' at bus location '*'
```

If we see in OpenOCD...

```
Error: failed read at 0x11, status=1
Error: Hart 0 is unavailable.
Error: Hart 0 doesn't exist.
Info : Hart 0 unexpectedly reset!
Error: failed read at 0x11, status=1
```

...Check that the GND pin is connected from the Sipeed JTAG Debugger to PineCone.

Note: For Sipeed JTAG Debugger, FTDI channel must be 0 in [`openocd.cfg`](openocd.cfg)...

```
ftdi_channel 0
```

## TODO: Flash Rust app to PineCone

Open another Command Prompt and enter...

```bash
cd pinecone-rust
export PATH="$PWD/xpack-riscv-none-embed-gcc-8.3.0-2.3/bin:$PATH"
cargo run

# Alternatively...
# cargo run --example bl602-gpio-blinky
```

## License

This project is licensed under either of Mulan PSL v2 or MIT.

```
Copyright (c) 2020 Sipeed Co.,Ltd.
bl602-hal is licensed under Mulan PSL v2.
You can use this software according to the terms and conditions of the Mulan PSL v2.
You may obtain a copy of Mulan PSL v2 at:

    http://license.coscl.org.cn/MulanPSL2

THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
See the Mulan PSL v2 for more details.
```
