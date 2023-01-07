use std::env;
use opencv::{prelude::*, videoio};
use anyhow::{Error, Result};
use cv_bridge::

struct CameraPublisher {
    node: rclrs::Node,
    _publisher: rclrs::Publisher<Image>,
}

impl CameraPublisher {
    fn new(context: &rclrs::Context) -> Result<Self, Error> {
        let mut node = rclrs::Node::new(context, "camera_subscriber")?;
        let _publisher = node.create_publisher::<sensor_msgs::msg::Image>("topic", rclrs::QOS_PROFILE_DEFAULT)?;
        Ok(Self{node, _subscription, gui})
    };
};
 
fn main() -> Result<(), Error> {
    let context = rclrs::Context::new(env::args())?;        
    let camera_subscriber = CameraPublisher::new(context)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if !videoio::VideoCapture::is_opened(&cam) {
        Err(())
    }
    while context.ok() {
        println!("Publishing frame!");
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        let msg = CvImage::from_cvmat(frame).into_imgmsg();
        publisher.publish(msg)?;
        std::thread::sleep(std::time::Duration::from_millis(500));
	}
    Ok(())
}