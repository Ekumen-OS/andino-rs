use dora_node_api::{DoraNode, arrow::array::StringArray, dora_core::config::DataId};
use eframe::{App, Frame};

// Struct to hold the application's state
pub struct DoraUITextPublisher<'a> {
    input_value: String,
    submitted_text: String,
    dora_node: &'a mut DoraNode,
}

// Default implementation for DoraUITextPublisher to initialize the state
impl<'a> DoraUITextPublisher<'a> {
    pub fn new(_cc: &eframe::CreationContext<'_>, dora_node: &'a mut dora_node_api::DoraNode) -> Self {
        Self {
            input_value: String::new(),
            submitted_text: "...".to_owned(),
            dora_node,
        }
    }

    pub fn dora_send(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let parameters = dora_node_api::MetadataParameters::default();
        let string_array = StringArray::from(vec![self.submitted_text.clone()]);
        self.dora_node
            .send_output(DataId::from(String::from("submitted_string")), parameters, string_array)?;
        Ok(())
    }
}

// Implementation of the eframe App trait for our DoraUITextPublisher struct.
impl App for DoraUITextPublisher<'_> {
    // Update method to handle the UI rendering and input handling.
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            // Center the UI elements
            ui.vertical_centered(|ui| {
                // Add some vertical spacing
                ui.add_space(20.0);
                // Set a max width for the widgets
                ui.set_max_width(300.0);
                // Add a label for instruction
                ui.label("Type something and press Enter:");
                // Create a single-line text input field
                let text_edit = ui.text_edit_singleline(&mut self.input_value);
                // Check if the "Enter" key was pressed while the text input was focused
                if text_edit.lost_focus() && ui.input(|i| i.key_pressed(eframe::egui::Key::Enter)) {
                    // If the input is not empty, update the submitted text and clear the input
                    if !self.input_value.is_empty() {
                        self.submitted_text = self.input_value.clone();
                        println!("Input received: {}", self.submitted_text);
                        let res = self.dora_send();
                        if let Err(e) = res {
                            eprintln!("Error sending data to Dora output: {}", e);
                        }
                        self.input_value.clear();
                        // Request focus back to the text edit for a better user experience
                        text_edit.request_focus();
                    }
                }
                // Add some spacing
                ui.add_space(10.0);
                // Display the submitted text
                ui.label(format!("Last submitted: {}", self.submitted_text));
            });
        });
    }
}
