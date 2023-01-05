use std::env;
use opencv::{highgui, prelude::*, videoio};
use cv_bridge::CvImage;
use anyhow::{Error, Result};

fn main() -> Result<(), Error> {
    let context = rclrs::Context::new(env::args())?;

    let mut node = rclrs::create_node(&context, "rust_subscriber")?;

    let mut num_messages: usize = 0;

    let _subscription = node.create_subscription::<sensor_msgs::msg::String, _>(
        "topic",
        rclrs::QOS_PROFILE_DEFAULT,
        move |msg: sensor_msgs::msg::Image| {
            num_messages += 1;
            println!("(Got {} messages so far)", num_messages);
            let gui = false;
            if gui {
                let mut frame = CvImage.from_imgmsg(frame).as_cvmat("bgr8");
                if frame.size()?.width > 0 {
                    highgui::imshow(window, &mut frame)?;
                }
                let key = highgui::wait_key(10)?;
                if key > 0 && key != 255 {
                    break;
                }
            }
            
        },
    )?;

    rclrs::spin(&node).map_err(|err| err.into())
}