#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Luigi {
    selection: std::collections::HashSet<usize>,
    reversed: bool,
    num_rows: usize,
}

impl Default for Luigi {
    fn default() -> Self {
        Self {
            selection: std::collections::HashSet::new(),
            reversed: false,
            num_rows: 100,
        }
    }
}

impl Luigi {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Luigi {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            use egui_extras::{Column, TableBuilder};
            let text_height = egui::TextStyle::Body
                .resolve(ui.style())
                .size
                .max(ui.spacing().interact_size.y);

            let available_height = ui.available_height();
            let mut table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .columns(Column::auto(), 3)
                .min_scrolled_height(0.0)
                .max_scroll_height(available_height)
                .sense(egui::Sense::click());

                table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        egui::Sides::new().show(
                            ui,
                            |ui| {
                                ui.strong("Row");
                            },
                            |ui| {
                                self.reversed ^=
                                    ui.button(if self.reversed { "⬆" } else { "⬇" }).clicked();
                            },
                        );
                    });
                    header.col(|ui| {
                        ui.strong("Clipped text");
                    });
                    header.col(|ui| {
                        ui.strong("Expanding content");
                    });
                })
                .body(|mut body| {
                    body.rows(text_height, self.num_rows, |mut row| {
                        let row_index = if self.reversed {
                            self.num_rows - 1 - row.index()
                        } else {
                            row.index()
                        };

                        row.set_selected(self.selection.contains(&row_index));

                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                        row.col(|ui| {
                            ui.label(format!("Clipped text {}", row_index));
                        });
                        row.col(|ui| {
                            ui.add(egui::Separator::default().horizontal());
                        });
                        self.toggle_row_selection(row_index, &row.response());
                    });
                });
        });
    }

}

impl Luigi {
    fn toggle_row_selection(&mut self, row_index: usize, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selection.contains(&row_index) {
                self.selection.remove(&row_index);
            } else {
                self.selection.insert(row_index);
            }
        }
    }
}