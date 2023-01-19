use science_servers_rs::GPIOServer;

fn main() {
    let device = String::from("brush");
    let pin_num = 13;
    let server = GPIOServer::new(device, pin_num).unwrap();
    server.run();
}