use err_derive::Error;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error(display = "Unsupported character '{}'", _0)]
    UnknownCharacter(char),
    #[error(display = "Error sending keycode: {}", _0)]
    Evdev(#[error(source)] nix::Error),
}
