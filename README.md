# EvType

Virtual typing using evdev.

## What

EvType is made to replace the `xdotool type` command for wayland systems, where there is no option for virtual keyboard input.

## Usage

- Start the `evtype_daemon` as root using your favorite init daemon (a systemd unit is [included](evtype.service)).
- Run `evtype <text>` to enter some text trough the virtual keyboard.

## Why a separate daemon

For security reasons your user generally doesn't have permissions to talk directly to evdev so root permissions are required.
By having a separate daemon do the talking to evdev the `evtype` tool doesn't need to be run as root and only a restricted api
is exposed to non-root users (i.e. only typing printable characters, no keyboard logging).

## Limitations

EvType currently always assumes qwerty layout and might result in unexpected results when the system is configured
with a different keyboard layout.

## Programmatic usage

You can use the `evtype_daemon` from your own programs by connecting to the unix socket at `/var/run/evtype.sock` and
send the text to be typed over the socket.

A basic rust example:

```rust
use std::io::Write;
use std::os::unix::net::UnixStream;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = UnixStream::connect("/var/run/evtype.sock")?;
    stream.write_all("hello world".as_bytes())?;
    Ok(());
}
```  