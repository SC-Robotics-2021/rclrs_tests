use science_servers_rs::CameraServer;

fn main() {
    let subsystem = "science";
    let device = "ir_camera";
    let camera_settings = CameraServer::define_settings(frame_height=640, frame_height=480, fps=30);
    let server = CameraServer::new().unwrap();
    server.run();
}