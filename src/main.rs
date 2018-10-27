extern crate hidapi;

fn main() {
    let api = hidapi::HidApi::new().unwrap();
    // Print out information about all connected devices
    let mut device: Option<hidapi::HidDevice> = None;
    for info in api.devices() {
        println!("{:#?}", info);
       
        match  info {
    	    &hidapi::HidDeviceInfo{product_id:0x0060, vendor_id: 0x1658, 
	  		           interface_number: 1, ..} => {
                println!("Device: {:04x}:{:04x} Interface: {} Usage: {}/{}", info.vendor_id, info.product_id, info.interface_number, info.usage_page, info.usage);         
		match info.open_device(&api) {
		    Ok(d) => {
			device = Some(d);
		    }
		    
		    Err(e) => panic!("Failed to open HID device")
		}
		break
	    },
	    _ => {}
        }	  	   
    }
    let device = device.expect("No encoder found");
        
   
    // Read data from device
    let mut buf = [0u8; 64];
    loop {
        let res = device.read(&mut buf[..]).unwrap();
        println!("Read: {:?}", &buf[..res]);
        if buf[0] == 3 && buf[1] == 2 && buf[4] == 4 {
            buf[0] = 4;
            buf[2] = 0x01;
            buf[4] = 0;
            //device.send_feature_report(&buf[..3+4*8-1]).unwrap();
            {
                let l= 3+4*8; 
                match device.write(&buf[..l]) {
                    Ok(s) => println!("Sent {} bytes", s),
                    Err(e) => println!("Failed to send {} bytes: {}", l, e)
                }
            }
        }
    }
    
    
}
