use science_clients_rs::CameraClient;
 
fn main() {
    let device = String::from("ir_camera");
    let client = CameraClient::new(device).unwrap();
    let _ = client.cli_control();
}