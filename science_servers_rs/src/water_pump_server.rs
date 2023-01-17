use science_servers_rs::GPIOServer;

fn main() {
    let server = GPIOServer::new(subsystem="science", device="water_pump", pin_num=22);
    server.run();
}