use crate::keyboard::{char_to_key, create_device};
use evdev::{EventType, InputEvent, Key, Synchronization};
use main_error::MainError;
use std::fs;
use std::fs::{create_dir_all, Permissions};
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;
use evdev::uinput::VirtualDevice;

mod keyboard;

const TYPE_DELAY: Duration = Duration::from_millis(10);

fn type_string(dev: &mut VirtualDevice, text: &str) -> Result<(), MainError> {
    for c in text.chars() {
        let (key, shift) = char_to_key(c)?;
        if shift {
            dev.emit(&[InputEvent::new(EventType::KEY, Key::KEY_LEFTSHIFT.0, 1)])?;
        }
        dev.emit(&[InputEvent::new(EventType::KEY, key.0, 1)])?;
        dev.emit(&[InputEvent::new(EventType::SYNCHRONIZATION, Synchronization::SYN_REPORT.0, 1)])?;
        sleep(TYPE_DELAY);
        dev.emit(&[InputEvent::new(EventType::KEY, key.0, 0)])?;
        if shift {
            dev.emit(&[InputEvent::new(EventType::KEY, Key::KEY_LEFTSHIFT.0, 0)])?;
        }
        dev.emit(&[InputEvent::new(EventType::SYNCHRONIZATION, Synchronization::SYN_REPORT.0, 0)])?;
        sleep(TYPE_DELAY);
    }
    Ok(())
}

fn main() -> ExitCode {
    let mut keyboard = match create_device() {
        Ok(keyboard) => keyboard,
        Err(e) => {
            eprintln!("Failed to create virtual keyboard: {e:?}");
            return ExitCode::FAILURE;
        }
    };

    let path = "/var/run/evtype/evtype.sock";
    let dir = Path::new("/var/run/evtype");

    if !dir.exists() {
        if let Err(e) = create_dir_all(dir) {
            eprintln!("Failed to create socket directory {}: {e:#}", dir.display());
            return ExitCode::FAILURE;
        }
    }
    let listener = match UnixListener::bind(path) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to socket {path}: {e:#}");
            return ExitCode::FAILURE;
        }
    };
    if let Err(e) = fs::set_permissions(path, Permissions::from_mode(0o666)) {
        eprintln!("Failed to set socket permissions for {path}: {e:#}");
        return ExitCode::FAILURE;
    }

    let mut incoming = listener.incoming();

    println!("listening on {}", path);

    ctrlc::set_handler(move || {
        let _ = fs::remove_file(path);
        std::process::exit(0);
    }).ok();

    let mut text_buffer = String::new();

    while let Some(Ok(mut stream)) = incoming.next() {
        text_buffer.clear();
        match stream.read_to_string(&mut text_buffer) {
            Ok(_) => {
                if let Err(e) = type_string(&mut keyboard, &text_buffer) {
                    eprintln!("Failed to type string: {e:?}");
                }
            },
            Err(e) => eprintln!("Failed to read from socket: {:#}", e),
        }
    }

    ExitCode::SUCCESS
}
