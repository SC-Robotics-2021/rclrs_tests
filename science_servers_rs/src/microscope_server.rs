use science_servers_rs::CameraServer;

fn main() {
    let device = String::from("microscope");
    let camera_id = 2;
    let server = CameraServer::new(device, camera_id).unwrap();
    server.run();
}