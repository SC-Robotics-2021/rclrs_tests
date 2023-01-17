use science_servers_rs::PositionClient;

fn main() {
    let server = PositionClient::new(subsystem="science", device="stepper_motor").unwrap();
    server.run();
}