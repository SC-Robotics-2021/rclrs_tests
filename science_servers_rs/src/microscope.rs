use science_servers_rs::CameraServer;

fn main() {
    let server = CameraServer::new(subsystem="science", device="microscope", camera_settings=CameraServer::define_settings(frame_height=640, frame_height=480, fps=30));
    server.run();
}