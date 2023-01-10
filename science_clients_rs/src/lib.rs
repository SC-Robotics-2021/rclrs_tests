use std::env;
use std::sync::{Arc, Mutex};
use anyhow::{Error, Result};
use inflector::Inflector;
use input_macro::input;
use colored::Colorize;
use opencv::{highgui, prelude::*};
use cv_bridge::CvImage;
use std_srvs::srv::SetBool;
use sensor_msgs::msg::Image;
use science_interfaces::srv::Position;

pub struct PositionClient {}
pub struct OnOffClient {}
pub struct CameraClient {
    _subscription: Arc<rclrs::Subscription<Image>>;
    frame: Arc<Mutex<Option<CvImage>>>;
}

trait ClientNode {
    node: rclrs::Node;
    _subsystem: String;
    _device: String;
    _client: Arc<rclrs::Client<_>>;
}

trait ClientExecution {
    fn send_request(&self);
    fn cli_control{&self};
}

impl ClientNode for OnOffClient {
    #[no_panic]
    fn new(subsystem, device) -> Result<Self, Error> {
        let mut node = rclrs::Node::new(rclrs::Context::new(env::args())?, format!("{&device}_client"))?;
        let _client = node.create_client::<SetBool>(format!("/{&subsystem}/{$device}/cmd"))?;
        let _subsystem = subsystem;
        let _device = str.replace($device, '_', ' ');
        Ok(Self{node, _subsystem, _device, _client})
    }
}

impl ClientNode for OnOffClient {
    #[no_panic]
    fn new(subsystem, device) -> Result<Self, Error> {
        let mut node = rclrs::Node::new(rclrs::Context::new(env::args())?, format!("{&device}_client"))?;
        let frame = Arc::new(Mutex::new(None));
        let frame_cb = Arc::clone(&frame);
        let _client = node.create_client::<SetBool>(format!("/{&subsystem}/{&device}/cmd"))?;
        let _subscription = {
            node.create_subscription(format!("/{&subsystem}/{$device}/images"), rclrs::QOS_PROFILE_DEFAULT,
                move |msg: Image| {
                    println!(format!("Recieving new {str.replace($device, '_', ' ')} image!"));
                   *frame.lock.unwrap() = Some(CvImage::from_imgmsg(msg).as_cvmat("bgr8".to_string()));
                    if *frame.lock.unwrap().size().unwrap().width > 0 {
                        highgui::imshow(format!("{&device}"), &*frame.lock.unwrap());
                    }
                    let _key = highgui::wait_key(10);
                }
            )?
        };
        let _subsystem = subsystem;
        let _device = str.replace($device, '_', ' ');
        Ok(Self{node, _subsystem, _device, _client, _subscription})
    }
}

impl ClientExecution for PositionClient {
    #[no_panic]
    fn send_request(&self, position) {
        while not node._client.wait_for_service(timeout_sec=1.0) {
            println!(format!("{&self._device} not available. Waiting..."))
        }
        let request = SetBool{data: position};
        let future = node._client.call_async(&request);
        println!("Request sent to {&self._device}.")
        while rclrs.ok() {
            if future.done() {
                let response = future.result()
                match response {
                    Ok(()) => { println!(format!("{&self._device} is now at position {&request}.")); },
                    Error => { println!(format!("Request failed: {&response.message}")); }
                }
                break;
            }
        }
    }

    #[no_panic]
    fn cli_control(&self) {
        std::thread::spawn(move || -> Result<(), Error> {
            rclrs::spin(&self.node)?;
        });
        let mut proceed: bool = true;
        while proceed {
            let mut position: u8 = input!(format!("Enter an integer position value ({'minimum => 0'.bold().blue()} | {'maximum => 255'.bold().red()}): ")).trim().to_lowercase().parse().unwrap();
            loop {
                match position {
                    Ok(()) => { break; }
                    Error => { input!(format!("Invalid input. Try again: ")).trim().to_lowercase().parse().unwrap(); }
                }
            }
            let request = {}
            self.send_request(&position)

            let proceed: bool = input!(format!("If you would like to continue inputing commands, type true, otherwise type false.")).trim().to_lowercase().parse().unwrap();
            loop {
                match proceed {
                    Ok(()) => { break; }
                    Error => { proceed = input!(format!("Invalid input. Try again: ")).trim().to_lowercase().parse().unwrap(); }
                }
            }
        }
    }
}

impl ClientExecution for OnOffClient, CameraClient {
    #[no_panic]
    fn send_request(&self, command) {
        while not node._client.wait_for_service(timeout_sec=1.0) {
            println!(format!("{&node._device.to_sentence_case()} not available. Waiting..."))
        }
        let request = SetBool{data: command};
        let future = node._client.call_async(&request);
        println!("Request sent to {&self._device}")
        while rclrs.ok() {
            if future.done() {
                match future.result() {
                    Ok(()) => { println!(format!("{&node._device.to_sentence_case()} is now {
                        match request.data {
                            true => { 'on' }
                            false => { 'off' }
                        }
                    }.")); },
                    Error => { println!(format!("Request failed! {&node._device.to_sentence_case()} already in requested state.")); }
                }
                break;
            }
        }
    }

    #[no_panic]
    fn cli_control(&self) -> Result<(), Error> {
        std::thread::spawn(move || -> Result<(), Error> {
            rclrs::spin(&self.node)?;
        });
        let mut proceed: bool = true;
        while proceed {
            let mut state : bool = input!(format!("Enter a command for the {&self._device} ({'on => true'.bold().blue()} | {'off => false'.bold().red()}): ")).trim().to_lowercase().parse().unwrap();
            loop {
                match state {
                    Ok(bool) => { break; }
                    Error => { input!(format!("Invalid input. Try again: ")).trim().to_lowercase().parse().unwrap(); }
                }
                self.send_request(&state)
            }
            let proceed: bool = input!(format!("If you would like to continue inputing commands, type {'true'.bold().blue()}, otherwise type {'false'.bold().red()}.")).trim().to_lowercase().parse().unwrap();
            loop {
                match proceed {
                    Ok(()) => { break; }
                    Error => { proceed = input!(format!("Invalid input. Try again: ")).trim().to_lowercase().parse().unwrap(); }
                }
            }
        }
    }
}