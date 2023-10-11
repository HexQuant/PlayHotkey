#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use inputbot::{KeybdKey, MouseButton};
use inputbot::{KeybdKey::*, MouseButton::*};
use std::{sync::Mutex, thread::sleep, time::Duration};

fn hook_inputs() {
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

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut a = false;
        let mut selected = 1;
        let mut ttext = String::new();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::new(&mut selected, "Game")
                .selected_text(format!("{:?}", selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected, 1, "Cyberpunk 2077");
                    ui.selectable_value(&mut selected, 2, "Elden Ring");
                });
            //ui.heading("Cyberpunk 2077");
            //ui.image(egui::include_image!("Cyberpunk_2077_logo.svg"));
            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(3)
                    .spacing([40.0, 12.0])
                    //.striped(true)
                    .show(ui, |ui| {
                        ui.label("Command");
                        ui.label("Ingame");
                        ui.label("Trigger");
                        ui.end_row();
                        //self.gallery_grid_contents(ui);
                        ui.checkbox(&mut a, "Aim toggle");
                        ui.button("33");
                        ui.button("33");
                        ui.end_row();
                        ui.checkbox(&mut a, "Dash and run");
                        ui.button("33");
                        ui.button("33");
                        ui.end_row();
                        ui.checkbox(&mut a, "Hide weapon");
                        ui.button("33");
                        ui.button("33");
                        ui.end_row();
                        ui.checkbox(&mut a, "Quick scripts");
                        ui.button("33");
                        ui.end_row();
                        ui.label("Script set #1");
                        ui.text_edit_singleline(&mut ttext);
                        ui.button("33");
                        ui.end_row();
                        ui.label("Script set #2");
                        ui.text_edit_singleline(&mut ttext);
                        ui.button("33");
                        ui.end_row();
                        ui.label("Script set #3");
                        ui.text_edit_singleline(&mut ttext);
                        if ui.button("33").clicked() {
                            println!("{}", ttext.clone());
                            KeybdKey::bind_all(|event| {
                                match inputbot::from_keybd_key(event) {
                                    Some(c) => println!("{c}"),
                                    None => println!("Unregistered Key"),
                                };
                            });
                            inputbot::handle_input_events();
                        }
                    });
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    println!("Hello, PlayHotkey!");
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
    //hook_inputs();
}
