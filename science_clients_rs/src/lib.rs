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
    _node: rclrs::Node,
    _client: Arc<rclrs::Client<SetBool>>,
}

impl OnOffClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let mut _node = rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?;
        let _client = _node.create_client::<SetBool>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        let _subsystem = subsystem;
        let _device = device.replace("_", " ").to_string();
        Ok(Self{_subsystem:_subsystem, _device:_device, _node:_node, _client:_client})
    }

    async fn send_request(&self, state: &bool) -> Result<(), Error> {
        while !self._client.unwrap()wait_for_service(1) {
            println!("The {} is not available. Waiting a second...", &self._device);
        }
        let future = self._client.call_async(&std_srvs::srv::SetBool_Request{data: *state});
        println!("Request sent to {}", &self._device);
        let response = future.await?;
        println!("{}", response.message);
        Ok(())
    }

    fn cli_control(&self) -> Result<(), Error> {
        std::thread::spawn(move || -> Result<(), Error> {
            Ok(rclrs::spin(&self._node)?)
        });
        let mut proceed: Result<bool, ParseBoolError> = Ok(true);
        let mut state : Result<bool, ParseBoolError>;
        while proceed? {
            state = input!("Enter a command for the {} (on => {} | off => {}): ", &self._device, "true".bold().yellow(), "false".bold().yellow()).trim().to_lowercase().parse::<bool>();
            loop {
                match state {
                    Ok(bool) => { break; }
                    ParseBoolError => { state = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<bool>(); }
                }
            }
            self.send_request(&state?);
            proceed = input!("If you would like to continue inputing commands, type {}, otherwise type {}.", "true".bold().blue(), "false".bold().red()).trim().to_lowercase().parse::<bool>();
            loop {
                match proceed {
                    Ok(bool) => { break; }
                    ParseBoolError => { proceed = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<bool>(); }
                }
            }
        }
        Ok(())
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
        let mut _node = rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?;
        let _frame = Arc::new(Mutex::new(None));
        let frame_clone = Arc::clone(&_frame);
        let _client = _node.create_client::<SetBool>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        let _subscription = {
            _node.create_subscription(format!("/{}/{}/images", &subsystem, &device).as_str(), rclrs::QOS_PROFILE_DEFAULT,
                move |msg: Image| {
                    println!("Recieving new {} image!", &device.replace("_", " "));
                    *frame_clone.lock().unwrap() = Some(CvImage::from_imgmsg(msg).as_cvmat("bgr8".to_string()));
                    if frame_clone.lock().unwrap().unwrap().size().unwrap().width > 0 {
                        highgui::imshow(&device.as_str(), &frame_clone.lock().unwrap().unwrap());
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
        while !self._client.unwrap()wait_for_service(1) {
            println!("The {} is not available right now. Waiting a second...", &self._device);
        }
        let future = self._client.call_async(&std_srvs::srv::SetBool_Request{data: *state});
        println!("Request sent to {}", &self._device);
        let response = future.await?;
        println!("{}", response.message);
        Ok(())
    }

    fn cli_control(&self) -> Result<(), Error> {
        std::thread::spawn(move || -> Result<(), Error> {
            Ok(rclrs::spin(&self._node)?)
        });
        let mut proceed: Result<bool, ParseBoolError> = Ok(true);
        let mut state : Result<bool, ParseBoolError>;
        while proceed? {
            state = input!("Enter a command for the {} ({} | {}): ", &self._device, "on => true".bold().yellow(), "off => false".bold().yellow()).trim().to_lowercase().parse::<bool>();
            loop {
                match state {
                    Ok(bool) => { break; }
                    ParseBoolError => { input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<bool>(); }
                }
            }
            self.send_request(&state?);
            proceed = input!("If you would like to continue inputing commands, type {}, otherwise type {}.", "true".bold().yellow(), "false".bold().yellow()).trim().to_lowercase().parse::<bool>();
            loop {
                match proceed {
                    Ok(bool) => { break; }
                    ParseBoolError => { proceed = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<bool>(); }
                }
            }
        }
        Ok(())
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
        let _device = device.replace("_", " ").to_string();
        Ok(Self{_subsystem:_subsystem, _device:_device, _node:_node, _client:_client})
    }

    async fn send_request(&self, position: &i32) -> Result<(), Error> {
        while !self._client.unwrap()wait_for_service(1) {
            println!("The {} is not available right now. Waiting a second...", &self._device);
        }
        let future = self._client.call_async(&science_interfaces_rs::srv::Position_Request{position: *position});
        println!("Request sent to {}.", &self._device);
        let response = future.await?;
        match response.success {
            true => { println!("Request completed. The {} is now at position {}.", &self._device, &response.position); },
            false => {
                println!("Request failed! The {} stopped at {}.", &self._device, &response.position);
                println!("{}", &response.error);
            }
        }
        Ok(())
    }

    fn cli_control(&self) -> Result<(), Error> {
        std::thread::spawn(move || -> Result<(), Error> {
            Ok(rclrs::spin(&self._node)?)
        });
        let mut proceed: Result<bool, ParseBoolError> = Ok(true);
        let mut position: Result<i32, ParseIntError>;
        while proceed? {
            position = input!("Enter an integer position value ({} | {}): ", "minimum => 0".bold().yellow(), "maximum => 2147483647".bold().yellow()).trim().to_lowercase().parse::<i32>();
            loop {
                match position {
                    Ok(i32) => {
                        if position? < 0 { position = Ok(0); }
                        break;
                    }
                    ParseIntError => { position = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<i32>(); }
                }
            }
            self.send_request(&position?);
            proceed = input!("If you would like to continue inputing commands, type {}, otherwise type {}.", "true".bold().yellow(), "false".bold().yellow()).trim().to_lowercase().parse::<bool>();
            loop {
                match proceed {
                    Ok(bool) => { break; }
                    ParseBoolError => { proceed = input!("{}", "Invalid input. Try again: ".yellow()).trim().to_lowercase().parse::<bool>(); }
                }
            }
        }
        Ok(())
    }
}