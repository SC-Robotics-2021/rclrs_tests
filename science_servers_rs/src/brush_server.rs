use science_servers_rs::GPIOServer;

fn main() {
    let server = GPIOServer::new(subsystem="science", device="brush", pin_num=13);
    server.run();
}