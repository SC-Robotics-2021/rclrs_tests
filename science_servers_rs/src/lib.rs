use std::{sync::Arc, thread, time};
use science_interfaces_rs::srv::Position;
use opencv::{prelude::*, highgui, videoio};
use std_srvs::srv::SetBool;
use rppal::gpio::Gpio;
use no_panic::no_panic;

trait ServerNode {
    node: rclrs::Node;
    _subsystem: String;
    _device: String;
    _server: Arc<rclrs::Server<_>>;
    fn new() -> Result<Self, Error>; 
}

trait ServerExecution {
    #[no_panic]
    fn run(&self);
}

pub struct GPIOServer {
    _pin: Arc<Mutex<Gpio>>;
}

impl ServerExecution for GPIOServer {
    #[no_panic]
    fn run(&self) {
        loop {
            let process = rclrs::spin(&self.node)?;
            match process {
                Ok(()) => { break; }
                Error => {}
            }
        }
    }
}

impl ServerNode for GPIOServer {
    #[no_panic]
    fn new(&self, subsystem, device, pin_num) -> Result<Self, Error> {
        let mut node = rclrs::Node::new(rclrs::Context::new(env::args())?, format!("{&device}_server"))?;
        let _pin = Arc::new(Mutex::new(Gpio::new()?.get(pin_num)?.into_output_low()));
        let pin_clone =  Arc::clone(&_pin);
        let _server = {
            node.create_subscription(format!("/{&subsystem}/{&device}/cmd"),
                move |_request_header: &rclrs::rmw_request_id_t, request: SetBool.Request| -> SetBool.Response {
                    pin = *pin_clone.lock().unwrap()
                    if request.data {
                        pin.set_high();
                        SetBool.Response {success: true, message: "{&self._device} is on." }
                    }
                    pin.set_low();
                    SetBool.Response {success: true, message: format!("{&self._device} is off.") }
            })?
        };
        let _subsystem = subsystem;
        let _device = device;
        Ok(Self{node, _subsystem, _device, _server})
    }
}

pub struct CameraServer {
    _publisher: Arc<rclrs::Publisher<Image>>;
    _cam: videoio::VideoCapture;
    _capture_delay: u8;
    _active: Arc<Mutex<bool>>;
}

impl ServerExecution for CameraServer {
    #[no_panic]
    fn run(&self) {
        loop {
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
            });
            let process = rclrs::spin(&self.node)?;
            match process { // this match statement will catch an error and rerun the node in the event it crashes
                Ok(()) => { break; },
                Error => {}
            }
        }
    }
}

