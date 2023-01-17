use science_servers_rs::PositionServer;

fn main() {
    let subsystem = "science";
    let device = "stepper_motor";
    let server = PositionServer::new(subsystem, device).unwrap();
    server.run();
}