use std::env;
use opencv::{prelude::*, videoio};
use cv_bridge::CvImage;
use sensor_msgs::msg::Image;
use anyhow::{Error, Result};

struct CameraPublisher {
    node: rclrs::Node,
    publisher: rclrs::Publisher<Image>,
}

impl CameraPublisher {
    fn new(context: &rclrs::Context) -> Result<Self, Error> {
        let mut node = rclrs::Node::new(context, "camera_subscriber")?;
        let publisher = node.create_publisher::<Image>("topic", rclrs::QOS_PROFILE_DEFAULT)?;
        Ok(Self{node, publisher})
    }
}
 
fn main() -> Result<(), Error> {
    let context = rclrs::Context::new(env::args())?;        
    let camera_publisher = CameraPublisher::new(&context)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if videoio::VideoCapture::is_opened(&cam)? {
        std::thread::spawn(move || -> Result<(), Error> {
            loop {
                println!("Publishing frame!");
                let mut frame = Mat::default();
                cam.read(&mut frame)?;
                let msg = CvImage::from_cvmat(frame).into_imgmsg();
                camera_publisher.publisher.publish(msg)?;
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        });
        rclrs::spin(&camera_publisher.node);
    }
    Ok(())
}