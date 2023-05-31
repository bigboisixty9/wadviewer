pub mod file_dialog;
pub mod hlwad;
pub mod hlmdl;
pub mod info;

#[macro_use]
extern crate bmp;

/// Something to view in the fileWidth windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// Something to view
pub trait HlFileWidget {
    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

pub trait GuiImage {
    fn to_rgb_image_vec(&self) -> Vec<u8>;
}

struct IdTool {
    cur_id: usize,
}

impl IdTool {
    fn get_id(&mut self) -> usize {
        self.cur_id.checked_add(1).expect("Idk how you did it, but you got 2^64 windows open. Good Job");
        self.cur_id
    }
}

