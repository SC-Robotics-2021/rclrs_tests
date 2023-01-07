use std::env;
use sensor_msgs::msg::Image
use opencv::{highgui, prelude::*};
use cv_bridge::CvImage;
use anyhow::{Error, Result};

struct CameraSubscriber {
    node: rclrs::Node,
    _subscription: Arc<rclrs::Subscription<Image>>,
    gui: bool
}

impl CameraSubscriber {
    fn new(context: &rclrs::Context) -> Result<Self, Error> {
        let mut node = rclrs::Node::new(context, "camera_subscriber")?;
        let _subscription = { // Create a new shared pointer instance that will be owned by the closure
            node.create_subscription(
                "topic",
                rclrs::QOS_PROFILE_DEFAULT,
                move |msg: Image| {
                    printl!("Recieving new image!");
                    if self.gui {
                        let frame = CvImage::from_imgmsg(msg).as_cvmat("bgr8".to_string());
                        if frame.size().width > 0 {
                            highgui::imshow(window, &frame);
                        }
                        let key = highgui::wait_key(10)?;
                    }
                }
            )
        },
        let gui = false;
        Ok(Self{node, _subscription, gui})
    };
};

fn main() -> Result<(), Error> {
    let context = rclrs::Context::new(env::args())?;
    let camera_subscriber = CameraSubscriber::new(context);
	highgui::named_window("video capture", highgui::WINDOW_AUTOSIZE)?;
    rclrs::spin(&camera_subscriber.node)?;
    Ok(())
}