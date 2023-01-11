use std::{sync::{Arc, Mutex}, thread, time, str::{ParseBoolError, ParseIntError}};
use science_interfaces_rs::srv::Position;
use opencv::{prelude::*, highgui, videoio};
use cv_bridge_rs::CvImage;
use std_srvs::srv::SetBool;
use rppal::gpio::Gpio;
use anyhow::{Result, Error};
use colored::*;

pub struct GPIOServer {
    _node: rclrs::Node,
    _subsystem: String,
    _device: String,
    _server: Arc<rclrs::Server<_>>,
    _pin: Arc<Mutex<Gpio>>,
}

impl GPIOServer {
    fn new(&self, subsystem: String, device: String, pin_num: u8) -> Result<Self, Error> {
        let mut _node = rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_server", &device).as_str())?;
        let _pin = Arc::new(Mutex::new(Gpio::new()?.get(pin_num)?.into_output_low()));
        let pin_clone =  Arc::clone(&_pin);
        let _server = {
            _node.create_subscription(format!("/{}/{}/cmd", &subsystem, &device).as_str(),
                move |_request_header: &rclrs::rmw_request_id_t, request: SetBool::Request| -> SetBool::Response {
                    pin = *pin_clone.lock().unwrap();
                    if request.data {
                        pin.set_high();
                        SetBool::Response {success: true, message: format!("{} is on.", &device) }
                    }
                    pin.set_low();
                    SetBool::Response {success: true, message: format!("{} is off.", &device) }
            }?)
        };
        let _subsystem = subsystem;
        let _device = &device.replace("_", " ");
        Ok(Self{_node:_node, _subsystem:_subsystem, _device:_device, _server:_server})
    }

    fn run(&self) -> Result<(), Error> {
        Ok(rclrs::spin(&self._node)?)
    }
}

pub struct CameraServer {
    _node: rclrs::Node,
    _subsystem: String,
    _device: String,
    _server: Arc<rclrs::Server<_>>,
    _publisher: Arc<rclrs::Publisher<Image>>,
    _cam: videoio::VideoCapture,
    _capture_delay: u8,
    _active: Arc<Mutex<bool>>,
}

impl CameraServer {
    fn new(subsystem: String, device: String, camera_num: String, frame_width: u16, frame_height: u16, capture_delay: u16) -> Result<Self, Error> { // capture delay is in milliseconds
        let mut _node = rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_server", &device).as_str())?;
        let _active = Arc::new(Mutex::(false));
        let active_clone =  Arc::clone(&active);
        let _server = {
            _node.create_subscription(format!("/{}/{}/cmd", &subsystem, &device).as_str(),
                move |_request_header: &rclrs::rmw_request_id_t, request: SetBool::Request| -> SetBool::Response {
                    if request.data == *active_clone.lock().unwrap() {
                        SetBool::Response {success: true, message: format!("{} is already in requested state.", &device).yellow() }
                    }
                    *active_clone.lock().unwrap() = request.data;
                    SetBool::Response {success: true, message: format!("{} is now in requested state.", &device) }
            }?)
        };
        let _publisher = _node.create_publisher(format!("/{}/{}/images", &subsystem, &device).as_str(), rclrs::QOS_PROFILE_DEFAULT)?;
        let _cam = videoio::VideoCapture(camera_num)?;
        _cam.set(videoio::CAP_PROP_FRAME_WIDTH, frame_width);
        _cam.set(videoio::CAP_PROP_FRAME_HEIGHT, frame_height);
        let _capture_delay = capture_delay; 
        let _subsystem = subsystem;
        let _device = &device.replace("_", " ");
        Ok(Self{_node:_node, _subsystem:_subsystem, _device:_device, _server:_server, _publisher:_publisher, _cam:_cam, _capture_delay:_capture_delay, _active:_active})
    }

