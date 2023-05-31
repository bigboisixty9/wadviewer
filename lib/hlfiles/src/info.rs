//TODO: Move the widget trait above and make the files a separate directory and info and lib here I
//guess Idk.
use egui::widgets::text_edit::TextBuffer;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_BRANCH: &str = env!("GIT_BRANCH");
const GIT_COMMIT: &str = env!("GIT_COMMIT");
//println!("{}", last_git_commit::LastGitCommit::new().build().unwrap().id().long());
//println!("{}", VERSION);

pub struct DevInfoWindow {
    id: usize, 
}

impl DevInfoWindow {
    pub fn new(id: usize) -> Self {
        Self {
            id
        }
    }
}

impl super::HlFileWidget for DevInfoWindow {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use super::View as _;
        egui::Window::new("Information")
            .open(open)
            .scroll2([false, false])
            .id(egui::Id::new(self.id))
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for DevInfoWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let mut text = format!("cargo version: {}\ngit branch: {}\ngit commit: {}", VERSION, GIT_BRANCH, GIT_COMMIT);
            ui.text_edit_multiline(&mut text);
            if ui.button("📋Copy").clicked() {
                // this should work but does not
                ui.output_mut(|o| o.copied_text = "cum bum".to_string());
            }
        });
    }
}
