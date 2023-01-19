use science_servers_rs::GPIOServer;

fn main() {
    let device = String::from("uv_light");
    let pin_num: u8 = 22;
    let server = GPIOServer::new(device, pin_num).unwrap();
    server.run();
}