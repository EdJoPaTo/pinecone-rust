# PineCone BL602 Rust Guide

Based on...

[lupyuen BL602 Rust Guide](https://github.com/lupyuen/bl602-rust-guide)
[Sipeed BL602 Rust Guide](https://github.com/sipeed/bl602-rust-guide)


## Build and run

```bash
cargo install blflash

git clone https://github.com/EdJoPaTo/pinecone-rust
cd pinecone-rust
rustup target add riscv32imac-unknown-none-elf

# Set Jumper / Switch to H(igh) and press Reset

cargo run --example bl602-gpio-blinky

# Set Jumper / Switch to L(ow) and press Reset

# enjoy the RGB LED blinking
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
