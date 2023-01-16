use std::{env::args, sync::{Arc, Mutex}, str::ParseBoolError, num::ParseIntError, thread::};
use rclrs::{Node, RclrsError, Subscription, Client, Context, spin}
use anyhow::{Result, Error};
use input_macro::input;
use colored::*;
use opencv::{highgui::{imshow, wait_key}, prelude::*};
use cv_bridge_rs::CvImage;
use std_srvs::srv::*;
use science_interfaces_rs::srv::*;ƒ
use sensor_msgs::msg::Image;
use dialoguer::{Select, Confirm, console::Term};


pub struct OnOffClient {
    _node: Arc<Mutex<Node>>,
    _client: Arc<Client<SetBool>>
}

impl OnOffClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let _node = Arc::new(Mutex::new(Node::new(&Context::new(args())?, format!("{}_client", &device).as_str())?));
        let node_clone = Arc::clone(&_node);
        let mut node = node_clone.lock().unwrap();
        let _client = node.create_client::<SetBool>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        Ok(Self{_node:_node, _client:_client})
    }

    async fn send_request(&self, state: bool) -> Result<(), Error> {
        let request = SetBool_Request{data: state};
        let future = self._client.call_async(&request);
        println!("Request sent.");
        let response = future.await?;
        println!("{}", response.message);
        Ok(())
    }

    fn run(&self) {
        let node_clone = Arc::clone(&*self._node);
        let _node_thread = spawn(move || -> Result<(), RclrsError> {
            let mut node = node_clone.lock().unwrap();
            spin(&node)
        });
    }

    fn cli_control(&self) -> Result<(), Error> {
        self.run();
        let mut proceed: bool = true;
        while proceed {
            match Select::new().item("Turn off").item("Turn on").interact_on_opt(&Term::stdout())? {
                Some(input) => { &self.send_request(input != 0); },
                None => { unreachable!(); }
            }
            proceed = Confirm::new().with_prompt("Do you want to continue command and control?").interact()?;
        }
        Ok(())
    }
}

pub struct CameraClient {
    _node: Arc<Mutex<Node>>,
    _client: Arc<Client<SetBool>>,
    _subscription: Arc<Subscription<Image>>,
    _frame: Arc<Mutex<Option<Mat>>>
}

impl CameraClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let _node = Arc::new(Mutex::new(Node::new(&Context::new(args())?, format!("{}_client", &device).as_str())?));
        let node_clone = Arc::clone(&_node);
        let mut node = node_clone.lock().unwrap();
        let _client = node.create_client::<SetBool>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        let _frame = Arc::new(Mutex::new(None));
        let frame_clone = Arc::clone(&_frame);
        let _subscription = {
            node.create_subscription(format!("/{}/{}/images", &subsystem, &device).as_str(), QOS_PROFILE_DEFAULT,
                move |msg: Image| {
                    println!("Recieving new image!");
                    *frame_clone.lock().unwrap() = Some(CvImage::from_imgmsg(msg).as_cvmat("bgr8".to_string()));
                    if frame_clone.lock().unwrap().as_ref().unwrap().size().unwrap().width > 0 {
                        imshow(&device.replace("_", " "), &frame_clone.lock().unwrap().as_ref().unwrap());
                    }
                    let _key = wait_key(10);
                },
            )?
        };
        Ok(Self{_node:_node, _client:_client, _subscription:_subscription, _frame:_frame})
    }

    async fn send_request(&self, state: bool) -> Result<(), Error> {
        let request = SetBool_Request{data: state};
        let future = self._client.call_async(&request);
        println!("Request sent.");
        let response = future.await?;
        println!("{}", response.message);
        Ok(())
    }

    fn run(&self) {
        let node_clone = Arc::clone(&*self._node);
        let _node_thread = spawn(move || -> Result<(), RclrsError> {
            let mut node = node_clone.lock().unwrap();
            spin(&node)
        });
    }

    fn cli_control(&self) -> Result<(), Error> {
        self.run();
        let mut proceed: bool = true;
        while proceed {
            match Select::new().item("Turn off").item("Turn on").interact_on_opt(&Term::stdout())? {
                Some(input) => { &self.send_request(input != 0); },
                None => { unreachable!(); }
            }
            proceed = Confirm::new().with_prompt("Do you want to continue command and control?").interact()?;
        }
        Ok(())
    }
}

pub struct PositionClient {
    _node: Arc<Mutex<Node>>,
    _client: Arc<Client<Position>>
}

impl PositionClient {
    fn new(subsystem: String, device: String) -> Result<Self, Error> {
        let _node = Arc::new(Mutex::new(Node::new(&Context::new(args())?, format!("{}_client", &device).as_str())?));
        let node_clone = Arc::clone(&_node);
        let mut node = node_clone.lock().unwrap();
        let _client = node.create_client::<Position>(format!("/{}/{}/cmd", &subsystem, &device).as_str())?;
        Ok(Self{_node:_node, _client:_client})
    }

    async fn send_request(&self, position: i32) -> Result<(), Error> {
        let request = Position_Request{position: position};
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
        let node_clone = Arc::clone(&*self._node);
        let _node_thread = spawn(move || -> Result<(), RclrsError> {
            let mut node = node_clone.lock().unwrap();
            spin(&node)
        });
    }

    fn cli_control(&self) -> Result<(), Error> {
        self.run();
        let mut proceed: bool = true;
        while proceed {
            loop {
                match input!("Enter an integer position value ({} | {}): ", "minimum => 0".bold().yellow(), "maximum => 2147483647".bold().yellow()).trim().to_lowercase().parse::<i32>() {
                    Ok(input) => {
                        if input < 0 {
                            &self.send_request(0);
                        } else {
                            &self.send_request(input);
                        }
                        break;
                    },
                    ParseIntError => { unreachable!(); }
                }
            }
            proceed = Confirm::new().with_prompt("Do you want to continue command and control?").interact()?;
        }
        Ok(())
    }
}