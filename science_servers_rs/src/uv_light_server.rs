use science_servers_rs::GPIOServer;

fn main() {
    let subsystem = "science";
    let device = "uv_light";
    let pin_num = 16;
    let server = GPIOServer::new(subsystem, device, pin_num).unwrap();
    server.run();
}