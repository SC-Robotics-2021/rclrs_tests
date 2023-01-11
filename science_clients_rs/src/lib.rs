use std::env;
use std::sync::{Arc, Mutex};
use anyhow::{Error, Result};
use inflector::Inflector;
use input_macro::input;
use colored::Colorize;
use opencv::{highgui, prelude::*};
use cv_bridge::CvImage;
use std_srvs::srv::SetBool;
use science_interfaces_rs::srv::Position;
use sensor_msgs::msg::Image;
use no_panic::no_panic;

pub struct OnOffClient {
    _subsystem: String,
    _device: String,
    _node: rclrs::Node;
    _client: Arc<rclrs::Client<SetBool>>,
}

impl OnOffClient {
    #[no_panic]
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let mut _node = rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{&device}_client").as_str())?;
        let _client = _node.create_client::<SetBool>(format!("/{&subsystem}/{$device}/cmd").as_str())?;
        let _subsystem = subsystem;
        let _device = str.replace($device, "_", " ");
        Ok(Self{_subsystem:_subsystem, _device:_device, _node:_node, _client:_client})
    }

    #[no_panic]
    fn send_request(&self, state: &bool) {
        while !self._client.wait_for_service(timeout_sec=1.0) {
            println!(format!("The {} is not available. Waiting a second...", &self._device))
        }
        let request = std_srvs::srv::SetBool_Request{data: state};
        let future = self._client.call_async(&request);
        println!(format!("Request sent to {}", &self._device))
        while rclrs.ok() {
            if future.done() {
                println!(future.result().message);
                break;
            }
        }
    }

    #[no_panic]
    fn cli_control(&self) -> Result<(), Error> {
        std::thread::spawn(move || -> Result<(), Error> {
            Ok(rclrs::spin(&self._node)?)
        });
        let mut proceed: Result<bool, Error> = Ok(true);
        let mut state : Result<bool, Error>;
        while Ok(proceed) {
            state = input!(format!("Enter a command for the {} ({} | {}): ", &self._device, "on => true".bold().blue(), "off => false".bold().red())).trim().to_lowercase().parse();
            loop {
                match state {
                    Ok(bool) => { break; }
                    Error => { state = input!(format!("{}", "Invalid input. Try again: ".yellow())).trim().to_lowercase().parse()?; }
                }
            }
            self.send_request(&state?);
            proceed = input!(format!("If you would like to continue inputing commands, type {'true'.bold().blue()}, otherwise type {'false'.bold().red()}.")).trim().to_lowercase().parse();
            loop {
                match proceed {
                    Ok(bool) => { break; }
                    Error => { proceed = input!(format!("{}", "Invalid input. Try again: ".yellow())).trim().to_lowercase().parse(); }
                }
            }
        }
        Ok(())
    }
}

pub struct CameraClient {
    _subsystem: String;
    _device: String;
    _node: rclrs::Node;
    _client: Arc<rclrs::Client<SetBool>>;
    _subscription: Arc<rclrs::Subscription<Image>>;
    _frame: Arc<Mutex<Option<CvImage>>>;
}

