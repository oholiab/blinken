Blinken
=======

A program for controlling the serial controlled LED strip from
[ledstrip_fun](https://github.com/mikepea/ledstrip_fun) written in Rust.

To build
--------
    git clone https://github.com/oholiab/blinken
    cargo build

To run
------
Usage is:

    blinken {usb_serial_device_name}

so on macOS:

    ./target/blinken /dev/cu.wchusbserial1420
