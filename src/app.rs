use std::fmt::Display;
use rusqlite::{params, Connection, Result, types::Value};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Luigi {
    selection: std::collections::HashSet<usize>,
    reversed: bool,
    #[serde(skip)] // TODO should we serialize tables?
    db: Vec<Vec<Value>>,
}

struct Cell(Value);

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Text(t) => write!(f, "{}", t),
            Value::Real(r) => write!(f, "{}", r),
            Value::Blob(_) => write!(f, "BLOB"),
            Value::Null => write!(f, "NULL"),
        }
    }
}

impl Default for Luigi {
    fn default() -> Self {
        Self {
            selection: std::collections::HashSet::new(),
            reversed: false,
            db: dummy_table(),
        }
    }
}

fn dummy_table() -> Vec<Vec<Value>> {
    let mut table = vec![];
    for i in 0..100 {
        let row = vec![
            Value::Integer(i as i64),
            Value::Text(format!("Clipped text {}", i)),
            Value::Text(format!("Expanding content {}", i)),
        ];
        table.push(row);
    }
    table
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
                .body(|body| {
                    body.rows(text_height, num_rows, |mut row| {
                        let row_index = if self.reversed {
                            num_rows - 1 - row.index()
                        } else {
                            row.index()
                        };

                        row.set_selected(self.selection.contains(&row_index));

                        row.col(|ui| {
                            ui.label(format!("{:?}", self.db[row_index][0]));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:?}", self.db[row_index][1]));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:?}", self.db[row_index][2]));
                        });
                        self.toggle_row_selection(row_index, &row.response());
                    });
                });
        });
    }
}

#[derive(Debug)]
struct Person {
    id: rusqlite::types::Value,
    name: rusqlite::types::Value,
    data: rusqlite::types::Value,
}

fn try_rusq() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (),
    )?;

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        ("Remy", None::<Vec<u8>>),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        dbg!(person?);
    }
    Ok(())
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
