use crate::game_of_life_logic::Grid;

const GRID_SIZE: usize = 16usize;

// persist app on shutdown
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConwayApp {
    grid: Grid<GRID_SIZE,GRID_SIZE>,
}

impl Default for ConwayApp {
    fn default() -> Self {
        Self {
            grid: Grid::empty(),
        }
    }
}

impl ConwayApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for ConwayApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel")
            .resizable(false)
            .show(ctx, |ui| {
            ui.heading("Controls");
            if ui.button("Randomize").clicked() {
                self.grid = Grid::random();
            }
            if ui.button("Next").clicked() {
                self.grid = self.grid.clone().into_iter().next().unwrap();
            }

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
            let grid_display = egui::Label::new(self.grid.to_string())
                .wrap(false);
            ui.add(grid_display);
        });

    }
}
