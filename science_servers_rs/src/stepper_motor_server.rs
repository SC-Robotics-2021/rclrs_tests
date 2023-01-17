use science_servers_rs::CameraClient;

fn main() {
    let server = CameraClient::new(subsystem="science", device="stepper_motor", camera_settings=CameraClient::define_settings(frame_height=640, frame_height=480, fps=30));
    server.run();
}