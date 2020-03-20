use main_error::MainError;
use std::io::Write;
use std::os::unix::net::UnixStream;

fn main() -> Result<(), MainError> {
    let path = "/var/run/evtype.sock";

    let text = std::env::args().skip(1).next().unwrap_or_default();

    let mut stream = UnixStream::connect(path)?;
    stream.write_all(text.as_bytes())?;

    Ok(())
}
