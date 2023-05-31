#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

extern crate hlfiles;

use hlfiles::hlmdl;
use hlfiles::hlwad;
use hlfiles::info;

mod file_dialog;
use crate::file_dialog::FileDialog;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let mut web_options = eframe::WebOptions::default();
    web_options.default_theme = eframe::Theme::Dark;

    web_sys::console::info(&js_sys::Array::from(&wasm_bindgen::JsValue::from_str("ass")));
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "bigwad", // hardcode it
            web_options,
            Box::new(|cc| Box::<MyApp>::default()),
        )
        .await
        .expect("failed to start eframe");
    })
}

struct MyApp {
    file_dialog: FileDialog,
    hl_file_widgets: Vec<Box<dyn hlfiles::HlFileWidget>>,
    id_incrementor: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file_dialog: Default::default(),
            hl_file_widgets: vec![],
            id_incrementor: 0,
        }
    }
}

impl MyApp {
    fn id_incrementor(&mut self) -> usize {
        self.id_incrementor = self.id_incrementor.checked_add(1).expect("Wow, that's a fuck ton of windows");
        self.id_incrementor
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for hl_file_widget in self.hl_file_widgets.iter_mut() {
            hl_file_widget.show(ctx, &mut true);
        }
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    if ui.button("Upload File").clicked() {
                        self.file_dialog.open(); 
                    }
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("Get Info").clicked() {
                        let id = self.id_incrementor();
                        self.hl_file_widgets.push(Box::new(info::DevInfoWindow::new(id)));
                    }
                });
            });
            if let Some(file) = self.file_dialog.get() {
                if hlwad::WadFile::validate_header(&file) {
                    let id = self.id_incrementor();
                    self.hl_file_widgets.push(Box::new(hlwad::WadFileWidget::from_bytes(&file, id)));
                }
                if hlmdl::MdlFile::validate_header(&file) {
                    let id = self.id_incrementor();
                    self.hl_file_widgets.push(Box::new(hlmdl::MdlFileWidget::from_bytes(&file, id)));
                }
            }
        });
    }
}