    fn run(&self) -> Result<(), Error> {
        let active_clone = Arc::clone(&self._active);
        std::thread::spawn(move || -> Result<(), Error> {
            let active = *active_clone.lock().unwrap();
            loop {
                if active {
                    let mut frame = Mat::default();
                    self._cam.read(&mut frame)?;
                    println!("Publishing frame!");
                    self._publisher.publisher.publish(CvImage::from_cvmat(frame).into_imgmsg())?;
                    std::thread::sleep(std::time::Duration::from_millis(self._capture_delay));
                }
            }
        }?);
        Ok(rclrs::spin(self._node)?)
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
        Command::new("ticcmd").arg("--position").arg(format!("{}", position)).spawn().expect("Failed to set stepper motor target position.".red());
    }
    fn set_relative_target(position: &i32) {
        Command::new("ticcmd").arg("--set-target-position-relative").arg(format!("{}", position)).spawn().expect("Failed to set relative stepper motor target position.".red());
    }
    fn set_target_velocity(velocity: &i32) {
        Command::new("ticcmd").arg("--set-target-velocity").arg(format!("{}", velocity)).spawn().expect("Failed to set stepper motor target velocity.".red());
    }
    fn halt_and_set_position(position: &i32) {
        Command::new("ticcmd").arg("--halt-and-set-position").arg(format!("{}", position)).spawn().expect("Failed to halt and set stepper motor position.".red());
    }
    fn halt_and_hold() {
        Command::new("ticcmd").arg("--halt-and-hold").spawn().expect("Failed to halt and hold stepper motor.".red());
    }
    fn go_home_forward() {
        Command::new("ticcmd").arg("--home").arg("fwd").spawn().expect("Failed to go to stepper motor home foward.".red());
    }
    fn reset_command_timeout() {
        Command::new("ticcmd").arg("--reset-command-timeout").spawn().expect("Failed to reset stepper motor command timeout.".red());
    }
    fn deenergize() {
        Command::new("ticcmd").arg("--deenergize").spawn().expect("Failed to deenergize stepper motor.".red());
    }
    fn energize() {
        Command::new("ticcmd").arg("--energize").spawn().expect("Failed to energize stepper motor.".red());
    }
    fn exit_safe_start() {
        Command::new("ticcmd").arg("--exit-safe-start").spawn().expect("Failed to exit stepper motor safe start.".red());
    }
    fn enter_safe_start() {
        Command::new("ticcmd").arg("--enter-safe-start").spawn().expect("Failed to enter stepper motor safe start.".red());
    }
    fn reset() {
        Command::new("ticcmd").arg("--reset").spawn().expect("Failed to reset stepper motor driver.".red());
    }
    fn clear_driver_error() {
        Command::new("ticcmd").arg("--clear-driver-error").spawn().expect("Failed to clear stepper motor driver error.".red());
    }
    fn set_max_speed(speed: &u32) {
        Command::new("ticcmd").arg("--set-max-speed").arg(format!("{}", speed)).spawn().expect("Failed to set stepper motor max speed.".red());
    }
    fn set_starting_speed(speed: &u32) {
        Command::new("ticcmd").arg("--set-starting-speed").arg(format!("{}", speed)).spawn().expect("Failed to set stepper motor starting speed.".red());
    }
    fn set_max_accel(accel: &u32) {
        Command::new("ticcmd").arg("--set-max-accel").arg(format!("{}", accel)).spawn().expect("Failed to set stepper motor max acceleration.".red());
    }
    fn set_max_deccel(deccel: &u32) {
        Command::new("ticcmd").arg("--set-max-deccel").arg(format!("{}", deccel)).spawn().expect("Failed to set stepper motor max decceleration.".red());
    }
    fn set_step_mode(mode: TicStepMode) {
        Command::new("ticcmd").arg("--step-mode").arg(format!("{}", mode)).spawn().expect("Failed to set stepper motor step mode.".red());
    }
    fn set_current_limit(limit: &u16) {
        Command::new("ticcmd").arg("--set-current-limit").arg(format!("{}", limit)).spawn().expect("Failed to set stepper motor current limit.".red());
    }
    fn save_settings() {
        Command::new("ticcmd").arg("--settings").arg("./src/rust_tests/stepper_motor_config.yaml").spawn().expect("Failed to save stepper motor settings.".red());
    }
    fn load_settings() {
        Command::new("ticcmd").arg("--settings").arg("./src/rust_tests/stepper_motor_config.yaml").spawn().expect("Failed to load stepper motor settings.".red());
    }
    fn status() {
        Command::new("ticcmd").arg("--status").arg("--full").spawn().expect("Failed to get stepper motor driver status.".red());
    }
    fn reached_top() -> Result<bool, Error> {
        todo!()
    }
    fn reached_bottom() -> Result<bool, Error> {
        todo!()
    }
}

pub struct StepperMotorServer {
    _node: rclrs::Node,
    _subsystem: String,
    _device: String,
    _server: Arc<rclrs::Server<_>>,
}

impl ServerNode for StepperMotorServer {
    fn new(&self, subsystem: String, device: String) -> Result<Self, Error> {
        let mut _node = rclrs::Node::new(&rclrs::Context::new(env::args())?, format!("{}_server", &device).as_str())?;
        let _server = {
            _node.create_subscription(format!("/{}/{}/cmd", &subsystem, &device).as_str(),
                move |_request_header: &rclrs::rmw_request_id_t, request: Position::Request| -> Position::Response {
                    let requested_displacement: Result<i32, Error> = request.position-TicDriver.get_current_position()?;
                    match requested_displacement {
                        Ok(i32) => {
                            println!("New position requested!");
                            if requested_displacement != 0 {
                                let is_direction_downward: bool = (requested_displacement > 0);
                                TicDriver.clear_driver_error();
                                TicDriver.energize();
                                TicDriver.exit_safe_start();
                                TicDriver.set_target_position_relative(position_delta);
                                match is_direction_downward {
                                    true => {
                                        while !TicDriver.reached_bottom() {
                                            TicDriver.reset_command_timeout();
                                        }
                                    }
                                    false => {
                                        while !TicDriver.reached_top() {
                                            TicDriver.reset_command_timeout();
                                        }
                                    }
                                }
                                TicDriver.deenergize();
                                TicDriver.enter_safe_start();
                                Position::Response {success: true, position: TicDriver.get_current_position(), errors: "" }
                            }
                            Position::Response{success: false, position: TicDriver.get_current_position(), errors: "Already at requested position.".yellow()}
                        },
                        Error => {
                            Position::Response{success: false, position: TicDriver.get_current_position(), errors: "Invalid request!".red() }
                        }
                    }
                }?
            )
        };
        let _subsystem = subsystem;
        let _device = &device.replace("_", " ");
        // Zero the platform's height
        TicDriver.set_current_limit(3200);
        TicDriver.set_current_limit(TicStepMode::Microstep256);
        TicDriver.save_settings();
        TicDriver.load_settings();
        TicDriver.clear_driver_error();
        TicDriver.clear_driver_error();
        TicDriver.energize();
        TicDriver.exit_safe_start();
        TicDriver.go_home_reverse();
        while !TicDriver.reached_top() {
            TicDriver.reset_command_timeout();
        }
        TicDriver.halt_and_set_position(0);
        TicDriver.deenergize();
        TicDriver.enter_safe_start();
        Ok(Self{_node:_node, _subsystem:_subsystem, _device:_device, _server:_server})
    }

    fn run(&self) -> Result<(), Error> {
        Ok(rclrs::spin(self._node)?)
    }
}