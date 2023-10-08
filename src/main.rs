use inputbot::{KeybdKey::*, MouseButton::*};
use std::{sync::Mutex, thread::sleep, time::Duration};

fn main() {
    println!("Hello, PlayHotkey!");

    // aim
    let ingame_aim_button = KKey;
    let trigger_aim_button = RightButton;

    // dash
    let ingame_dash_button = LKey;
    //let ingame_run_button = LShiftKey;
    let trigger_run_button = LShiftKey;

    // hack
    let ingame_hack_button = NKey;
    let trigger_hack_button = LControlKey;

    // hide weapon
    let ingame_hide_button = XKey;
    let trigger_hide_button = RKey;

    // Aim toggle
    let aim_toggle = Mutex::new(false);
    trigger_aim_button.bind(move || {
        let mut toggle = aim_toggle.lock().unwrap();
        if *toggle {
            // LeftButton.press();
            // LeftButton.release();
            ingame_aim_button.release();
            *toggle = false;
            //sleep(Duration::from_millis(30));
        } else {
            ingame_aim_button.press();
            *toggle = true;
        }
    });

    // Dash
    trigger_run_button.bind(move || {
        ingame_dash_button.press();
        ingame_dash_button.release();
    });

    // Hide weapot
    trigger_hide_button.bind(move || {
        let mut i = 0;
        while trigger_hide_button.is_pressed() {
            sleep(Duration::from_millis(100));
            i += 1;
            if i > 10 {
                ingame_hide_button.press();
                ingame_hide_button.release();
                break;
            }
        }
    });

    // Hack
    trigger_hack_button.bind(move || {
        SKey.press();
        SKey.release();
        ingame_hack_button.press();
        ingame_hack_button.release();
        SKey.press();
        SKey.release();
        ingame_hack_button.press();
        ingame_hack_button.release();
    });

    inputbot::handle_input_events();
}
