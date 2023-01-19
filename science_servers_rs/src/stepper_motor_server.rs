use science_servers_rs::StepperMotorServer;

fn main() {
    let server = StepperMotorServer::new().unwrap();
    server.run();
}