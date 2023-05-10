#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

extern crate bbwad;

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
            upload_menu: false,
        }
    }
}

impl MyApp {
    fn detect_files_being_dropped(&mut self, ctx: &egui::Context) {
        use egui::*;
   
        let input = ctx.input(move |i| i.clone());
        //if ctx.input(move |i| i.raw.hovered_files.is_empty()) {
        if input.raw.hovered_files.is_empty() {

            let mut text = "Dropping files:\n".to_owned();
            let input = ctx.input(move |i| i.clone());
            for file in input.raw.hovered_files {
                if let Some(path) = &file.path {
                    text += &format!("\n{}", path.display());

                    self.picked_path = Some(path.as_path().to_string_lossy().to_string());
                } else if !file.mime.is_empty() {
                    text += &format!("\n{}", file.mime);
                } else {
                    text += "\n???";
                }
            }

            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

            let screen_rect = ctx.input(move |i| i.screen_rect());
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                egui::FontId::monospace(14.0),
                Color32::WHITE,
            );
        }

        // Collect dropped files:
        //if !ctx.input(move |i| i.raw.dropped_files.is_empty()) {
        if !input.raw.dropped_files.is_empty() {
            self.wad_file = None;
            self.dropped_files = input.raw.dropped_files.clone();
        }
    }

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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wad Viewer");
            let mut update_active_image = false;

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

            egui::ScrollArea::horizontal()
                .id_source("second")
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for (itr, texture) in self.textures.iter().enumerate() {
                            if ui.add(egui::ImageButton::new(texture, texture.size_vec2())).clicked() {
                                self.wad_index = itr;
                                update_active_image = true;
                            }
                        }
                    });
                });

            if ui.button("regen").clicked() {
                self.wad_file.as_mut().unwrap().regenerate();
            }

            if update_active_image {
                self.update_active_image(&ui);
            }

        });
        egui::SidePanel::right("side_panel_right").show(ctx, |ui| {
            if ui.button("Upload File").clicked() {
                self.upload_menu = true;
            }
                
            if self.upload_menu {
                self.detect_files_being_dropped(ctx);
                if let Some(picked_path) = &self.picked_path {
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(picked_path);
                    });
                }
                // Show dropped files (if any):
                if !self.dropped_files.is_empty() {
                    ui.group(|ui| {
                        ui.label("Dropped files:");

                        for file in &self.dropped_files {
                            let mut info = if let Some(path) = &file.path {
                                path.display().to_string()
                            } else if !file.name.is_empty() {
                                file.name.clone()
                            } else {
                                "???".to_owned()
                            };
                            if let Some(bytes) = &file.bytes {
                                info += &format!(" ({} bytes)", bytes.len());
                                if self.wad_file.is_none() {
                                    self.wad_file = Some(bbwad::WadFile::from_bytes(&bytes.to_vec()));
                                    self.upload_menu = false;
                                }
                            }
                            ui.label(info);
                        }
                    });
                }
                // assure to clean the dropped files list as soon as we have an image. Needed to reload a new, future image.
                if self.wad_file.is_some() {
                    self.fill_textures_vector(&ui); 
                    self.dropped_files.clear();
                }
            }
        });
    }
}
