use std::sync::Arc;
use eframe::Frame;
use egui::{Context, Ui};

pub fn render_gui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximize_button(false)
            .with_resizable(false)
            .with_inner_size([1280., 720.]),
        ..Default::default()
    };

    eframe::run_native(
        "Ani-GUI",
        options,
        Box::new(|cc| {
            //visuals
            // let mut visuals = egui::Visuals::dark();
            //
            // visuals.panel_fill = egui::Color32::BLACK;
            // visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
            // visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
            //
            // cc.egui_ctx.set_visuals(visuals);

            //font
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert("font".to_owned(), Arc::new(egui::FontData::from_static(include_bytes!("assets/fonts/Rajdhani-Regular.ttf"))));
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "font".to_owned());
            cc.egui_ctx.set_fonts(fonts);


            Ok(Box::<AniGUI>::default())
        })
    )
}



struct AniGUI {
    open_setup_modal: bool,
    show_dock_tab: bool,
    open_about_model: bool,
    selected_tab: i8,
}


impl Default for AniGUI {
    fn default() -> Self {
        Self {
            open_setup_modal: false,
            show_dock_tab: false,
            open_about_model: false,
            selected_tab: 1,
        }
    }
}


impl eframe::App for AniGUI {
    // Main app loop!
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {

        egui::TopBottomPanel::top("Title").show(ctx, |ui: &mut Ui| {
            ui.vertical_centered(|ui: &mut Ui| {
                ui.label(egui::RichText::new("Ani-GUI").heading().size(55.));
            });

        });

        //modal logic
        if self.open_about_model {
            egui::Modal::new(egui::Id::new("About Menu")).show(ctx, |ui: &mut Ui| {
                ui.set_width(250.0);
                ui.vertical_centered(|ui: &mut Ui| {
                    ui.spacing_mut().button_padding = egui::vec2(10.10, 5.10);
                    ui.heading("About Ani-GUI");
                    ui.add_space(20.);
                    ui.label("Ani-GUI is a fork of the popular command line tool, 'ani-cli', which aims to allow you to watch anime without the hassle of going to sketchy, untrusted sites!");
                    ui.add_space(10.);
                    if ui.button("Ok").on_hover_text("This will close the about modal!").clicked() {
                        self.open_about_model = false;
                    }
                });

            });
        }

        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {

            // main shit????
            match self.selected_tab {
                1 => {
                    ui.vertical_centered(|ui: &mut Ui| {
                        ui.heading("Hey I am here!");
                    });
                }

                2 => {
                    ui.vertical_centered(|ui: &mut Ui| {
                        ui.heading("Hey I am here 2!");
                    });
                }

                3 => {
                    ui.vertical_centered(|ui: &mut Ui| {
                        ui.heading("Hey I am here 3!");
                    });
                }

                _ => {}
            }

            // dock logic
            let rect = egui::Rect::from_min_size(
                egui::pos2(480., 650.),
                egui::vec2(300., 50.)
            );

            ui.painter().rect_filled(
                rect,
                6.0,
                egui::Color32::from_rgb(53,56,57),
            );

            ui.allocate_ui_at_rect(rect, |ui: &mut Ui| {
                ui.horizontal_centered(|ui: &mut Ui| {
                    ui.spacing_mut().button_padding = egui::vec2(8., 12.);
                    ui.add_space(45.);
                    if ui.button("Tab 1").clicked() {
                        self.selected_tab = 1;
                    }
                    ui.add_space(30.);
                    if ui.button("Tab 2").clicked() {
                        self.selected_tab = 2;
                    }
                    ui.add_space(30.);
                    if ui.button("Tab 3").clicked() {
                        self.selected_tab = 3;
                    }
                });
            });
        });
    }
}