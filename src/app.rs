#[derive(Default,serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DUI {
    #[serde(skip)] // TODO should we serialize tables?
    db: Vec<Vec<String>>,
}

impl DUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn load_file(&mut self, file: &egui::DroppedFile) {
        if let Some(path) = &file.path {
            if let Ok(file) = std::fs::read_to_string(path) {
                self.db = file
                    .lines()
                    .map(|line| line.split('\t').map(|s| s.to_string()).collect())
                    .collect();
            }
        }
    }
}

impl eframe::App for DUI {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.load_file(&i.raw.dropped_files[0]);
            }
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
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
            let table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .columns(Column::auto(), 3)
                .min_scrolled_height(0.0)
                .max_scroll_height(available_height)
                .sense(egui::Sense::click());

            let num_rows = self.db.len();

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Row");
                    });
                    header.col(|ui| {
                        ui.strong("Clipped text");
                    });
                    header.col(|ui| {
                        ui.strong("Expanding content");
                    });
                })
                .body(|body| {
                    body.rows(text_height, num_rows, |mut row| {
                        let row_index = row.index();

                        row.col(|ui| {
                            ui.label(format!("{:?}", self.db[row_index][0]));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:?}", self.db[row_index][1]));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:?}", self.db[row_index][2]));
                        });
                    });
                });
        });
    }
}
