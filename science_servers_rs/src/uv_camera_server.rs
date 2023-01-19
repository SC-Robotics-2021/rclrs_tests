use science_servers_rs::CameraServer;

fn main() {
    let device = String::from("uv_camera");
    let camera_id = 4;
    let server = CameraServer::new(device, camera_id).unwrap();
    server.run();
}