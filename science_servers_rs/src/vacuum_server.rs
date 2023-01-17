use science_servers_rs::GPIOServer;

fn main() {
    let server = GPIOServer::new(subsystem="science", device="vacuum", pin_num=18).unwrap();
    server.run();
}