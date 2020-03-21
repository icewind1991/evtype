use crate::common::Error;
use crate::keyboard::{char_to_key, create_device};
use evdev::{data, raw, uinput};
use main_error::MainError;
use std::fs;
use std::fs::Permissions;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::thread::sleep;
use std::time::Duration;

mod common;
mod keyboard;

const TYPE_DELAY: Duration = Duration::from_millis(10);

fn type_string(dev: &mut uinput::Device, text: &str) -> Result<(), Error> {
    for c in text.chars() {
        let (key, shift) = char_to_key(c)?;
        dev.write(data::KEY, key as u16, 1)?;
        if shift {
            dev.write(data::KEY, data::KEY_LEFTSHIFT as u16, 1)?;
        }
        dev.write(data::SYNCHRONIZATION, data::SYN_REPORT as u16, 0)?;
        sleep(TYPE_DELAY);
        dev.write(data::KEY, key as u16, 0)?;
        if shift {
            dev.write(data::KEY, data::KEY_LEFTSHIFT as u16, 0)?;
        }
        dev.write(data::SYNCHRONIZATION, data::SYN_REPORT as u16, 0)?;
        sleep(TYPE_DELAY);
    }
    Ok(())
}

fn main() -> Result<(), MainError> {
    let mut keyboard = create_device()?;

    let path = "/var/run/evtype.sock";

    let listener = UnixListener::bind(path)?;
    fs::set_permissions(path, Permissions::from_mode(0o666))?;

    let mut incoming = listener.incoming();

    println!("listening on {}", path);

    ctrlc::set_handler(move || {
        let _ = fs::remove_file(path);
        std::process::exit(0);
    })?;

    let mut text_buffer = String::new();

    while let Some(Ok(mut stream)) = incoming.next() {
        text_buffer.clear();
        match stream.read_to_string(&mut text_buffer) {
            Ok(_) => type_string(&mut keyboard, &text_buffer)?,
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}
