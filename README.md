# BL602 Rust Guide

## Install and run OpenOCD for macOS

Connect Sipeed JTAG Debugger and enter...

```bash
cd bl602-rust-guide
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
Info : clock speed 2000 kHz
Warn : Haven't made progress in mpsse_flush() for 2039ms.
Warn : Haven't made progress in mpsse_flush() for 4078ms.
Warn : Haven't made progress in mpsse_flush() for 8157ms.
```

If the Sipeed JTAG Debugger is not detected, we will see...

```
Error: no device found
Error: unable to open ftdi device with vid 0403, pid 6010, description '*', serial '*' at bus location '*'
```

## Try it out!

Open one terminal and execute:

```
openocd
```

Open another and execute:

```
cargo run --example bl602-gpio-blinky
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
