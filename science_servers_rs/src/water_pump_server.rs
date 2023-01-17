use science_servers_rs::GPIOServer;

fn main() {
    let subsystem = "science";
    let device = "water_pump";
    let pin_num = 22;
    let server = GPIOServer::new(subsystem, device, pin_num).unwrap();
    server.run();
}