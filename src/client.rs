use dirs::runtime_dir;
use main_error::MainError;
use std::io::Write;
use std::os::unix::net::UnixStream;

fn main() -> Result<(), MainError> {
    let mut path = runtime_dir().ok_or("Can't get runtime directory")?;
    path.push("evtype.sock");

    let text = std::env::args().skip(1).next().unwrap_or_default();

    let mut stream = UnixStream::connect(&path)?;
    stream.write_all(text.as_bytes())?;

    Ok(())
}
