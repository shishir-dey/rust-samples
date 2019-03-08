#[derive(Debug)]
struct Device {
    id: u8,
    model: u8,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
struct USB_Device {
    d: Device,
    class: u8,
}

fn main() {
    let s: USB_Device = USB_Device {d: Device {id: 0, model: 4}, class: 2};
    println!("s = {:?}", s);
}