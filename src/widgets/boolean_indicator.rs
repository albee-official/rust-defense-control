use eframe::egui::{Color32, Widget};

pub struct BooleanIndicator<'a> {
    pub label: String,
    pub value_ref: &'a bool,
}

impl Widget for BooleanIndicator<'_> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        ui.horizontal(|ui| {
            let label_response = ui.label(self.label);

            let status_response = if *self.value_ref {
                ui.colored_label(Color32::LIGHT_GREEN, "✅")
            } else {
                ui.colored_label(Color32::LIGHT_RED, "❌")
            };

            return label_response.union(status_response);
        })
        .inner
    }
}
