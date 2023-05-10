#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

extern crate bbwad;

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

    let web_options = eframe::WebOptions::default();

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
    wad_index: usize,
    active_image: Option<[egui::TextureHandle; 4]>,
    textures: Vec<egui::TextureHandle>,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    wad_file: Option<bbwad::WadFile>,
    file_dialog: FileDialog,
    upload_menu: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            wad_index: 0,
            active_image: None,
            textures: Vec::<egui::TextureHandle>::new(),
            dropped_files: Vec::<egui::DroppedFile>::new(),
            picked_path: None,
            wad_file: None,
            file_dialog: Default::default(),
            upload_menu: false,
        }
    }
}

impl MyApp {
    fn update_active_image(&mut self, ui: &egui::Ui) {
        match &self.wad_file {
            Some(wad_file) =>  {
                let wad_texture = &wad_file.entries[self.wad_index].texture;
                self.active_image.replace(
                    [
                        ui.ctx().load_texture(
                            "my-image", 
                            egui::ColorImage::from_rgb(
                                [wad_texture.header.n_width as usize, wad_texture.header.n_height as usize], 
                                &wad_texture.to_rgb_image_vec(bbwad::MIPMAP_LEVEL::LEVEL0)),
                            Default::default()),
                        ui.ctx().load_texture(
                            "my-image", 
                            egui::ColorImage::from_rgb(
                                [(wad_texture.header.n_width/2) as usize, (wad_texture.header.n_height/2) as usize], 
                                &wad_texture.to_rgb_image_vec(bbwad::MIPMAP_LEVEL::LEVEL1)),
                            Default::default()),
                        ui.ctx().load_texture(
                            "my-image", 
                            egui::ColorImage::from_rgb(
                                [(wad_texture.header.n_width/4) as usize, (wad_texture.header.n_height/4) as usize], 
                                &wad_texture.to_rgb_image_vec(bbwad::MIPMAP_LEVEL::LEVEL2)),
                            Default::default()),
                        ui.ctx().load_texture(
                            "my-image", 
                            egui::ColorImage::from_rgb(
                                [(wad_texture.header.n_width/8) as usize, (wad_texture.header.n_height/8) as usize], 
                                &wad_texture.to_rgb_image_vec(bbwad::MIPMAP_LEVEL::LEVEL3)),
                            Default::default())
                    ]
                );
            },
            None => (),
        }
    }

    fn fill_textures_vector(&mut self, ui: &egui::Ui) {
        match &self.wad_file {
            Some(wad_file) => {
                self.textures.clear();
                for entry in wad_file.entries.iter() {
                    let texture = ui.ctx().load_texture(
                        "my-image", 
                        egui::ColorImage::from_rgb(
                            [entry.texture.header.n_width as usize, entry.texture.header.n_height as usize], 
                            &entry.texture.to_rgb_image_vec(bbwad::MIPMAP_LEVEL::LEVEL0)),
                        Default::default());
                        self.textures.push(texture);
                }
            },
            None => (),
        }
    }

    fn get_entry_info_str(&self) -> Option<String> {
        match &self.wad_file {
            Some(wad_file) => {
                let dir_entry = &wad_file.entries[self.wad_index].dir_entry;
                match dir_entry.name_str() {
                    Some(name) => {
                        Some(
                            format!(
                                "name {:?}\nsize {:?}\ncompressed {:?}\n", 
                                name, dir_entry.n_disk_size, dir_entry.b_compression))
                    },
                    None => None,
                }
            },
            None => None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut update_active_image = false;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wad Viewer");

            ui.horizontal(|ui| {
                if ui.button("Prev").clicked() {
                    self.wad_index -= 1;
                    update_active_image = true;
                }
                if ui.button("Next").clicked() {
                    self.wad_index += 1;
                    update_active_image = true;
                }
            });

            ui.horizontal(|ui| {
                match &self.active_image {
                    Some(image) => {
                        ui.horizontal(|ui| {
                            ui.set_height(256.);
                            ui.set_width(512.);
                            for texture in image {
                                ui.add(egui::Image::new(texture, texture.size_vec2()));
                            }
                        });
                    },
                    None => {
                        let texture: &egui::TextureHandle = &ui.ctx().load_texture(
                            "my-image", 
                            egui::ColorImage::example(),
                            Default::default());
                        ui.add_sized([300., 300.], egui::Image::new(texture, texture.size_vec2()));
                    },
                }
                match self.get_entry_info_str() {
                    Some(mut label) => {
                        ui.add(
                             egui::TextEdit::multiline(&mut label)
                             .interactive(false)
                             .frame(false));
                    },
                    None => (),
                }
            });

            if ui.button("regen").clicked() {
                self.wad_file.as_mut().unwrap().regenerate();
            }

        });
        egui::TopBottomPanel::bottom("bottom")
            .resizable(true)
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(64, 64, 128)))
            .show(ctx, |ui| {
            egui::ScrollArea::horizontal()
                .id_source("second")
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for (itr, texture) in self.textures.iter().enumerate() {
                            let response = ui.add(egui::ImageButton::new(texture, texture.size_vec2()));
                            if response.clicked() {
                                self.wad_index = itr;
                                update_active_image = true;
                            }
                        }
                    });
                });
            if update_active_image {
                self.update_active_image(&ui);
            }
        });
        egui::SidePanel::right("side_panel_right").show(ctx, |ui| {
            if ui.button("Upload File").clicked() {
                self.file_dialog.open(); 
            }

            if let Some(file) = self.file_dialog.get() {
                self.wad_file.replace(bbwad::WadFile::from_bytes(&file));
                self.fill_textures_vector(&ui); 
                self.update_active_image(&ui);
                update_active_image = true;
            }
        });
    }
}
