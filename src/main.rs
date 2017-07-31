extern crate libusb;
use std::io::Write;
use std::error::Error;
use std::time::Duration;

use libusb::Context;

fn main() {
    let usbctxt = Context::new().unwrap();
    if !usbctxt.has_hid_access() {
        writeln!(std::io::stderr(), "Can't access HID devices").unwrap();
        return;
    }
    let devices = 
        match usbctxt.devices() {
            Ok(d) => d,
            Err(e) => {
                 writeln!(std::io::stderr(), "Failed to list USB devices: {}", e.description()).unwrap();
                return
            },
        };
    let mut device = None;
    for d in devices.iter() {
        if let Ok(desc) = d.device_descriptor() {
            println!("{:04x}:{:04x}",desc.vendor_id(), desc.product_id());
            match (desc.vendor_id(), desc.product_id()) {
                (0x2222, 0x3060) => {
                    if device.is_some() {
                        writeln!(std::io::stderr(), 
                                 "More than one usable devices found").unwrap();
                        return
                    }
                    device = Some(d);
                },
                _ => {} 
            }
        }
    }
    let mut handle = 
        match device {
            Some(dev) => 
                match dev.open() {
                    Ok(h) => h,
                    Err(e) => {
                        writeln!(std::io::stderr(), 
                                 "Failed to open USB device: {}", 
                                 e.description()).unwrap();
                        return
                    }
                },
            None => {
                writeln!(std::io::stderr(), 
                         "No usable device found").unwrap();
                return
            }
        };
    match handle.detach_kernel_driver(0) {
        _ => {}
    }
    if let Err(e) = handle.claim_interface(0) {
        writeln!(std::io::stderr(), 
                 "Failed to claim interface: {}", 
                 e.description()).unwrap();
        return
    }
    loop {
        let mut buf = [0; 4];
        match handle.read_interrupt(0x81, &mut buf,Duration::new(0,0)) {
            Ok(n) => {
                println!("Got {} bytes",n);
                println!("{:?}",&buf[0..n]);
            },
            Err(e) => {
                writeln!(std::io::stderr(), 
                         "Reading from USB device failed: {}", 
                         e.description()).unwrap();   
            }
        }
                
    }
}