impl CameraClient {
    #[no_panic]
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let mut _node = rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?;
        let _frame = Arc::new(Mutex::new(None));
        let frame_clone = Arc::clone(&_frame);
        let _client = _node.create_client::<SetBool>(format!("/{&subsystem}/{&device}/cmd").as_str())?;
        let _subscription = {
            _node.create_subscription(format!("/{&subsystem}/{$device}/images"), rclrs::QOS_PROFILE_DEFAULT,
                move |msg: Image| {
                    println!(format!("Recieving new {} image!", str.replace($device, '_', ' ')));
                   *frame_clone.lock()? = Some(CvImage::from_imgmsg(msg).as_cvmat("bgr8".to_string()));
                    if *frame_clone.lock()?.size()?.width > 0 {
                        highgui::imshow(format!("{}", &device), &*frame.lock()?);
                    }
                    let _key = highgui::wait_key(10);
                }
            )?
        };
        let _subsystem = subsystem;
        let _device = str.replace($device, "_", " ");
        Ok(Self{_subsystem:_subsystem, _device:_device, _node:_node, _client:_client, _subscription:_subscription, _frame:_frame})
    }

    #[no_panic]
    fn send_request(&self, state: &bool) {
        while !self._client.wait_for_service(timeout_sec=1.0) {
            println!(format!("The {} is not available. Waiting a second...", &self._device))
        }
        let request = std_srvs::srv::SetBool_Request{data: state};
        let future = self._client.call_async(&request);
        println!(format!("Request sent to {}", &self._device))
        while rclrs.ok() {
            if future.done() {
                println!(future.result().message);
                break;
            }
        }
    }

    #[no_panic]
    fn cli_control(&self) -> Result<(), Error> {
        std::thread::spawn(move || -> Result<(), Error> {
            Ok(rclrs::spin(&self._node)?)
        });
        let mut proceed: Result<bool, Error> = Ok(true);
        let mut state : Result<bool, Error>;
        while proceed {
            state = input!(format!("Enter a command for the {} ({} | {}): ", &self._device, "on => true".bold().blue(), "off => false".bold().red())).trim().to_lowercase().parse();
            loop {
                match state {
                    Ok(bool) => { break; }
                    Error => { input!(format!("{}", "Invalid input. Try again: ".yellow())).trim().to_lowercase().parse()?; }
                }
                self.send_request(&state?)
            }
            proceed = input!(format!("If you would like to continue inputing commands, type {'true'.bold().blue()}, otherwise type {'false'.bold().red()}.")).trim().to_lowercase().parse();
            loop {
                match proceed {
                    Ok(bool) => { break; }
                    Error => { proceed = input!(format!("{}", "Invalid input. Try again: ".yellow())).trim().to_lowercase().parse(); }
                }
            }
        }
        Ok(())
    }
}

pub struct PositionClient {
    _subsystem: String;
    _device: String;
    _node: rclrs::Node;
    _client: Arc<rclrs::Client<Position>>;
}

impl PositionClient {
    #[no_panic]
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let mut _node = rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str().as_str())?;
        let _client = _node.create_client::<Position>(format!("/{}/{}/cmd", &subsystem, $device).as_str())?;
        let _subsystem = subsystem;
        let _device = str.replace($device, "_", " ");
        Ok(Self{_subsystem:_subsystem, _device:_device, _node:_node, _client:_client})
    }

    #[no_panic]
    fn send_request(&self, position: &i32) {
        while !self._client.wait_for_service(timeout_sec=1.0) {
            println!(format!("{} not available. Waiting...", &self._device));
        }
        let request = science_interfaces_rs::srv::Position_Request{position: position};
        let future = self._client.call_async(&request);
        println!(format!("Request sent to {}.", &self._device));
        while rclrs.ok() {
            if future.done() {
                let response = future.result();
                match response.success {
                    true => { println!(format!("Request completed. The {} is now at position {}.", &self._device, &response.position)); },
                    false => {
                        println!(format!("Request failed! The {} stopped at {}.", &self._device, &response.position));
                        println!(&response.error);
                    }
                }
                break;
            }
        }
    }

    #[no_panic]
    fn cli_control(&self) {
        std::thread::spawn(move || -> Result<(), Error> {
            Ok(rclrs::spin(&self._node)?)
        });
        let mut proceed: Result<bool, Error> = Ok(true);
        let mut position: Result<i32, Error>;
        while proceed {
            position = input!(format!("Enter an integer position value ({} | {}): ", "minimum => 0".bold().blue(), "maximum => 2147483647".bold().red())).trim().to_lowercase().parse();
            loop {
                match position {
                    Ok(i32) => {
                        if position < 0 { position = 0; }
                        break;
                    }
                    Error => { position = input!(format!("{}", "Invalid input. Try again: ".yellow())).trim().to_lowercase().parse()?; }
                }
            }
            self.send_request(&position?);
            proceed = input!(format!("If you would like to continue inputing commands, type true, otherwise type false.")).trim().to_lowercase().parse();
            loop {
                match proceed {
                    Ok(bool) => { break; }
                    Error => { proceed = input!(format!("{}", "Invalid input. Try again: ".yellow())).trim().to_lowercase().parse(); }
                }
            }
        }
        Ok(())
    }
}