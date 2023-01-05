use std::env;
use opencv::{prelude::*, videoio};
use anyhow::{Error, Result};
use cv_bridge::CvImage;

fn main() -> Result<(), Error> {
    let context = rclrs::Context::new(env::args())?;

    let node = rclrs::create_node(&context, "rust_publisher")?;

    let publisher =
        node.create_publisher::<sensor_msgs::msg::Image>("topic", rclrs::QOS_PROFILE_DEFAULT)?;

    let mut publish_count: u32 = 0;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    
    if !videoio::VideoCapture::is_opened(&cam)? {
		panic!("Unable to open default camera!");
	}
    while context.ok() {
        publish_count += 1;
        println!("Publishing frame {}!", &publish_count);
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
        let msg = CvImage::from_cvmat(frame).into_imgmsg();
        publisher.publish(msg)?;
        std::thread::sleep(std::time::Duration::from_millis(500));
	}

    Ok(())
}