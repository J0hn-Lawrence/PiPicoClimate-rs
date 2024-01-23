# PiPicoClimate-rs

This project uses Raspberry Pi Pico W ([`rp2040`][1]) and a Pimoroni sensor board together with the [`embassy`][2] asynchronous embedded development framework for [Rust][3] to measure sensor data and send that data out via MQTT.

DISCLAIMER: Parts of this README.md are copied from [here][26], an excellent repository to get started with Embassy and Rust on the Raspberry Pi Pico.

It includes all of the [`knurling-rs`][4] tooling ([`defmt`][5], [`defmt-rtt`][5], [`panic-probe`][5], [`flip-link`][6],
[`probe-run`][7]) to enhance the embedded development process.

The default [`cargo`][8] runner is configured as [`probe-run`][7], so you can build, flash and run your firmware _with_ output from the device via a [`probe-rs`][9] compatible debug probe with the command:

```shell
cargo run
```

If you want to use a different runner with your debugger (e.g., [`cargo-embed`][10], [`probe-rs-debugger`][11], etc.) or if you _aren't_ using a debugger and want the runner to flash the firmware via USB (e.g., [`elf2uf2-rs`][12], [`picotool`][13], etc.) then see: [Alternative Runners][14]

- [PiPicoClimate-rs](#pipicoclimate-rs)
  - [Requirements](#requirements)
  - [Setup](#setup)
    - [System Setup](#system-setup)
    - [Picoprobe Setup](#picoprobe-setup)
    - [Hardware Setup](#hardware-setup)
      - [Connecting the Raspberry Pi Pico Debug Probe](#connecting-the-raspberry-pi-pico-debug-probe)
      - [Raspberry Pi Pico I/O expansion board](#raspberry-pi-pico-io-expansion-board)
  - [Usage](#usage)
    - [Running](#running)
    - [Logging](#logging)
  - [Appendix](#appendix)
    - [Documentation](#documentation)
    - [Resources](#resources)
  - [License](#license)

## Requirements

- Ubuntu or Fedora Linux
- Raspberry Pi Pico
- Debug Probe (_or_ another Raspberry Pi Pico)
- Rust Toolchain ([`cargo`][8], [`rustup`][15])

## Setup

### System Setup

1. Install [Rust][3] and [`cargo`][8] using [`rustup`][15]

    ```shell
    # Install `rustup` for Rust Toolchain
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Install Cortex-M Target Toolchain Support for [`Rust`][3]

    ```shell
    # Install `thumbv6m-none-eabi` Target for `rp2040`
    rustup target add thumbv6m-none-eabi
    ```

3. Install [`probe-run`][7]

    ```shell
    # Install Linux Dependencies
    # For Ubuntu
    sudo apt install -y libusb-1.0-0-dev libudev-dev
    # For Fedora
    sudo dnf install -y libusb1

    # Install `probe-run`
    cargo install probe-run

    # Install `udev` Rules and Reload
    sudo curl https://probe.rs/files/69-probe-rs.rules -o /etc/udev/rules.d/69-probe-rs.rules
    sudo udevadm control --reload
    sudo udevadm trigger

    # (Optional and Ubuntu ONLY!) Add User to `plugdev` Group
    sudo usermod -aG plugdev $USER
    ```

4. Install [`flip-link`][6]

    ```shell
    # Install `flip-link`
    cargo install flip-link
    ```cargo

### Picoprobe Setup

You can use a second Raspberry Pi Pico (a Pico W is also fine) as a so called Picoprobe. This way you can flash your firmware without having to set the Pico inti bootloader mode everytime. To set this up, follow these steps:

1. Download the Picoprobe firmware from [here][27]
2. Boot the Raspberry Pi Pico that you intend to use as a Picoprobe in "Bootloader Mode" by holding the _BOOTSEL_ button while plugging it in via USB
3. Open the mounted Raspberry Pi Pico storage device in your file explorer
4. Copy the `picoprobe.uf2` onto the Raspberry Pi Pico
5. Firmware will be flashed to the Raspberry Pi Pico and it will disconnect itself

Whenever you now plug in the Raspberry Pi Pico, it will automatically connect to your computer as a Picoprobe. You can now use it as a debug probe with [`probe-run`][7]. This is the recommended way to flash your firmware and is automatically configured in this project.

### Hardware Setup

#### Connecting the Raspberry Pi Pico Debug Probe

The diagram below shows the wiring between Raspberry Pi Pico A (left) and Raspberry Pi Pico B (right), configuring Raspberry Pi Pico A as a debug probe.

<!-- Embed Image -->
![Image description](https://user-images.githubusercontent.com/62866982/191892108-daabc0d6-5ec1-4265-8722-226c512b995c.svg)

The connections shown in the diagram above are listed below.

``` text
Pico A GND -> Pico B GND
Pico A GP2 -> Pico B SWCLK
Pico A GP3 -> Pico B SWDIO
Pico A GP4/UART1 TX -> Pico B GP1/UART0 RX
Pico A GP5/UART1 RX -> Pico B GP0/UART0 TX
Pico A VSYS -> Pico B VSYS
```

For more information on connecting the two Raspberry Pi Picos, the wiring between them and its connections, seethe section _Appendix A > Wiring Loom_ in: [Getting Started with Raspberry Pi Pico][18]

#### Raspberry Pi Pico I/O expansion board

We are using [this][28] I/O expansion board from AliExpress, which allows us to connect the Pimoroni enviro plus sensor board to the Raspberry Pi Pico while still beeing able to access the Picos pins in order to flash the firmware via the connected Picoprobe.

Further details TBD.

## Usage

### Running

To run the firmware in debug mode:

```shell
cargo run
```

To run the firmware in release mode:

```shell
cargo run --release
```

NOTE: If you flash the firmware in release mode, the code will run upon boot of your Pico. So if you disconnect and reconnect the Pico, the firmware will run again. If you flash the firmware in debug mode, the code will only run when you start the firmware with `cargo run`, after disconnecting and reconnecting the Pico, the firmware will not run again.

### Logging

To change the default [`defmt`][5] log level, see `.cargo/config.toml`:

```toml
[env]
DEFMT_LOG = "trace"
```

You can also set the log level inline:

```shell
DEFMT_LOG=debug cargo run
DEFMT_LOG=error cargo run --release
```

## Appendix

### Documentation

- [Raspberry Pi Pico][1]
- [Rust][3]
- [Cargo][8]
- [Rustup][15]
- [Embassy][2]
- [Knurling-RS `defmt`][5]
- [Knurling-RS `flip-link`][6]
- [Knurling-RS `probe-run`][7]
- [Probe-RS `cargo-embed`][10]
- [Probe-RS `probe-rs-debugger`][11]
- [Raspberry Pi Pico `elf2uf2`][12]
- [Raspberry Pi Pico `picotool`][13]
- [CMSIS-DAP Firmware `DapperMime`][16]

### Resources

- [Rust Embedded Book][20]
- [Awesome Embedded Rust][21]
- [Getting Started with Raspberry Pi Pico][22]
- [Ferrous Systems Embedded Training][23]
- [Ferrous Systems Embedded Teaching Material][24]
- [RP-RS App Template][25]
- [RP-RS Alternative Debug Probes][17]
- [RP-RS Alternative Runners][14]
- [Knurling-RS App Template][4]
- [Probe-RS Probe Setup][9]
- [Raspberry Pi Pico Dev Board][19]

<!-- Reference -->
[1]: https://www.raspberrypi.com/documentation/microcontrollers/rp2040.html
[2]: https://embassy.dev/dev/index.html
[3]: https://www.rust-lang.org/
[4]: https://github.com/knurling-rs/app-template
[5]: https://github.com/knurling-rs/defmt
[6]: https://github.com/knurling-rs/flip-link
[7]: https://github.com/knurling-rs/probe-run
[8]: https://doc.rust-lang.org/cargo/
[9]: https://probe.rs/docs/getting-started/probe-setup/
[10]: https://github.com/probe-rs/cargo-embed
[11]: https://github.com/probe-rs/vscode
[12]: https://github.com/JoNil/elf2uf2-rs
[13]: https://github.com/raspberrypi/picotool
[14]: https://github.com/rp-rs/rp2040-project-template#alternative-runners
[15]: https://rustup.rs/
[16]: https://github.com/majbthrd/DapperMime
[17]: https://github.com/rp-rs/rp2040-project-template/blob/main/debug_probes.md
[18]: https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf#picoprobe-wiring-section
[19]: https://timsavage.github.io/rpi-pico-devboard/
[20]: https://docs.rust-embedded.org/book/
[21]: https://github.com/rust-embedded/awesome-embedded-rust
[22]: https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf
[23]: https://embedded-trainings.ferrous-systems.com/
[24]: https://github.com/ferrous-systems/teaching-material
[25]: https://github.com/rp-rs/rp2040-project-template
[26]: https://github.com/SupImDos/embassy-rp-skeleton/blob/main/README.md
[27]: https://github.com/raspberrypi/picoprobe/releases
[28]: https://de.aliexpress.com/i/1005002345808710.html

## License

Licensed under MIT license [LICENSE](LICENSE)
