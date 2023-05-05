use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::{config, Dma, SpiDeviceDriver};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

const FREQUENCY: i64 = 915;

const IMAGE: [u8; 784] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 84, 185, 159, 151, 60, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 254, 254, 254, 254, 241, 198, 198, 198, 198, 198, 198, 198,
    198, 170, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 114, 72, 114, 163, 227, 254, 225, 254,
    254, 254, 250, 229, 254, 254, 140, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 66,
    14, 67, 67, 67, 59, 21, 236, 254, 106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 83, 253, 209, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 22, 233, 255, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    129, 254, 238, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59,
    249, 254, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 133, 254,
    187, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 205, 248, 58,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 254, 182, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 251, 240, 57, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 221, 254, 166, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 203, 254, 219, 35, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 254, 254, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 224, 254, 115, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 133, 254, 254, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 242, 254, 254, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 121, 254, 254, 219, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 121, 254, 207, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

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
    let device =
        SpiDeviceDriver::new_single(spi, sck, miso, Some(mosi), Dma::Disabled, Some(go), &config)?;

    println!("Initializing LoRa");
    let mut lora = sx127x_lora::LoRa::new(device, cs, rts, FREQUENCY, Ets).unwrap();

    lora.set_tx_power(17, 1).unwrap();

    let mut init_msg = [0; 255];
    init_msg[0] = 2;
    init_msg[1] = 2;
    init_msg[2] = 2;

    loop {
        // Send
        #[cfg(feature = "send")]
        {
            println!("Sending Image");

            let mut buffer = [0; 255];

            let mut part: i32 = -1;
            while part < 3 {
                if !lora.transmitting().unwrap() {
                    let mut size = 0;
                    if part == -1 {
                        buffer[0] = 2;
                        buffer[1] = 2;
                        buffer[2] = 2;
                        size = 3;
                    } else {
                        let limit = if part == 3 { 255 - 19 } else { 255 };
                        let prefix = (part * 255) as usize;
                        for b in 0..limit {
                            buffer[b] = IMAGE[prefix + b];
                        }
                        size = limit;
                    }

                    let transmit = lora.transmit_payload(buffer, size);

                    match transmit {
                        Ok(_) => {
                            part += 1;
                            println!("Sent packet");
                        }
                        Err(_) => println!("Error"),
                    }
                }
            }
        }

        // Receive
        #[cfg(feature = "receive")]
        {
            let mut packets_received: i32 = -1;
            let mut received_image = [0; 784];
            while packets_received < 3 {
                let poll = lora.poll_irq(Some(2000)); //2 Second timeout
                match poll {
                    Ok(size) => {
                        let buffer = lora.read_packet().unwrap();
                        #[cfg(feature = "debug-data")]
                        {
                            println!("packet: {:?}", buffer);

                            let rssi = lora.get_packet_rssi().unwrap();
                            println!("RSSI: {}", rssi);

                            let snr = lora.get_packet_snr().unwrap();
                            println!("SNR: {}", snr);

                            let err = lora.get_packet_frequency_error().unwrap();
                            println!("FE: {}", err);
                        }

                        if buffer == init_msg {
                            println!("Received image request");
                            packets_received += 1;
                        } else if packets_received >= 0 {
                            #[cfg(feature = "debug-data")]
                            println!("Partial image received");
                            let limit: usize = if packets_received == 3 { 255 - 19 } else { 255 };
                            let prefix = (packets_received * 255) as usize;
                            for b in 0..limit {
                                received_image[prefix + b] = buffer[b];
                            }

                            packets_received += 1;
                        }
                    }
                    Err(_) => println!("Timeout"),
                }
            }
            println!("Image: {:?}", received_image);
        }
    }
}
