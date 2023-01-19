use science_servers_rs::CameraServer;

fn main() {
    let device = String::from("ir_camera");
    let camera_id = 0;
    let server = CameraServer::new(device, camera_id).unwrap();
    server.run();
}