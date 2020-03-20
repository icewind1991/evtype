use crate::common::Error;
use evdev::raw::uinput_setup;
use evdev::{
    data,
    uinput::{Builder, Device},
    uinput_ioctl,
};

pub fn char_to_key(c: char) -> Result<(data::Key, bool), Error> {
    match c {
        '0' => Ok((data::KEY_0, false)),
        '1' => Ok((data::KEY_1, false)),
        '2' => Ok((data::KEY_2, false)),
        '3' => Ok((data::KEY_3, false)),
        '4' => Ok((data::KEY_4, false)),
        '5' => Ok((data::KEY_5, false)),
        '6' => Ok((data::KEY_6, false)),
        '7' => Ok((data::KEY_7, false)),
        '8' => Ok((data::KEY_8, false)),
        '9' => Ok((data::KEY_9, false)),
        ')' => Ok((data::KEY_0, true)),
        '!' => Ok((data::KEY_1, true)),
        '@' => Ok((data::KEY_2, true)),
        '#' => Ok((data::KEY_3, true)),
        '$' => Ok((data::KEY_4, true)),
        '%' => Ok((data::KEY_5, true)),
        '^' => Ok((data::KEY_6, true)),
        '&' => Ok((data::KEY_7, true)),
        '*' => Ok((data::KEY_8, true)),
        '(' => Ok((data::KEY_9, true)),
        'A' => Ok((data::KEY_A, true)),
        'B' => Ok((data::KEY_B, true)),
        'C' => Ok((data::KEY_C, true)),
        'D' => Ok((data::KEY_D, true)),
        'E' => Ok((data::KEY_E, true)),
        'F' => Ok((data::KEY_F, true)),
        'G' => Ok((data::KEY_G, true)),
        'H' => Ok((data::KEY_H, true)),
        'I' => Ok((data::KEY_I, true)),
        'J' => Ok((data::KEY_J, true)),
        'K' => Ok((data::KEY_K, true)),
        'L' => Ok((data::KEY_L, true)),
        'M' => Ok((data::KEY_M, true)),
        'N' => Ok((data::KEY_N, true)),
        'O' => Ok((data::KEY_O, true)),
        'P' => Ok((data::KEY_P, true)),
        'Q' => Ok((data::KEY_Q, true)),
        'R' => Ok((data::KEY_R, true)),
        'S' => Ok((data::KEY_S, true)),
        'T' => Ok((data::KEY_T, true)),
        'U' => Ok((data::KEY_U, true)),
        'V' => Ok((data::KEY_V, true)),
        'W' => Ok((data::KEY_W, true)),
        'X' => Ok((data::KEY_X, true)),
        'Y' => Ok((data::KEY_Y, true)),
        'Z' => Ok((data::KEY_Z, true)),
        'a' => Ok((data::KEY_A, false)),
        'b' => Ok((data::KEY_B, false)),
        'c' => Ok((data::KEY_C, false)),
        'd' => Ok((data::KEY_D, false)),
        'e' => Ok((data::KEY_E, false)),
        'f' => Ok((data::KEY_F, false)),
        'g' => Ok((data::KEY_G, false)),
        'h' => Ok((data::KEY_H, false)),
        'i' => Ok((data::KEY_I, false)),
        'j' => Ok((data::KEY_J, false)),
        'k' => Ok((data::KEY_K, false)),
        'l' => Ok((data::KEY_L, false)),
        'm' => Ok((data::KEY_M, false)),
        'n' => Ok((data::KEY_N, false)),
        'o' => Ok((data::KEY_O, false)),
        'p' => Ok((data::KEY_P, false)),
        'q' => Ok((data::KEY_Q, false)),
        'r' => Ok((data::KEY_R, false)),
        's' => Ok((data::KEY_S, false)),
        't' => Ok((data::KEY_T, false)),
        'u' => Ok((data::KEY_U, false)),
        'v' => Ok((data::KEY_V, false)),
        'w' => Ok((data::KEY_W, false)),
        'x' => Ok((data::KEY_X, false)),
        'y' => Ok((data::KEY_Y, false)),
        'z' => Ok((data::KEY_Z, false)),
        '-' => Ok((data::KEY_MINUS, false)),
        '_' => Ok((data::KEY_MINUS, true)),
        '=' => Ok((data::KEY_EQUAL, false)),
        '+' => Ok((data::KEY_EQUAL, true)),
        ',' => Ok((data::KEY_COMMA, false)),
        '<' => Ok((data::KEY_COMMA, true)),
        '.' => Ok((data::KEY_DOT, false)),
        '>' => Ok((data::KEY_DOT, true)),
        '[' => Ok((data::KEY_LEFTBRACE, false)),
        '{' => Ok((data::KEY_LEFTBRACE, true)),
        ']' => Ok((data::KEY_RIGHTBRACE, false)),
        '}' => Ok((data::KEY_RIGHTBRACE, true)),
        ';' => Ok((data::KEY_SEMICOLON, false)),
        ':' => Ok((data::KEY_SEMICOLON, true)),
        '\'' => Ok((data::KEY_APOSTROPHE, false)),
        '"' => Ok((data::KEY_APOSTROPHE, true)),
        '`' => Ok((data::KEY_GRAVE, false)),
        '~' => Ok((data::KEY_GRAVE, true)),
        '\\' => Ok((data::KEY_BACKSLASH, false)),
        '|' => Ok((data::KEY_BACKSLASH, true)),
        '/' => Ok((data::KEY_SLASH, false)),
        '?' => Ok((data::KEY_SLASH, true)),
        ' ' => Ok((data::KEY_SPACE, false)),
        '\n' => Ok((data::KEY_ENTER, false)),
        c => Err(Error::UnknownCharacter(c)),
    }
}

pub fn create_device() -> Result<Device, Error> {
    let mut conf = uinput_setup::default();
    conf.set_name("EvType")?;
    conf.id.bustype = 0x16;
    conf.id.vendor = 69;
    conf.id.product = 69;

    let builder = Builder::new(&std::path::Path::new("/dev/uinput"))?;

    uinput_ioctl!(ui_set_evbit(builder.fd(), data::KEY.number()))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_0 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_1 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_2 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_3 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_4 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_5 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_6 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_7 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_8 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_9 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_0 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_1 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_2 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_3 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_4 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_5 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_6 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_7 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_8 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_9 as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_A as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_B as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_C as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_D as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_E as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_F as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_G as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_H as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_I as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_J as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_K as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_L as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_M as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_N as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_O as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_P as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_Q as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_R as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_S as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_T as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_U as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_V as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_W as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_X as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_Y as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_Z as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_A as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_B as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_C as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_D as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_E as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_F as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_G as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_H as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_I as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_J as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_K as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_L as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_M as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_N as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_O as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_P as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_Q as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_R as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_S as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_T as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_U as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_V as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_W as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_X as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_Y as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_Z as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_MINUS as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_MINUS as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_EQUAL as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_EQUAL as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_COMMA as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_COMMA as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_DOT as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_DOT as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_LEFTBRACE as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_LEFTBRACE as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_RIGHTBRACE as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_RIGHTBRACE as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_SEMICOLON as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_SEMICOLON as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_APOSTROPHE as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_APOSTROPHE as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_GRAVE as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_GRAVE as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_BACKSLASH as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_BACKSLASH as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_SLASH as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_SLASH as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_SPACE as i32))?;
    uinput_ioctl!(ui_set_keybit(builder.fd(), data::KEY_ENTER as i32))?;

    Ok(builder.setup(conf)?)
}
