use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::config::DriverConfig;
use esp_idf_hal::spi::{config, Dma, SpiDeviceDriver};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

const FREQUENCY: i64 = 915;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    println!("Getting peripherals");
    let peripherals = Peripherals::take().unwrap();

    println!("SPI");
    let spi = peripherals.spi2;

    println!("Getting pins");
    let mosi = peripherals.pins.gpio13;
    let miso = peripherals.pins.gpio12;
    let sck = peripherals.pins.gpio14;

    println!("CS Output");
    let cs = PinDriver::output(peripherals.pins.gpio25)?;
    println!("RTS Output");
    let rts = PinDriver::output(peripherals.pins.gpio15)?;

    let go = peripherals.pins.gpio27;

    println!("Initializing SPI");
    let config = config::Config::new().baudrate(8.MHz().into());
    let driver_config = DriverConfig::new().dma(Dma::Disabled);
    let device = SpiDeviceDriver::new_single(
        spi,
        sck,
        miso,
        Some(mosi),
        Some(go),
        &driver_config,
        &config,
    )?;

    println!("Initializing LoRa");
    let mut lora = sx127x_lora::LoRa::new(device, cs, rts, FREQUENCY, Ets).unwrap();

    lora.set_tx_power(17, 1).unwrap();

    let send_msg = [11; 255];

    #[allow(clippy::empty_loop)]
    loop {
        // Send
        #[cfg(feature = "send")]
        {
            println!("Sending");

            if !lora.transmitting().unwrap() {
                let transmit = lora.transmit_payload(send_msg, 255);

                match transmit {
                    Ok(_) => {
                        println!("Sent packet");
                    }
                    Err(_) => println!("Error"),
                }
            }
        }

        // Receive
        #[cfg(feature = "receive")]
        {
            let poll = lora.poll_irq(Some(2000)); //2 Second timeout
            match poll {
                Ok(_size) => {
                    let buffer = lora.read_packet().unwrap();
                    println!("packet: {:?}", buffer);

                    let rssi = lora.get_packet_rssi().unwrap();
                    println!("RSSI: {}", rssi);

                    let snr = lora.get_packet_snr().unwrap();
                    println!("SNR: {}", snr);

                    let err = lora.get_packet_frequency_error().unwrap();
                    println!("FE: {}", err);

                    // Check if buffer works
                    for (i, data) in buffer.iter().enumerate() {
                        if *data != send_msg[i] {
                            println!("Mismatch in {}, got {} expected {}", i, send_msg[i], data);
                        }
                    }
                }
                Err(_) => println!("Timeout"),
            }
        }
    }
}
