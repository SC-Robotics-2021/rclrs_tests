use science_clients_rs::CameraClient;
 
fn main() {
    let client = CameraClient::new(subsystem="science", device="microscope");
    client.cli_control();
}