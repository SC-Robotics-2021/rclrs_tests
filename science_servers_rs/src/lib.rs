use std::{sync::{Arc, Mutex}, env::args, thread::{spawn, sleep}, time::Duration, process::Command, str::ParseBoolError, num::ParseIntError};
use rclrs::{Node, Service, Publisher, Context, spin, RclrsError, rmw_request_id_t};
use science_interfaces_rs::srv::Position;
use opencv::{prelude::*, videoio, core::Vector};
use cv_bridge_rs::CvImage;
use sensor_msgs::msg::Image;
use std_srvs::srv::SetBool;
use rppal::gpio::{Gpio, OutputPin};
use anyhow::{Result, Error};
use colored::*;

pub struct GPIOServer {
    _node: Arc<Mutex<Node>>,
    _pin: Arc<Mutex<OutputPin>>,
    _server: Arc<Service<SetBool>>,
}

impl GPIOServer {
    fn new(subsystem: String, device: String, pin_num: u8) -> Result<Self, Error> {
        let _node = Arc::new(Mutex::new(Node::new(&Context::new(args())?, format!("{}_server", &device).as_str())?));
        let node_clone = Arc::clone(&_node);
        let mut node = node_clone.lock().unwrap();
        let _pin = Arc::new(Mutex::new(Gpio::new()?.get(pin_num)?.into_output_low()));
        let pin_clone =  Arc::clone(&_pin);
        let _server = node.create_service(format!("/{}/{}/cmd", &subsystem, &device).as_str(),
            move |_request_header: &rmw_request_id_t, request: std_srvs::srv::SetBool_Request| -> std_srvs::srv::SetBool_Response {
                let mut pin = pin_clone.lock().unwrap();
                let message: String;
                if request.data {
                    pin.set_high();
                    message = format!("{} is on.", &device);
                } else {
                    pin.set_low();
                    message = format!("{} is off.", &device);
                }
                std_srvs::srv::SetBool_Response{success: true, message: message }
            }
        )?;
        Ok(Self{_node:_node, _server:_server, _pin:_pin})
    }

    fn run(&self) {
        let node_clone = Arc::clone(&self._node);
        let _node_thread = spawn(move || -> Result<(), RclrsError> {
            let node = node_clone.lock().unwrap();
            spin(&node)
        });
    }
}

pub struct CameraServer {
    _node: Arc<Mutex<Node>>,
    _publisher: Arc<Publisher<Image>>,
    _camera_id: Arc<i32>,
    _camera_settings: Arc<Vector<i32>>,
    _capture_delay: Arc<u64>,
    _active: Arc<Mutex<bool>>,
    _server: Arc<Service<SetBool>>
}

impl CameraServer {
    fn new(subsystem: String, device: String, camera_id: i32, camera_settings: Vector<i32>, capture_delay: u64) -> Result<Self, Error> { // capture delay is in milliseconds
        let _node = Arc::new(Mutex::new(Node::new(&Context::new(args())?, format!("{}_server", &device).as_str())?));
        let node_clone = Arc::clone(&_node);
        let mut node = node_clone.lock().unwrap();
        let _publisher = Arc::new(node.create_publisher(format!("/{}/{}/images", &subsystem, &device).as_str(), rclrs::QOS_PROFILE_DEFAULT)?);
        let _active = Arc::new(Mutex::new(false));
        let active_clone =  Arc::clone(&_active);
        let _server = node.create_service(format!("/{}/{}/cmd", &subsystem, &device).as_str(),
            move |_request_header: &rmw_request_id_t, request: std_srvs::srv::SetBool_Request| -> std_srvs::srv::SetBool_Response {
                let message: String;
                let success: bool;
                if request.data != *active_clone.lock().unwrap() {
                    *active_clone.lock().unwrap() = request.data;
                    success = true;
                    message = format!("{} is now in requested state.", &device).to_string();

                } else {
                    success = false;
                    message = format!("{} is already in requested state.", &device).yellow().to_string();
                }
                std_srvs::srv::SetBool_Response{success: success, message: message}
            }
        )?;
        let _camera_id = Arc::new(camera_id);
        let _camera_settings = Arc::new(camera_settings);
        let _capture_delay = Arc::new(capture_delay); 
        Ok(Self{_node:_node, _server:_server, _publisher:_publisher, _camera_id:_camera_id, _camera_settings:_camera_settings, _capture_delay:_capture_delay, _active:_active})
    }

