use science_servers_rs::CameraServer;

fn main() {
    let device = String::from("microscope");
    let camera_id = 2;
    let frame_width = 640; 
    let frame_height = 480;
    let fps = 30;
    let camera_settings = CameraServer::define_settings(frame_width, frame_height, fps);
    let server = CameraServer::new(device, camera_id, camera_settings).unwrap();
    server.run();
}