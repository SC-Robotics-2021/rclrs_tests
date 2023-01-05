use std::env;
use opencv::{prelude::*, videoio};
use anyhow::{Error, Result};
use cv_bridge::CvImage;

fn main() -> Result<(), Error> {
    let context = rclrs::Context::new(env::args())?;

    let node = rclrs::create_node(&context, "rust_publisher")?;

    let publisher =
        node.create_publisher::<sensor_msgs::msg::Image>("topic", rclrs::QOS_PROcd ../_DEFAULT)?;

    let mut message = sensor_msgs::msg::Image::default();

    let mut publish_count: u32 = 1;

    while context.ok() {
        println!("Publishing frame!");
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
        let msg = CvImage.from_cvmat(frame).into_imgmsg();
        publisher.publish(&msg)?;
        std::thread::sleep(std::time::Duration::from_millis(500));
	}

    Ok(())
}