impl ServerNode for CameraServer {
    #[no_panic]
    fn new(&self, subsystem, device, camera_num, frame_width, frame_height, capture_delay) -> Result<Self, Error> { // capture delay is in milliseconds
        let mut node = rclrs::Node::new(rclrs::Context::new(env::args())?, format!("{&device}_server"))?;
        let _active = Arc::new(Mutex::(false));
        let active_clone =  Arc::clone(&active);
        let _server = {
            node.create_subscription(format!("/{&subsystem}/{&device}/cmd"),
                move |_request_header: &rclrs::rmw_request_id_t, request: SetBool.Request| -> SetBool.Response {
                    active = *active_clone.lock().unwrap()
                    if request.data == active {
                        SetBool.Response {success: false, message: "Already in requested state" }
                    }
                    if request.data {
                        SetBool_Response {success: true, message: format!("{&self._device} is now on.") }
                    }
                    SetBool.Response {success: true, message: format!("{&self._device} is now off.") }
            })?
        };
        let _publisher = node.create_publisher(format!("/{&subsystem}/{&device}/images"), rclrs::QOS_PROFILE_DEFAULT)?;
        let _cam = videoio::VideoCapture(camera_num)?;
        _cam.set(videoio::CAP_PROP_FRAME_WIDTH, frame_width);
        _cam.set(videoio::CAP_PROP_FRAME_HEIGHT, frame_height);
        let _capture_delay = capture_delay; 
        let _subsystem = subsystem;
        let _device = device;
        Ok(Self{node, _subsystem, _device, _server, _publisher, _cam, _capture_delay, _active})
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

struct TicDriver {
    fn get_current_position() -> Result<i32, Error>;
    fn set_target_position(position: &i32);
    fn set_target_position_relative(position: &i32);
    fn set_target_velocity(velocity: &i32);
    fn halt_and_set_position(position: &i32);
    fn halt_and_hold();
    fn go_home_forward();
    fn go_home_reverse();
    fn reset_command_timeout();
    fn deenergize();
    fn energize();
    fn exit_safe_start();
    fn enter_safe_start();
    fn clear_driver_error();
    fn set_max_speed(speed: &u32);
    fn set_starting_speed(speed: &u32);
    fn set_max_accel(accel: &u32);
    fn set_max_deccel(deccel: &u32);
    fn set_current_limit(limit: &u16);
    fn status();
    fn reached_top() -> Result<bool, Error>;
    fn reached_bottom() -> Result<bool, Error>;
}

impl TicDriver {
    #[no_panic]
    fn get_current_position() -> Result<i32, Error> {
        todo!();
    }
    #[no_panic]
    fn set_target_position(position: &i32) {
        Command::new("ticcmd").arg("--position").arg(format!"{position}").spawn().expect(format!("{'Failed to set stepper motor target position.'.red()}"));
    }
    #[no_panic]
    fn set_relative_target(position: &i32) {
        Command::new("ticcmd").arg("--set-target-position-relative").arg(format!"{position}").spawn().expect(format!("{'Failed to set relative stepper motor target position.'.red()}"));
    }
    #[no_panic]
    fn set_target_velocity(velocity: &i32) {
        Command::new("ticcmd").arg("--set-target-velocity").arg(format!"{velocity}").spawn().expect(format!("{'Failed to set stepper motor target velocity.'.red()}"));
    }
    #[no_panic]
    fn halt_and_set_position(position: &i32) {
        Command::new("ticcmd").arg("--halt-and-set-position").arg(format!"{position}").spawn().expect(format!("{'Failed to halt and set stepper motor position.'.red()}"));
    }
    #[no_panic]
    fn halt_and_hold() {
        Command::new("ticcmd").arg("--halt-and-hold").spawn().expect(format!("{'Failed to halt and hold stepper motor.'.red()}"));
    }
    fn go_home_forward() {
        Command::new("ticcmd").arg("--home").arg("fwd").spawn().expect(format!("{'Failed to go to stepper motor home foward.'.red()}"));
    }
    #[no_panic]
    fn reset_command_timeout() {
        Command::new("ticcmd").arg("--reset-command-timeout").spawn().expect(format!("{'Failed to reset stepper motor command timeout.'.red()}"));
    }
    #[no_panic]
    fn deenergize() {
        Command::new("ticcmd").arg("--deenergize").spawn().expect(format!("{'Failed to deenergize stepper motor.'.red()}"));
    }
    #[no_panic]
    fn energize() {
        Command::new("ticcmd").arg("--energize").spawn().expect(format!("{'Failed to energize stepper motor.'.red()}"));
    }
    #[no_panic]
    fn exit_safe_start() {
        Command::new("ticcmd").arg("--exit-safe-start").spawn().expect(format!("{'Failed to exit stepper motor safe start.'.red()}"));
    }
    #[no_panic]
    fn enter_safe_start() {
        Command::new("ticcmd").arg("--enter-safe-start").spawn().expect(format!("{'Failed to enter stepper motor safe start.'.red()}"));
    }
    #[no_panic]
    fn reset() {
        Command::new("ticcmd").arg("--reset").spawn().expect(format!("{'Failed to reset stepper motor driver.'.red()}"));
    }
    #[no_panic]
    fn clear_driver_error() {
        Command::new("ticcmd").arg("--clear-driver-error").spawn().expect(format!("{'Failed to clear stepper motor driver error.'.red()}"));
    }
    #[no_panic]
    fn set_max_speed(speed: &u32) {
        Command::new("ticcmd").arg("--set-max-speed").arg(format!("{speed}")).spawn().expect(format!("{'Failed to set stepper motor max speed.'.red()}"));
    }
    #[no_panic]
    fn set_starting_speed(speed: &u32) {
        Command::new("ticcmd").arg("--set-starting-speed").arg(format!("{speed}")).spawn().expect(format!("{'Failed to set stepper motor starting speed.'.red()}"));
    }
    fn set_max_accel(accel: &u32) {
        Command::new("ticcmd").arg("--set-max-accel").arg(format!("{accel}")).spawn().expect(format!("{'Failed to set stepper motor max acceleration.'.red()}"));
    }
    #[no_panic]
    fn set_max_deccel(deccel: &u32) {
        Command::new("ticcmd").arg("--set-max-deccel").arg(format!("{deccel}")).spawn().expect(format!("{'Failed to set stepper motor max decceleration.'.red()}"));
    }
    #[no_panic]
    fn set_step_mode(mode: TicStepMode) {
        Command::new("ticcmd").arg("--step-mode").arg(format!("{mode}")).spawn().expect(format!("{'Failed to set stepper motor step mode.'.red()}"));
    }
    #[no_panic]
    fn set_current_limit(limit: &u16) {
        Command::new("ticcmd").arg("--set-current-limit").arg(format!("{limit}")).spawn().expect(format!("{'Failed to set stepper motor current limit.'.red()}"));
    }
    #[no_panic]
    fn save_settings() {
        Command::new("ticcmd").arg("--settings").arg("./src/rust_tests/stepper_motor_config.yaml").spawn().expect(format!("{'Failed to save stepper motor settings.'.red()}"));
    }
    #[no_panic]
    fn load_settings() {
        Command::new("ticcmd").arg("--settings").arg("./src/rust_tests/stepper_motor_config.yaml").spawn().expect(format!("{'Failed to load stepper motor settings.'.red()}"));
    }
    #[no_panic]
    fn status() {
        Command::new("ticcmd").arg("--status").arg("--full").spawn().expect(format!("{'Failed to get stepper motor driver status.'.red()}"));
    }
    #[no_panic]
    fn reached_top() -> Result<bool, Error> {
        todo!()
    }
    #[no_panic]
    fn reached_bottom() -> Result<bool, Error> {
        todo!()
    }
}

pub struct StepperMotorServer {}

impl ServerExecution for StepperMotorServer {
    #[no_panic]
    fn run(&self) {
        loop {
            let process = rclrs::spin(&self.node)?;
            match process {
                Ok(_) => { break; }
                Error => {}
            }
        }
    }
}

impl ServerNode for StepperMotorServer {
    #[no_panic]
    fn new(&self, subsystem, device) -> Result<Self, Error> {
        let mut node = rclrs::Node::new(rclrs::Context::new(env::args())?, format!("{&device}_server"))?;
        let _server = {
            node.create_subscription(format!("/{&subsystem}/{&device}/cmd"),
                move |_request_header: &rclrs::rmw_request_id_t, request: Position.Request| -> Position.Response {
                    let requested_displacement: i32 = request.position-TicDriver.get_current_position()?;
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
                                Position_Response {success: true, position: TicDriver.get_current_position(), errors: "" }
                            }
                            Position_Response{success: false, position: TicDriver.get_current_position(), errors: format!("{'Already at requested position.'.red()}")}
                        },
                        Error => {
                            Position_Response{success: false, position: TicDriver.get_current_position(), errors: format!("{'Invalid request!'.red()}") }
                        }
                    }
                }
            )?
        };
        let _subsystem = subsystem;
        let _device = device;
        let _active = active;
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
        Ok(Self{node, _subsystem, _device, _server, _active})
    }
}