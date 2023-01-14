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
    _node: Arc<Mutex<rclrs::Node>>,
    _client: Arc<rclrs::Client<SetBool>>
}

impl OnOffClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let _node = Arc::new(Mutex::new(rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?));
        let node_clone = Arc::clone(&_node);
        let mut node = node_clone.lock().unwrap();
        let _client = node.create_client::<SetBool>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        Ok(Self{_node:_node, _client:_client})
    }

    async fn send_request(&self, state: &bool) -> Result<(), Error> {
        let request = std_srvs::srv::SetBool_Request{data: *state};
        let future = self._client.call_async(&request);
        println!("Request sent.");
        let response = future.await?;
        println!("{}", response.message);
        Ok(())
    }

    fn run(&self) {
        let node_clone = Arc::clone(&self._node);
        let node_thread std::thread::spawn(move || {
            let node = node_clone.lock().unwrap();
            rclrs::spin(&node);
        });
        node_thread.join().expect("Unable to join node thread");
    }

    fn cli_control(&self) -> Result<(), Error> {
        self.run();
        let mut proceed: bool + 'static = true;
        let mut state: bool + 'static;
        while proceed {
            loop {
                match input!("Enter a command (on => {} | off => {}): ", "true".bold().yellow(), "false".bold().yellow()).trim().to_lowercase().parse::<bool>()? {
                    Ok(a) => {
                        state = a;
                        break;
                    }
                    ParseIntError => { continue; }
                }
            }
            self.send_request(&state);
            loop {
                match input!("Enter {} to continue inputing commands, otherwise enter {}.", "true".bold().yellow(), "false".bold().yellow()).trim().to_lowercase().parse::<bool>()? {
                    Ok(b) => {
                        proceed = b;
                        break;
                    }
                    ParseBoolError => { continue; }
                }
            }
        }
        Ok(())
    }
}

pub struct CameraClient {
    _node: Arc<Mutex<rclrs::Node>>,
    _client: Arc<rclrs::Client<SetBool>>,
    _subscription: Arc<rclrs::Subscription<Image>>,
    _frame: Arc<Mutex<Option<Mat>>>
}

impl CameraClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let _node = Arc::new(Mutex::new(rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?));
        let node_clone = Arc::clone(&_node);
        let mut node = node_clone.lock().unwrap();
        let _client = node.create_client::<SetBool>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        let _frame = Arc::new(Mutex::new(None));
        let frame_clone = Arc::clone(&_frame);
        let _subscription = {
            node.create_subscription(format!("/{}/{}/images", &subsystem, &device).as_str(), rclrs::QOS_PROFILE_DEFAULT,
                move |msg: Image| {
                    println!("Recieving new image!");
                    *frame_clone.lock().unwrap() = Some(CvImage::from_imgmsg(msg).as_cvmat("bgr8".to_string()));
                    if frame_clone.lock().unwrap().as_ref().unwrap().size().unwrap().width > 0 {
                        highgui::imshow(&device.replace("_", " "), &frame_clone.lock().unwrap().as_ref().unwrap());
                    }
                    let _key = highgui::wait_key(10);
                },
            )?
        };
        Ok(Self{_node:_node, _client:_client, _subscription:_subscription, _frame:_frame})
    }

    async fn send_request(&self, state: &bool) -> Result<(), Error> {
        let request = std_srvs::srv::SetBool_Request{data: *state};
        let future = self._client.call_async(&request);
        println!("Request sent.");
        let response = future.await?;
        println!("{}", response.message);
        Ok(())
    }

    fn run(&self) {
        let node_clone = Arc::clone(&self._node);
        let node_thread = std::thread::spawn(move || {
            let node = node_clone.lock().unwrap();
            rclrs::spin(&node);
        });
        node_thread.join().expect("Unable to join node thread");
    }

    fn cli_control(&self) -> Result<(), Error> {
        self.run();
        let mut proceed: bool + 'static = true;
        let mut state: bool + 'static;
        while proceed {
            loop {
                match input!("Enter a command (on => {} | off => {}): ", "true".bold().yellow(), "false".bold().yellow()).trim().to_lowercase().parse::<bool>()? {
                    Ok(a) => {
                        state = a;
                        break;
                    }
                    ParseIntError => { continue; }
                }
            }
            self.send_request(&state);
            loop {
                match input!("Enter {} to continue inputing commands, otherwise enter {}.", "true".bold().yellow(), "false".bold().yellow()).trim().to_lowercase().parse::<bool>()? {
                    Ok(b) => {
                        proceed = b;
                        break;
                    }
                    ParseBoolError => { continue; }
                }
            }
        }
        Ok(())
    }
}

pub struct PositionClient {
    _node: Arc<Mutex<rclrs::Node>>,
    _client: Arc<rclrs::Client<Position>>
}

impl PositionClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let _node = Arc::new(Mutex::new(rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_client", &device).as_str())?));
        let node_clone = Arc::clone(&_node);
        let mut node = node_clone.lock().unwrap();
        let _client = node.create_client::<Position>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        Ok(Self{_node:_node, _client:_client})
    }

    async fn send_request(&self, position: &i32) -> Result<(), Error> {
        let request = science_interfaces_rs::srv::Position_Request{position: *position};
        let future = self._client.call_async(&request);
        println!("Request sent.");
        let response = future.await?;
        match response.success {
            true => { println!("Request completed. Now at position {}.", &response.position); },
            false => {
                println!("Request failed! Stopped at {}.", &response.position);
                println!("{}", &response.message);
            }
        }
        Ok(())
    }

    fn run(&self) {
        let node_clone = Arc::clone(&self._node);
        let node_thread = std::thread::spawn(move || {
            let node = node_clone.lock().unwrap();
            rclrs::spin(&node);
        });
        node_thread.join().expect("Unable to join node thread");
    }

    fn cli_control(&self) -> Result<(), Error> {
        self.run();
        let mut proceed: bool + 'static = true;
        let mut position: i32 + 'static;
        while proceed {
            loop {
                match input!("Enter an integer position value ({} | {}): ", "minimum => 0".bold().yellow(), "maximum => 2147483647".bold().yellow()).trim().to_lowercase().parse::<i32>()? {
                    a if a < 0 => {
                        position = 0;
                        break;
                    }
                    b if b >= 0 {
                        position = b;
                        break;
                    }
                    ParseIntError => { continue; }
                }
            }
            self.send_request(&position);
            loop {
                match input!("Enter {} to continue inputing commands, otherwise enter {}.", "true".bold().yellow(), "false".bold().yellow()).trim().to_lowercase().parse::<bool>()? {
                    Ok(c) => {
                        proceed = c;
                        break;
                    }
                    ParseBoolError => { continue; }
                }
            }
        }
        Ok(())
    }
}