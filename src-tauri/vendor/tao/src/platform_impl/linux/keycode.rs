// Copyright 2014-2021 The winit contributors
// Copyright 2021-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use crate::keyboard::{KeyCode, NativeKeyCode};

pub fn keycode_to_scancode(code: KeyCode) -> Option<u32> {
  // See `from_scancode` for more info
  match code {
    KeyCode::Backquote => Some(0x0031),
    KeyCode::Backslash => Some(0x0033),
    KeyCode::Backspace => Some(0x0016),
    KeyCode::BracketLeft => Some(0x0022),
    KeyCode::BracketRight => Some(0x001B),
    KeyCode::Comma => Some(0x003B),
    KeyCode::Digit0 => Some(0x0013),
    KeyCode::Digit1 => Some(0x000A),
    KeyCode::Digit2 => Some(0x000B),
    KeyCode::Digit3 => Some(0x000C),
    KeyCode::Digit4 => Some(0x000D),
    KeyCode::Digit5 => Some(0x000E),
    KeyCode::Digit6 => Some(0x000F),
    KeyCode::Digit7 => Some(0x0010),
    KeyCode::Digit8 => Some(0x0011),
    KeyCode::Digit9 => Some(0x0012),
    KeyCode::Equal => Some(0x0015),
    KeyCode::IntlBackslash => Some(0x005E),
    KeyCode::IntlRo => Some(0x0061),
    KeyCode::IntlYen => Some(0x0084),
    KeyCode::KeyA => Some(0x0026),
    KeyCode::KeyB => Some(0x0038),
    KeyCode::KeyC => Some(0x0036),
    KeyCode::KeyD => Some(0x0028),
    KeyCode::KeyE => Some(0x001A),
    KeyCode::KeyF => Some(0x0029),
    KeyCode::KeyG => Some(0x002A),
    KeyCode::KeyH => Some(0x002B),
    KeyCode::KeyI => Some(0x001F),
    KeyCode::KeyJ => Some(0x002C),
    KeyCode::KeyK => Some(0x002D),
    KeyCode::KeyL => Some(0x002E),
    KeyCode::KeyM => Some(0x002E),
    KeyCode::KeyN => Some(0x0039),
    KeyCode::KeyO => Some(0x0020),
    KeyCode::KeyP => Some(0x0021),
    KeyCode::KeyQ => Some(0x0018),
    KeyCode::KeyR => Some(0x001B),
    KeyCode::KeyS => Some(0x0027),
    KeyCode::KeyT => Some(0x001C),
    KeyCode::KeyU => Some(0x001E),
    KeyCode::KeyV => Some(0x0037),
    KeyCode::KeyW => Some(0x0019),
    KeyCode::KeyX => Some(0x0035),
    KeyCode::KeyY => Some(0x001D),
    KeyCode::KeyZ => Some(0x0034),
    KeyCode::Minus => Some(0x0014),
    KeyCode::Period => Some(0x003C),
    KeyCode::Quote => Some(0x0030),
    KeyCode::Semicolon => Some(0x002F),
    KeyCode::Slash => Some(0x003D),
    KeyCode::AltLeft => Some(0x0040),
    KeyCode::AltRight => Some(0x006C),
    KeyCode::CapsLock => Some(0x0042),
    KeyCode::ContextMenu => Some(0x0087),
    KeyCode::ControlLeft => Some(0x0025),
    KeyCode::ControlRight => Some(0x0069),
    KeyCode::Enter => Some(0x0024),
    KeyCode::SuperLeft => Some(0x0085),
    KeyCode::SuperRight => Some(0x0086),
    KeyCode::ShiftLeft => Some(0x0032),
    KeyCode::ShiftRight => Some(0x003E),
    KeyCode::Space => Some(0x0041),
    KeyCode::Tab => Some(0x0017),
    KeyCode::Convert => Some(0x0064),
    KeyCode::Lang1 => Some(0x0082),
    KeyCode::Lang2 => Some(0x0083),
    KeyCode::KanaMode => Some(0x0065),
    KeyCode::NonConvert => Some(0x0066),
    KeyCode::Delete => Some(0x0077),
    KeyCode::End => Some(0x0073),
    KeyCode::Home => Some(0x006E),
    KeyCode::Insert => Some(0x0076),
    KeyCode::PageDown => Some(0x0075),
    KeyCode::PageUp => Some(0x0070),
    KeyCode::ArrowDown => Some(0x0074),
    KeyCode::ArrowLeft => Some(0x0071),
    KeyCode::ArrowRight => Some(0x0072),
    KeyCode::ArrowUp => Some(0x006F),
    KeyCode::NumLock => Some(0x004D),
    KeyCode::Numpad0 => Some(0x005A),
    KeyCode::Numpad1 => Some(0x0057),
    KeyCode::Numpad2 => Some(0x0058),
    KeyCode::Numpad3 => Some(0x0059),
    KeyCode::Numpad4 => Some(0x0058),
    KeyCode::Numpad5 => Some(0x0053),
    KeyCode::Numpad6 => Some(0x0054),
    KeyCode::Numpad7 => Some(0x0055),
    KeyCode::Numpad8 => Some(0x0050),
    KeyCode::Numpad9 => Some(0x0051),
    KeyCode::NumpadAdd => Some(0x0056),
    KeyCode::NumpadComma => Some(0x0081),
    KeyCode::NumpadDecimal => Some(0x005B),
    KeyCode::NumpadDivide => Some(0x006A),
    KeyCode::NumpadEnter => Some(0x0068),
    KeyCode::NumpadEqual => Some(0x007D),
    KeyCode::NumpadMultiply => Some(0x003F),
    KeyCode::NumpadSubtract => Some(0x0052),
    KeyCode::Escape => Some(0x0009),
    KeyCode::F1 => Some(0x0043),
    KeyCode::F2 => Some(0x0044),
    KeyCode::F3 => Some(0x0045),
    KeyCode::F4 => Some(0x0046),
    KeyCode::F5 => Some(0x0047),
    KeyCode::F6 => Some(0x0048),
    KeyCode::F7 => Some(0x0049),
    KeyCode::F8 => Some(0x004A),
    KeyCode::F9 => Some(0x004B),
    KeyCode::F10 => Some(0x004C),
    KeyCode::F11 => Some(0x005F),
    KeyCode::F12 => Some(0x0060),
    KeyCode::PrintScreen => Some(0x006B),
    KeyCode::ScrollLock => Some(0x004E),
    KeyCode::Pause => Some(0x007F),
    KeyCode::BrowserBack => Some(0x00A6),
    KeyCode::BrowserFavorites => Some(0x00A4),
    KeyCode::BrowserForward => Some(0x00A7),
    KeyCode::BrowserHome => Some(0x00B4),
    KeyCode::BrowserRefresh => Some(0x00B5),
    KeyCode::BrowserSearch => Some(0x00E1),
    KeyCode::BrowserStop => Some(0x0088),

    KeyCode::LaunchApp1 => Some(0x0098),
    KeyCode::LaunchApp2 => Some(0x0094),
    KeyCode::LaunchMail => Some(0x00A3),
    KeyCode::MediaPlayPause => Some(0x00AC),
    KeyCode::MediaSelect => Some(0x00B3),
    KeyCode::MediaStop => Some(0x00AE),
    KeyCode::MediaTrackNext => Some(0x00AB),
    KeyCode::MediaTrackPrevious => Some(0x00AD),
    KeyCode::AudioVolumeDown => Some(0x007A),
    KeyCode::AudioVolumeMute => Some(0x0079),
    KeyCode::AudioVolumeUp => Some(0x007B),
    KeyCode::Unidentified(NativeKeyCode::Gtk(scancode)) => Some(scancode as u32),
    _ => None,
  }
}

