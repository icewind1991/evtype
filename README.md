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