use science_servers_rs::GPIOServer;

fn main() {
    let subsystem = "science";
    let device = "vacuum";
    let pin_num = 18;
    let server = GPIOServer::new(subsystem, device, pin_num).unwrap();
    server.run();
}