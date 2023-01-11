use std::{env, sync::{Arc, Mutex}, str::{ParseBoolError, ParseIntError}};
use anyhow::{Result, Error};
use inflector::Inflector;
use input_macro::input;
use colored::*;
use opencv::{highgui, prelude::*};
use cv_bridge::CvImage;
use std_srvs::srv::SetBool;
use science_interfaces_rs::srv::Position;
use sensor_msgs::msg::Image;

pub struct OnOffClient {
    _subsystem: String,
    _device: String,
    _node: rclrs::Node,
    _client: Arc<rclrs::Client<SetBool>>,
}

impl OnOffClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let mut _node = rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?;
        let _client = _node.create_client::<SetBool>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        let _subsystem = subsystem;
        let _device = &device.replace("_", " ");
        Ok(Self{_subsystem:_subsystem, _device:_device, _node:_node, _client:_client})
    }

    fn send_request(&self, state: &bool) {
        while !self._client.wait_for_service(timeout_sec=1.0) {
            println!("The {} is not available. Waiting a second...", &self._device);
        }
        let request = <std_srvs::srv::SetBool as Trait>::Request{data: *state};
        let future = self._client.call_async(&request);
        println!("Request sent to {}", &self._device);
        future.await;
        println!(future.result().message);
    }

    fn cli_control(&self) {
        std::thread::spawn(move || -> Result<(), Error> {
            Ok(rclrs::spin(&self._node)?)
        });
        let mut proceed: Result<bool, ParseBoolError> = Ok(true);
        let mut state : Result<bool, ParseBoolError>;
        while proceed == Ok(true) {
            state = input!(format!("Enter a command for the {} ({} | {}): ", &self._device, "on => true".bold().blue(), "off => false".bold().red()).as_str()).trim().to_lowercase().parse();
            loop {
                match state {
                    Ok(bool) => { break; }
                    ParseBoolError => { state = input!(format!("{}", "Invalid input. Try again: ".yellow()).as_str()).trim().to_lowercase().parse(); }
                }
            }
            self.send_request(&state);
            proceed = input!(format!("If you would like to continue inputing commands, type {'true'.bold().blue()}, otherwise type {'false'.bold().red()}.")).trim().to_lowercase().parse();
            loop {
                match proceed {
                    Ok(bool) => { break; }
                    ParseBoolError => { proceed = input!(format!("{}", "Invalid input. Try again: ".yellow()).as_str()).trim().to_lowercase().parse(); }
                }
            }
        }
    }
}

pub struct CameraClient {
    _subsystem: String,
    _device: String,
    _node: rclrs::Node,
    _client: Arc<rclrs::Client<SetBool>>,
    _subscription: Arc<rclrs::Subscription<Image>>,
    _frame: Arc<Mutex<Option<CvImage>>>,
}

impl CameraClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let mut _node = rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?;
        let _frame = Arc::new(Mutex::new(None));
        let frame_clone = Arc::clone(&_frame);
        let _client = _node.create_client::<SetBool>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        let _subscription = {
            _node.create_subscription(format!("/{}/{}/images", &subsystem, &device).as_str(), rclrs::QOS_PROFILE_DEFAULT,
                move |msg: Image| -> Result<(), Error> {
                    println!("Recieving new {} image!", &device.replace("_", " "));
                   let frame = Some(CvImage::from_imgmsg(msg).as_cvmat("bgr8".to_string()))
                   *frame_clone.lock().unwrap() = frame;
                    if frame.size()?.width > 0 {
                        highgui::imshow(&device.as_str(), &frame_clone.lock().unwrap());
                    }
                    let _key = highgui::wait_key(10);
                }
            )?
        };
        let _subsystem = subsystem;
        let _device = device.replace("_", " ");
        Ok(Self{_subsystem:_subsystem, _device:_device, _node:_node, _client:_client, _subscription:_subscription, _frame:_frame})
    }

    fn send_request(&self, state: &bool) {
        while !self._client.wait_for_service(timeout_sec=1.0) {
            println!("The {} is not available. Waiting a second...", &self._device);
        }
        let request = <std_srvs::srv::SetBool as Trait>::Request{data: *state};
        let future = self._client.call_async(&request);
        println!("Request sent to {}", &self._device);
        future.await;
        println!("{}", future.result().message);
    }

    fn cli_control(&self) {
        std::thread::spawn(move || -> Result<(), Error> {
            Ok(rclrs::spin(&self._node)?)
        });
        let mut proceed: Result<bool, ParseBoolError> = Ok(true);
        let mut state : Result<bool, ParseBoolError>;
        while proceed? {
            state = input!(format!("Enter a command for the {} ({} | {}): ", &self._device, "on => true".bold().blue(), "off => false".bold().red())).trim().to_lowercase().parse();
            loop {
                match state {
                    Ok(bool) => { break; }
                    ParseBoolError => { input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse(); }
                }
                self.send_request(&state)
            }
            proceed = input!("If you would like to continue inputing commands, type {}, otherwise type {}.", "true".bold().blue(), "false".bold().red()).trim().to_lowercase().parse();
            loop {
                match proceed {
                    Ok(bool) => { break; }
                    ParseBoolError => { proceed = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse(); }
                }
            }
        }
    }
}

pub struct PositionClient {
    _subsystem: String,
    _device: String,
    _node: rclrs::Node,
    _client: Arc<rclrs::Client<Position>>,
}

impl PositionClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let mut _node = rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?;
        let _client = _node.create_client::<Position>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        let _subsystem = subsystem;
        let _device = &device.replace("_", " ");
        Ok(Self{_subsystem:_subsystem, _device:_device, _node:_node, _client:_client})
    }

    fn send_request(&self, position: &i32) {
        while !self._client.wait_for_service(timeout_sec=1.0) {
            println!("{} not available. Waiting...", &self._device);
        }
        let request = <science_interfaces_rs::srv::Position as Trait>::Request{position: *position};
        let future = self._client.call_async(&request);
        println!("Request sent to {}.", &self._device);
        future.await;
        let response = future::result();
        match response.success {
            true => { println!("Request completed. The {} is now at position {}.", &self._device, &response.position); },
            false => {
                println!("Request failed! The {} stopped at {}.", &self._device, &response.position);
                println!("{}", &response.error);
            }
        }
    }

    fn cli_control(&self) {
        std::thread::spawn(move || -> Result<(), Error> {
            Ok(rclrs::spin(&self._node)?)
        });
        let mut proceed: Result<bool, ParseIntError> = Ok(true);
        let mut position: Result<i32, ParseBoolError>;
        while proceed? {
            position = input!("Enter an integer position value ({} | {}): ", "minimum => 0".bold().blue(), "maximum => 2147483647".bold().red()).trim().to_lowercase().parse();
            loop {
                match position {
                    Ok(i32) => {
                        if (position? < 0) { position = Ok(0); }
                        break;
                    }
                    ParseIntError => { position = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse(); }
                }
            }
            self.send_request(&position);
            proceed = input!("If you would like to continue inputing commands, type true, otherwise type false.").trim().to_lowercase().parse();
            loop {
                match proceed {
                    Ok(bool) => { break; }
                    ParseBoolError => { proceed = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse(); }
                }
            }
        }
    }
}