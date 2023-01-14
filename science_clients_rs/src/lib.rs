use std::{env, sync::{Arc, Mutex}, str::ParseBoolError, num::ParseIntError};
use anyhow::{Result, Error};
use input_macro::input;
use colored::*;
use opencv::{highgui, prelude::*};
use cv_bridge_rs::CvImage;
use std_srvs::srv::SetBool;
use science_interfaces_rs::srv::Position;
use sensor_msgs::msg::Image;

pub struct OnOffClient {
    _subsystem: String,
    _device: String,
    _node: Arc<Mutex<rclrs::Node>,
    _client: Arc<rclrs::Client<SetBool>>,
}

impl OnOffClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let mut _node = Arc::new(Mutex::new(rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?));
        let _client = _node.unwrap().create_client::<SetBool>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        let _subsystem = subsystem;
        let _device = device.replace("_", " ").to_string();
        Ok(Self{_subsystem:_subsystem, _device:_device, _node:_node, _client:_client})
    }

    async fn send_request(&self, state: &bool) -> Result<(), Error> {
        let request = std_srvs::srv::SetBool_Request{data: *state};
        let future = self._client.call_async(&request);
        println!("Request sent to {}", &self._device);
        let response = future.await?;
        println!("{}", response.message);
        Ok(())
    }

    fn cli_control(&self) -> Result<(), Error> {
        self.run();
        let mut proceed: Result<bool, ParseBoolError> = Ok(true);
        let mut state : Result<bool, ParseBoolError>;
        while proceed? {
            state = input!("Enter a command for the {} (on => {} | off => {}): ", &self._device, "true".bold().yellow(), "false".bold().yellow()).trim().to_lowercase().parse::<bool>();
            loop {
                match state.as_ref().unwrap() {
                    bool => { break; }
                    ParseBoolError => { state = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<bool>(); }
                }
            }
            self.send_request(state.as_ref().unwrap());
            proceed = input!("If you would like to continue inputing commands, type {}, otherwise type {}.", "true".bold().blue(), "false".bold().red()).trim().to_lowercase().parse::<bool>();
            loop {
                match proceed.as_ref().unwrap() {
                    bool => { break; }
                    ParseBoolError => { proceed = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<bool>(); }
                }
            }
        }
        Ok(())
    }
    
    fn run(&self) {
        let node_clone = Arc::clone(&self._node);
        std::thread::spawn(|| {
            let node = node_clone.lock().unwrap()
            rclrs::spin(&node);
        });
    }
}

pub struct CameraClient {
    _subsystem: String,
    _device: String,
    _node: rclrs::Node,
    _client: Arc<rclrs::Client<SetBool>>,
    _subscription: Arc<rclrs::Subscription<Image>>,
    _frame: Arc<Mutex<Option<Mat>>>,
}

impl CameraClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let mut _node = Arc::new(Mutex::new(rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?));
        let _frame = Arc::new(Mutex::new(None));
        let frame_clone = Arc::clone(&_frame);
        let _client = _node.unwrap().create_client::<SetBool>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        let _subscription = {
            _node.unwrap().create_subscription(format!("/{}/{}/images", &subsystem, &device).as_str(), rclrs::QOS_PROFILE_DEFAULT,
                move |msg: Image| {
                    println!("Recieving new {} image!", device.replace("_", " "));
                    *frame_clone.lock().unwrap() = Some(CvImage::from_imgmsg(msg).as_cvmat("bgr8".to_string()));
                    if frame_clone.lock().unwrap().as_ref().unwrap().size().unwrap().width > 0 {
                        highgui::imshow(&device.as_str(), &frame_clone.lock().unwrap().as_ref().unwrap());
                    }
                    let _key = highgui::wait_key(10);
                },
            )?
        };
        let _subsystem = subsystem;
        let _device = device.replace("_", " ").to_string();
        Ok(Self{_subsystem:_subsystem, _device:_device, _node:_node, _client:_client, _subscription:_subscription, _frame:_frame})
    }

    async fn send_request(&self, state: &bool) -> Result<(), Error> {
        let request = std_srvs::srv::SetBool_Request{data: *state};
        let future = self._client.call_async(&request);
        println!("Request sent to {}", &self._device);
        let response = future.await?;
        println!("{}", response.message);
        Ok(())
    }

    fn cli_control(&self) -> Result<(), Error> {
        self.run();
        let mut proceed: Result<bool, ParseBoolError> = Ok(true);
        let mut state : Result<bool, ParseBoolError>;
        while proceed? {
            state = input!("Enter a command for the {} ({} | {}): ", &self._device, "on => true".bold().yellow(), "off => false".bold().yellow()).trim().to_lowercase().parse::<bool>();
            loop {
                match state.as_ref().unwrap() {
                    bool => { break; }
                    ParseBoolError => { state = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<bool>(); }
                }
            }
            self.send_request(state.as_ref().unwrap());
            proceed = input!("If you would like to continue inputing commands, type {}, otherwise type {}.", "true".bold().yellow(), "false".bold().yellow()).trim().to_lowercase().parse::<bool>();
            loop {
                match proceed.as_ref().unwrap() {
                    bool => { break; }
                    ParseBoolError => { proceed = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<bool>(); }
                }
            }
        }
        Ok(())
    }

    fn run(&self) {
        let node_clone = Arc::clone(&self._node);
        std::thread::spawn(|| {
            let node = node_clone.lock().unwrap()
            rclrs::spin(&node);
        });
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
        let mut _node = Arc::new(Mutex::new(rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?));
        let _client = _node.unwrap().create_client::<Position>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        let _subsystem = subsystem;
        let _device = device.replace("_", " ").to_string();
        Ok(Self{_subsystem:_subsystem, _device:_device, _node:_node, _client:_client})
    }

    async fn send_request(&self, position: &i32) -> Result<(), Error> {
        let request = science_interfaces_rs::srv::Position_Request{position: *position};
        let future = self._client.call_async(&request);
        println!("Request sent to {}.", &self._device);
        let response = future.await?;
        match response.success {
            true => { println!("Request completed. The {} is now at position {}.", &self._device, &response.position); },
            false => {
                println!("Request failed! The {} stopped at {}.", &self._device, &response.position);
                println!("{}", &response.message);
            }
        }
        Ok(())
    }

    fn cli_control(&self) -> Result<(), Error> {
        self.run();
        let mut proceed: Result<bool, ParseBoolError> = Ok(true);
        let mut position: Result<i32, ParseIntError>;
        while proceed? {
            position = input!("Enter an integer position value ({} | {}): ", "minimum => 0".bold().yellow(), "maximum => 2147483647".bold().yellow()).trim().to_lowercase().parse::<i32>();
            loop {
                match position.as_ref().unwrap() {
                    d if d < 0 => {
                        position = Ok(0); 
                        break;
                    }
                    d if d >= 0 => {
                        break;
                    }
                    ParseIntError => { position = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<i32>(); }
                }
            }
            self.send_request(position.as_ref().unwrap());
            proceed = input!("If you would like to continue inputing commands, type {}, otherwise type {}.", "true".bold().yellow(), "false".bold().yellow()).trim().to_lowercase().parse::<bool>();
            loop {
                match proceed.as_ref().unwrap() {
                    bool => { break; }
                    ParseBoolError => { proceed = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<bool>(); }
                }
            }
        }
        Ok(())
    }

    fn run(&self) {
        let node_clone = Arc::clone(&self._node);
        std::thread::spawn(|| {
            let node = node_clone.lock().unwrap()
            rclrs::spin(&node);
        });
    }
}