    fn run(&self) {
        let node_clone = Arc::clone(&self._node);
        let _node_thread = spawn(move || -> Result<(), RclrsError> {
            let node = node_clone.lock().unwrap();
            spin(&node)
        });

        let publisher = Arc::clone(&self._publisher);
        let camera_id = Arc::clone(&self._camera_id);
        let camera_settings = Arc::clone(&self._camera_settings);
        let capture_delay = Arc::clone(&self._capture_delay);
        let active_clone = Arc::clone(&self._active);

        let _publisher_thread = spawn(move || {
            let active = active_clone.lock().unwrap();
            let mut cam = videoio::VideoCapture::new_with_params(*camera_id, videoio::CAP_ANY, &*camera_settings).unwrap();
            loop {
                if *active {
                    let mut frame = Mat::default();
                    let _ = cam.read(&mut frame);
                    println!("Publishing frame!");
                    let _ = publisher.publish(CvImage::from_cvmat(frame).into_imgmsg());
                    sleep(Duration::from_millis(1/(camera_settings.get(5)? as u64)));
                }
            }
        });
    }

    fn define_settings(frame_width: i32, frame_height: i32, fps: i32) -> Vector<i32> {
        let mut settings = Vector::with_capacity(6);
        settings.push(videoio::CAP_PROP_FRAME_WIDTH);
        settings.push(frame_width);
        settings.push(videoio::CAP_PROP_FRAME_HEIGHT);
        settings.push(frame_height);
        settings.push(videoio::CAP_PROP_FPS);
        settings.push(fps);
        settings
    }
}

enum TicStepMode {
    Microstep1 = 0,
    Microstep2 = 1,
    Microstep4 = 2,
    Microstep8 = 3,
    Microstep16 = 4,
    Microstep32 = 5,
    Microstep64 = 7,
    Microstep128 = 8,
    Microstep256 = 9,
}

struct TicDriver {}

impl TicDriver {
    fn get_current_position() -> Result<i32, Error> {
        todo!();
    }
    fn set_target_position(position: &i32) {
        Command::new("ticcmd").arg("--position").arg(format!("{}", position))
        .spawn().expect(format!("{}", "Failed to set stepper motor target position.".red()).as_str());
    }
    fn set_target_position_relative(position: &i32) {
        Command::new("ticcmd").arg("--set-target-position-relative").arg(format!("{}", position))
        .spawn().expect(format!("{}", "Failed to set relative stepper motor target position.".red()).as_str());
    }
    fn set_target_velocity(velocity: &i32) {
        Command::new("ticcmd").arg("--set-target-velocity").arg(format!("{}", velocity))
        .spawn().expect(format!("{}", "Failed to set stepper motor target velocity.".red()).as_str());
    }
    fn halt_and_set_position(position: &i32) {
        Command::new("ticcmd").arg("--halt-and-set-position").arg(format!("{}", position))
        .spawn().expect(format!("{}", "Failed to halt and set stepper motor position.".red()).as_str());
    }
    fn halt_and_hold() {
        Command::new("ticcmd").arg("--halt-and-hold")
        .spawn().expect(format!("{}", "Failed to halt and hold stepper motor.".red()).as_str());
    }
    fn go_home_forward() {
        Command::new("ticcmd").arg("--home").arg("fwd")
        .spawn().expect(format!("{}", "Failed to go to stepper motor home foward.".red()).as_str());
    }
    fn go_home_reverse() {
        Command::new("ticcmd").arg("--home").arg("rev")
        .spawn().expect(format!("{}", "Failed to go to stepper motor home in reverse.".red()).as_str());
    }
    fn reset_command_timeout() {
        Command::new("ticcmd").arg("--reset-command-timeout")
        .spawn().expect(format!("{}", "Failed to reset stepper motor command timeout.".red()).as_str());
    }
    fn deenergize() {
        Command::new("ticcmd").arg("--deenergize")
        .spawn().expect(format!("{}", "Failed to deenergize stepper motor.".red()).as_str());
    }
    fn energize() {
        Command::new("ticcmd").arg("--energize")
        .spawn().expect(format!("{}", "Failed to energize stepper motor.".red()).as_str());
    }
    fn exit_safe_start() {
        Command::new("ticcmd").arg("--exit-safe-start")
        .spawn().expect(format!("{}", "Failed to exit stepper motor safe start.".red()).as_str());
    }
    fn enter_safe_start() {
        Command::new("ticcmd").arg("--enter-safe-start")
        .spawn().expect(format!("{}", "Failed to enter stepper motor safe start.".red()).as_str());
    }
    fn reset() {
        Command::new("ticcmd").arg("--reset")
        .spawn().expect(format!("{}", "Failed to reset stepper motor driver.".red()).as_str());
    }
    fn clear_driver_error() {
        Command::new("ticcmd").arg("--clear-driver-error")
        .spawn().expect(format!("{}", "Failed to clear stepper motor driver error.".red()).as_str());
    }
    fn set_max_speed(speed: &u32) {
        Command::new("ticcmd").arg("--set-max-speed").arg(format!("{}", speed))
        .spawn().expect(format!("{}", "Failed to set stepper motor max speed.".red()).as_str());
    }
    fn set_starting_speed(speed: &u32) {
        Command::new("ticcmd").arg("--set-starting-speed").arg(format!("{}", speed))
        .spawn().expect(format!("{}", "Failed to set stepper motor starting speed.".red()).as_str());
    }
    fn set_max_accel(accel: &u32) {
        Command::new("ticcmd").arg("--set-max-accel").arg(format!("{}", accel))
        .spawn().expect(format!("{}", "Failed to set stepper motor max acceleration.".red()).as_str());
    }
    fn set_max_deccel(deccel: &u32) {
        Command::new("ticcmd").arg("--set-max-deccel").arg(format!("{}", deccel))
        .spawn().expect(format!("{}", "Failed to set stepper motor max decceleration.".red()).as_str());
    }
    fn set_step_mode(mode: TicStepMode) {
        Command::new("ticcmd").arg("--step-mode").arg(format!("{}", mode as u8))
        .spawn().expect(format!("{}", "Failed to set stepper motor step mode.".red()).as_str());
    }
    fn set_current_limit(limit: &u16) {
        Command::new("ticcmd").arg("--set-current-limit").arg(format!("{}", limit))
        .spawn().expect(format!("{}", "Failed to set stepper motor current limit.".red()).as_str());
    }
    fn save_settings() {
        Command::new("ticcmd").arg("--settings").arg("./src/rust_tests/stepper_motor_config.yaml")
        .spawn().expect(format!("{}", "Failed to save stepper motor settings.".red()).as_str());
    }
    fn load_settings() {
        Command::new("ticcmd").arg("--settings").arg("./src/rust_tests/stepper_motor_config.yaml")
        .spawn().expect(format!("{}", "Failed to load stepper motor settings.".red()).as_str());
    }
    fn status() {
        Command::new("ticcmd").arg("--status").arg("--full")
        .spawn().expect(format!("{}", "Failed to get stepper motor driver status.".red()).as_str());
    }
    fn reached_top() -> Result<bool, Error> {
        todo!()
    }
    fn reached_bottom() -> Result<bool, Error> {
        todo!()
    }
}

