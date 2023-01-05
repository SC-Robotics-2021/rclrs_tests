use std::env;
use opencv::{highgui, prelude::*, videoio};
use cv_bridge::CvImage;
use anyhow::{Error, Result};

fn main() -> Result<(), Error> {
    let context = rclrs::Context::new(env::args())?;
    let window = "video capture";
	highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let mut node = rclrs::create_node(&context, "rust_subscriber")?;

    let mut num_messages: usize = 0;

    let _subscription = node.create_subscription::<sensor_msgs::msg::Image, _>(
        "topic",
        rclrs::QOS_PROFILE_DEFAULT,
        move |msg: sensor_msgs::msg::Image| {
            num_messages += 1;
            println!("(Got {} messages so far)", &num_messages);
            let gui = false;
            if gui {
                let frame = CvImage::from_imgmsg(msg).as_cvmat("bgr8");
                if frame.size()?.width > 0 {
                    highgui::imshow(window, frame)?;
                }
                let key = highgui::wait_key(10)?;
            }
            
        },
    )?;

    rclrs::spin(&node).map_err(|err| err.into())
}