# LoRa Image Noise Cleaning

This is a repository for the LoRa Autoencoder project, which is focused on testing the performance of autoencoders in removing noise from LoRa transmissions. The project includes a low level package that can be built for both transmitters and receivers, along with a working autoencoder code.

## Project Overview

LoRa is a wireless communication technology that is commonly used for long-range, low-power applications such as Internet of Things (IoT) devices. 
However, LoRa transmissions can be susceptible to noise, which can lead to errors in data transmission. 
Autoencoders are a type of neural network that can be used to remove noise from data.

The goal of this project is to test the performance of autoencoders in removing noise from LoRa transmissions. 
To achieve this, we have created a low level package that can be built for both transmitters and receivers. 
The package includes code for transmitting and receiving LoRa packets, as well as code for adding noise to the packets.

There is also a working autoencoder code, which is available in a Google Colab notebook. 
The notebook includes code for training and testing the autoencoder on LoRa packets with added noise. 
The notebook shows that the autoencoder is able to effectively remove the noise from the packets, improving the accuracy of data transmission.

The remaining parts of the project involve building the trained autoencoder model into a file and embedding it into the low level package. 
This will allow the autoencoder to be used with a Rust TensorFlow library, which will enable us to test the performance of the autoencoder in real-world scenarios.

## Getting Started
To get started with this project, you can clone the repository and build the low level package. 
The package includes instructions for building the package for both transmitters and receivers.

You can also explore the Google Colab notebook, which includes code for training and testing the autoencoder on LoRa packets with added noise. 
The notebook provides a step-by-step guide for working with the autoencoder code.

## Building
After installing rust, you must install cargo make which streamlines all of the setup pains
```bash
cargo install cargo-make
```

Then you must setup all of the cargo tools used in this projects for the ESP32
```bash
cargo make setup
```

Now you can upload the code to either the sender
```bash
cargo make flash-send
```
or the receiver
```bash
cargo make flash-receiver
```

and read the output from the receiver like so
```bash
cargo make monitor-default
```

or 
```bash
cargo make monitor /dev/usb_path
```
if you have a custom setup for the ESP32