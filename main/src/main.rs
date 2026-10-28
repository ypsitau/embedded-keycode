//use std::fmt::Display as _;
use embedded_keycode as keycode;

fn print_keys(modifier: keycode::Modifier) {
    macro_rules! print_key_variants {
        ($modifier:expr; $($key:ident),* $(,)?) => {
            $(
                println!("{}", keycode::Key::$key($modifier.into()));
            )*
        };
    }
    print_key_variants!(modifier;
        Reserved, Esc, Num1, Num2, Num3, Num4, Num5, Num6,
        Num7, Num8, Num9, Num0, Minus, Equal, Backspace, Tab,
        Q, W, E, R, T, Y, U, I,
        O, P, LeftBrace, RightBrace, Enter, LeftCtrl, A, S,
        D, F, G, H, J, K, L, Semicolon,
        Apostrophe, Grave, LeftShift, Backslash, Z, X, C, V,
        B, N, M, Comma, Dot, Slash, RightShift, KpAsterisk,
        LeftAlt, Space, CapsLock, F1, F2, F3, F4, F5,
        F6, F7, F8, F9, F10, NumLock, ScrollLock, Kp7,
        Kp8, Kp9, KpMinus, Kp4, Kp5, Kp6, KpPlus, Kp1,
        Kp2, Kp3, Kp0, KpDot, ZenkakuHankaku, Nd102, F11, F12,
        Ro, Katakana, Hiragana, Henkan, KatakanaHiragana, Muhenkan, KpJpComma, KpEnter,
        RightCtrl, KpSlash, Sysrq, RightAlt, Linefeed, Home, Up, PageUp,
        Left, Right, End, Down, PageDown, Insert, Delete, Macro,
        Mute, VolumeDown, VolumeUp, Power, KpEqual, KpPlusMinus, Pause, Scale,
        KpComma, Hangeul, Hanguel, Hanja, Yen, LeftMeta, RightMeta, Compose,
        Stop, Again, Props, Undo, Front, Copy, Open, Paste,
        Find, Cut, Help, Menu, Calc, Setup, Sleep, WakeUp,
        File, SendFile, DeleteFile, Xfer, Prog1, Prog2, Www, Msdos,
        Coffee, ScreenLock, RotateDisplay, Direction, CycleWindows, Mail, Bookmarks, Computer,
        Back, Forward, CloseCd, EjectCd, EjectCloseCd, NextSong, PlayPause, PreviousSong,
        StopCd, Record, Rewind, Phone, Iso, Config, Homepage, Refresh,
        Exit, Move, Edit, ScrollUp, ScrollDown, KpLeftParen, KpRightParen, New,
        Redo, F13, F14, F15, F16, F17, F18, F19,
        F20, F21, F22, F23, F24, PlayCd, PauseCd, Prog3,
        Prog4, AllApplications, Dashboard, Suspend, Close, Play, FastForward, Bassboost,
        Print, Hp, Camera, Sound, Question, Email, Chat, Search,
        Connect, Finance, Sport, Shop, Alterase, Cancel, BrightnessDown, BrightnessUp,
        Media, Switchvideomode, KbdIllumToggle, KbdIllumDown, KbdIllumUp, Send, Reply, ForwardMail,
        Save, Documents, Battery, Bluetooth, Wlan, Uwb, Unknown, VideoNext,
        VideoPrev, BrightnessCycle, BrightnessAuto, BrightnessZero, DisplayOff, Wwan, Wimax, Rfkill,
        MicMute, Ok, Select, GoTo, Clear, Power2, Option, Info,
        Time, Vendor, Archive, Program, Channel, Favorites, Epg, Pvr,
        Mhp, Language, Title, Subtitle, Angle, FullScreen, Zoom, Mode,
        Keyboard, AspectRatio, Screen, Pc, Tv, Tv2, Vcr, Vcr2,
        Sat, Sat2, Cd, Tape, Radio, Tuner, Player, Text,
        Dvd, Aux, Mp3, Audio, Video, Directory, List, Memo,
        Calendar, Red, Green, Yellow, Blue, ChannelUp, ChannelDown, First,
        Last, Ab, Next, Restart, Slow, Shuffle, Break, Previous,
        Digits, Teen, Twen, Videophone, Games, ZoomIn, ZoomOut, ZoomReset,
        WordProcessor, Editor, Spreadsheet, GraphicsEditor, Presentation, Database, News, VoiceMail,
        AddressBook, Messenger, DisplayToggle, BrightnessToggle, Spellcheck, Logoff, Dollar, Euro,
        FrameBack, FrameForward, ContextMenu, MediaRepeat, TenChannelsUp, TenChannelsDown, Images, NotificationCenter,
        PickupPhone, HangupPhone, LinkPhone, DelEol, DelEos, InsLine, DelLine, Fn,
        FnEsc, FnF1, FnF2, FnF3, FnF4, FnF5, FnF6, FnF7,
        FnF8, FnF9, FnF10, FnF11, FnF12, Fn1, Fn2, FnD,
        FnE, FnF, FnS, FnB, FnRightShift, BrlDot1, BrlDot2, BrlDot3,
        BrlDot4, BrlDot5, BrlDot6, BrlDot7, BrlDot8, BrlDot9, BrlDot10, Numeric0,
        Numeric1, Numeric2, Numeric3, Numeric4, Numeric5, Numeric6, Numeric7, Numeric8,
        Numeric9, NumericStar, NumericPound, NumericA, NumericB, NumericC, NumericD, CameraFocus,
        WpsButton, TouchpadToggle, TouchpadOn, TouchpadOff, CameraZoomin, CameraZoomout, CameraUp, CameraDown,
        CameraLeft, CameraRight, AttendantOn, AttendantOff, AttendantToggle, LightsToggle, AlsToggle, RotateLockToggle,
        RefreshRateToggle, Buttonconfig, TaskManager, Journal, ControlPanel, AppSelect, ScreenSaver, VoiceCommand,
        Assistant, KbdLayoutNext, EmojiPicker, Dictate, CameraAccessEnable, CameraAccessDisable, CameraAccessToggle, Accessibility,
        DoNotDisturb, BrightnessMin, BrightnessMax, EprivacyScreenOn, EprivacyScreenOff, ActionOnSelection, ContextualInsert, ContextualQuery,
        KbdinputassistPrev, KbdinputassistNext, KbdinputassistPrevgroup, KbdinputassistNextgroup, KbdinputassistAccept, KbdinputassistCancel, RightUp, RightDown,
        LeftUp, LeftDown, RootMenu, MediaTopMenu, Numeric11, Numeric12, AudioDesc, Mode3d,
        NextFavorite, StopRecord, PauseRecord, Vod, Unmute, FastReverse, SlowReverse, Data,
        OnScreenKeyboard, PrivacyScreenToggle, SelectiveScreenshot, NextElement, PreviousElement, AutoPilotEngageToggle, MarkWayPoint, Sos,
        NavChart, FishingChart, SingleRangeRadar, DualRangeRadar, RadarOverlay, TraditionalSonar, ClearVuSonar, SideVuSonar,
        NavInfo, BrightnessMenu, Macro1, Macro2, Macro3, Macro4, Macro5, Macro6,
        Macro7, Macro8, Macro9, Macro10, Macro11, Macro12, Macro13, Macro14,
        Macro15, Macro16, Macro17, Macro18, Macro19, Macro20, Macro21, Macro22,
        Macro23, Macro24, Macro25, Macro26, Macro27, Macro28, Macro29, Macro30,
        MacroRecordStart, MacroRecordStop, MacroPresetCycle, MacroPreset1, MacroPreset2, MacroPreset3, KbdLcdMenu1, KbdLcdMenu2,
        KbdLcdMenu3, KbdLcdMenu4, KbdLcdMenu5, Performance, MinInteresting, Max,
    );
}

fn main() {
    let modifier = keycode::Modifier::default();
    println!("{}", keycode::Key::CookedChar('A'));
    println!("{}", keycode::Key::CookedChar('B'));
    println!("{}", keycode::Key::CookedChar('C'));
    println!("{}", keycode::Key::CookedChar('D'));
    println!("{}", keycode::Key::CookedCtrl(0x00));
    println!("{}", keycode::Key::CookedCtrl(0x01));
    println!("{}", keycode::Key::CookedCtrl(0x02));
    println!("{}", keycode::Key::CookedCtrl(0x03));
    println!("{}", keycode::Key::CookedCtrl(0x04));
    print_keys(modifier);
    print_keys(modifier.alt_l().shift_l().ctrl_l().meta_l());
}
