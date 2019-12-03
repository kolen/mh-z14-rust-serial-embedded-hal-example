# Example of using MH-Z14/MH-Z19 sensor with Rust, `mh-z19` crate and `serial-embedded-hal`

This is example of using [`mh-z19`](https://crates.io/crates/mh-z19)
crate to read MH-Z19/MH-Z14 sensor (which have the same protocol)
readings.

In this example, the sensor is connected to PC via USB-to-UART
adapter, therefore
[`serial-embedded-hal`](https://crates.io/crates/serial-embedded-hal)
crate is used. It allows to use `embedded_hal::serial`, a piece of
embedded ecosystem, on regular PC, with the same interface.

Device file name is hardcoded in source. When running, it outputs
readings in PPM every 2 seconds.
