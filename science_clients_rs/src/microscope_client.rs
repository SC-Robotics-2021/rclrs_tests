use science_clients_rs::CameraClient;
 
fn main() {
    let client = CameraClient::new(subsystem="science", device="microscope").unwrap();
    let _ = client.cli_control();
}