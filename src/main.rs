use embedded_hal::serial::{Read, Write};
use mh_z19;
use nb::block;
use serial::PortSettings;
use serial_embedded_hal::Serial;
use std::thread::sleep;
use std::time::Duration;

// 'core' ecosystem has implementation of core::fmt::Write for this
fn write<Error>(tx: &mut impl Write<u8, Error = Error>, data: &[u8]) -> Result<(), Error> {
    data.into_iter().map(|c| block!(tx.write(*c))).collect()
}

fn read<Error>(rx: &mut impl Read<u8, Error = Error>, bytes: usize) -> Result<Vec<u8>, Error> {
    let mut vec = Vec::with_capacity(bytes);
    for _ in 0..bytes {
        vec.push(block!(rx.read())?);
    }
    Ok(vec)
}

fn main() {
    let (mut tx, mut rx) = Serial::new(
        "/dev/tty.wchusbserialfd130",
        &PortSettings {
            baud_rate: serial::Baud9600,
            char_size: serial::Bits8,
            parity: serial::ParityNone,
            stop_bits: serial::Stop1,
            flow_control: serial::FlowNone,
        },
    )
    .unwrap()
    .split();

    loop {
        write(&mut tx, &mh_z19::read_gas_concentration(1)).unwrap();
        let result = read(&mut rx, 9);
        if result.is_err() {
            println!("Error reading result");
            continue;
        }
        let concentration = mh_z19::parse_gas_contentration_ppm(&result.unwrap());
        match concentration {
            Ok(c) => println!("{} ppm", c),
            Err(e) => println!("Error: {}", e),
        };

        // In embedded applications
        // embedded_hal::blocking::delay::DelayMs should be used,
        // however I didn't found implementation of it for std
        sleep(Duration::from_secs(2));
    }
}