pub fn keycode_from_scancode(scancode: u32) -> KeyCode {
  // See: https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html
  // and: https://www.w3.org/TR/uievents-code/
  // and: The widget/NativeKeyToDOMCodeName.h file in the firefox source
  match scancode {
    0x0009 => KeyCode::Escape,
    0x000A => KeyCode::Digit1,
    0x000B => KeyCode::Digit2,
    0x000C => KeyCode::Digit3,
    0x000D => KeyCode::Digit4,
    0x000E => KeyCode::Digit5,
    0x000F => KeyCode::Digit6,
    0x0010 => KeyCode::Digit7,
    0x0011 => KeyCode::Digit8,
    0x0012 => KeyCode::Digit9,
    0x0013 => KeyCode::Digit0,
    0x0014 => KeyCode::Minus,
    0x0015 => KeyCode::Equal,
    0x0016 => KeyCode::Backspace,
    0x0017 => KeyCode::Tab,
    0x0018 => KeyCode::KeyQ,
    0x0019 => KeyCode::KeyW,
    0x001A => KeyCode::KeyE,
    0x001B => KeyCode::KeyR,
    0x001C => KeyCode::KeyT,
    0x001D => KeyCode::KeyY,
    0x001E => KeyCode::KeyU,
    0x001F => KeyCode::KeyI,
    0x0020 => KeyCode::KeyO,
    0x0021 => KeyCode::KeyP,
    0x0022 => KeyCode::BracketLeft,
    0x0023 => KeyCode::BracketRight,
    0x0024 => KeyCode::Enter,
    0x0025 => KeyCode::ControlLeft,
    0x0026 => KeyCode::KeyA,
    0x0027 => KeyCode::KeyS,
    0x0028 => KeyCode::KeyD,
    0x0029 => KeyCode::KeyF,
    0x002A => KeyCode::KeyG,
    0x002B => KeyCode::KeyH,
    0x002C => KeyCode::KeyJ,
    0x002D => KeyCode::KeyK,
    0x002E => KeyCode::KeyL,
    0x002F => KeyCode::Semicolon,
    0x0030 => KeyCode::Quote,
    0x0031 => KeyCode::Backquote,
    0x0032 => KeyCode::ShiftLeft,
    0x0033 => KeyCode::Backslash,
    0x0034 => KeyCode::KeyZ,
    0x0035 => KeyCode::KeyX,
    0x0036 => KeyCode::KeyC,
    0x0037 => KeyCode::KeyV,
    0x0038 => KeyCode::KeyB,
    0x0039 => KeyCode::KeyN,
    0x003A => KeyCode::KeyM,
    0x003B => KeyCode::Comma,
    0x003C => KeyCode::Period,
    0x003D => KeyCode::Slash,
    0x003E => KeyCode::ShiftRight,
    0x003F => KeyCode::NumpadMultiply,
    0x0040 => KeyCode::AltLeft,
    0x0041 => KeyCode::Space,
    0x0042 => KeyCode::CapsLock,
    0x0043 => KeyCode::F1,
    0x0044 => KeyCode::F2,
    0x0045 => KeyCode::F3,
    0x0046 => KeyCode::F4,
    0x0047 => KeyCode::F5,
    0x0048 => KeyCode::F6,
    0x0049 => KeyCode::F7,
    0x004A => KeyCode::F8,
    0x004B => KeyCode::F9,
    0x004C => KeyCode::F10,
    0x004D => KeyCode::NumLock,
    0x004E => KeyCode::ScrollLock,
    0x004F => KeyCode::Numpad7,
    0x0050 => KeyCode::Numpad8,
    0x0051 => KeyCode::Numpad9,
    0x0052 => KeyCode::NumpadSubtract,
    0x0053 => KeyCode::Numpad4,
    0x0054 => KeyCode::Numpad5,
    0x0055 => KeyCode::Numpad6,
    0x0056 => KeyCode::NumpadAdd,
    0x0057 => KeyCode::Numpad1,
    0x0058 => KeyCode::Numpad2,
    0x0059 => KeyCode::Numpad3,
    0x005A => KeyCode::Numpad0,
    0x005B => KeyCode::NumpadDecimal,
    0x005E => KeyCode::IntlBackslash,
    0x005F => KeyCode::F11,
    0x0060 => KeyCode::F12,
    0x0061 => KeyCode::IntlRo,
    0x0064 => KeyCode::Convert,
    0x0065 => KeyCode::KanaMode,
    0x0066 => KeyCode::NonConvert,
    0x0068 => KeyCode::NumpadEnter,
    0x0069 => KeyCode::ControlRight,
    0x006A => KeyCode::NumpadDivide,
    0x006B => KeyCode::PrintScreen,
    0x006C => KeyCode::AltRight,
    0x006E => KeyCode::Home,
    0x006F => KeyCode::ArrowUp,
    0x0070 => KeyCode::PageUp,
    0x0071 => KeyCode::ArrowLeft,
    0x0072 => KeyCode::ArrowRight,
    0x0073 => KeyCode::End,
    0x0074 => KeyCode::ArrowDown,
    0x0075 => KeyCode::PageDown,
    0x0076 => KeyCode::Insert,
    0x0077 => KeyCode::Delete,
    0x0079 => KeyCode::AudioVolumeMute,
    0x007A => KeyCode::AudioVolumeDown,
    0x007B => KeyCode::AudioVolumeUp,
    0x007D => KeyCode::NumpadEqual,
    0x007F => KeyCode::Pause,
    0x0081 => KeyCode::NumpadComma,
    0x0082 => KeyCode::Lang1,
    0x0083 => KeyCode::Lang2,
    0x0084 => KeyCode::IntlYen,
    0x0085 => KeyCode::SuperLeft,
    0x0086 => KeyCode::SuperRight,
    0x0087 => KeyCode::ContextMenu,
    0x0088 => KeyCode::BrowserStop,
    0x0089 => KeyCode::Again,
    0x008A => KeyCode::Props,
    0x008B => KeyCode::Undo,
    0x008C => KeyCode::Select,
    0x008D => KeyCode::Copy,
    0x008E => KeyCode::Open,
    0x008F => KeyCode::Paste,
    0x0090 => KeyCode::Find,
    0x0091 => KeyCode::Cut,
    0x0092 => KeyCode::Help,
    0x0094 => KeyCode::LaunchApp2,
    0x0097 => KeyCode::WakeUp,
    0x0098 => KeyCode::LaunchApp1,
    // key to right of volume controls on T430s produces 0x9C
    // but no documentation of what it should map to :/
    0x00A3 => KeyCode::LaunchMail,
    0x00A4 => KeyCode::BrowserFavorites,
    0x00A6 => KeyCode::BrowserBack,
    0x00A7 => KeyCode::BrowserForward,
    0x00A9 => KeyCode::Eject,
    0x00AB => KeyCode::MediaTrackNext,
    0x00AC => KeyCode::MediaPlayPause,
    0x00AD => KeyCode::MediaTrackPrevious,
    0x00AE => KeyCode::MediaStop,
    0x00B3 => KeyCode::MediaSelect,
    0x00B4 => KeyCode::BrowserHome,
    0x00B5 => KeyCode::BrowserRefresh,
    0x00E1 => KeyCode::BrowserSearch,
    _ => KeyCode::Unidentified(NativeKeyCode::Gtk(scancode as u16)),
  }
}