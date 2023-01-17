use science_servers_rs::PositionServer;

fn main() {
    let device = "stepper_motor";
    let server = PositionServer::new(device).unwrap();
    server.run();
}