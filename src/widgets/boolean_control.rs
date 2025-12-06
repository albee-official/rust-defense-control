use eframe::egui::{Color32, Response, Widget};

use crate::data::{ControlledValue, ValueRef};

pub struct BooleanControl<'a, OnChange>
where
    OnChange: FnOnce(bool) -> (),
{
    pub label: String,
    pub value_ref: &'a mut ControlledValue<bool>,
    pub on_change: OnChange,
}

impl<OnChange> Widget for BooleanControl<'_, OnChange>
where
    OnChange: FnOnce(bool) -> (),
{
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        ui.horizontal(|ui| {
            let label_response = ui.label(self.label);

            let status_response: Response;
            match self.value_ref.edit_value() {
                ValueRef::Editable(value) => {
                    status_response = if *value {
                        ui.toggle_value(value, "✅")
                    } else {
                        ui.toggle_value(value, "❌")
                    };

                    if status_response.clicked() {
                        (self.on_change)(*value);
                    }
                }

                ValueRef::Readonly(value) => {
                    status_response = if *value {
                        ui.colored_label(Color32::LIGHT_GREEN, " ✅")
                    } else {
                        ui.colored_label(Color32::LIGHT_RED, " ❌")
                    };
                }
            };

            return label_response.union(status_response);
        })
        .inner
    }
}
