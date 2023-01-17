use science_servers_rs::GPIOServer;

fn main() {
    let server = GPIOServer::new(subsystem="science", device="uv_light", pin_num=16).unwrap();
    server.run();
}