pub struct StepperMotorServer {
    _node: Arc<Mutex<Node>>,
    _server: Arc<Service<Position>>
}

impl StepperMotorServer {
    fn new(&self, subsystem: String, device: String) -> Result<Self, Error> {
        // Zero the platform's height
        TicDriver::set_current_limit(&3200);
        TicDriver::set_step_mode(TicStepMode::Microstep256);
        TicDriver::save_settings();
        TicDriver::load_settings();
        TicDriver::clear_driver_error();
        TicDriver::clear_driver_error();
        TicDriver::energize();
        TicDriver::exit_safe_start();
        TicDriver::go_home_reverse();
        while !TicDriver::reached_top()? {
            TicDriver::reset_command_timeout();
        }
        TicDriver::halt_and_set_position(&0);
        TicDriver::deenergize();
        TicDriver::enter_safe_start();
        let _node = Arc::new(Mutex::new(Node::new(&Context::new(args())?, format!("{}_server", &device).as_str())?));
        let node_clone = Arc::clone(&_node);
        let mut node = node_clone.lock().unwrap();
        let _server = node.create_service(format!("/{}/{}/cmd", &subsystem, &device).as_str(),
            move |_request_header: &rmw_request_id_t, request: science_interfaces_rs::srv::Position_Request| -> science_interfaces_rs::srv::Position_Response {
                let success: bool;
                let message: String;
                println!("New position requested!");
                let requested_displacement: i32 = request.position-TicDriver::get_current_position().unwrap();
                if requested_displacement != 0 {
                    let is_direction_downward: bool = requested_displacement > 0;
                    TicDriver::clear_driver_error();
                    TicDriver::energize();
                    TicDriver::exit_safe_start();
                    TicDriver::set_target_position_relative(&requested_displacement);
                    if let true = is_direction_downward {
                        while !TicDriver::reached_bottom().unwrap() {
                            TicDriver::reset_command_timeout();
                        };
                    } else {
                        while !TicDriver::reached_top().unwrap() {
                            TicDriver::reset_command_timeout();
                        };
                    }
                    TicDriver::deenergize();
                    TicDriver::enter_safe_start();
                    success = true;
                    message = String::new();
                } else {
                    success = false;
                    message = "Already at requested position.".yellow().to_string();
                }
                science_interfaces_rs::srv::Position_Response{success: success, position: TicDriver::get_current_position().unwrap(), message: message}
            }
        )?;
        Ok(Self{_node:_node, _server:_server})
    }

    fn run(&self) {
        let node_clone = Arc::clone(&self._node);
        let _node_thread = spawn(move || -> Result<(), RclrsError> {
            let node = node_clone.lock().unwrap();
            spin(&node)
        });
    }
}