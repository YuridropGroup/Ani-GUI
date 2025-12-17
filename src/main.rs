mod gui;
use crate::gui::gui_manager;


use std::sync::Arc;
use eframe::Frame;
use egui::{Context, Ui};

fn main(){
    gui_manager::render_gui();
}