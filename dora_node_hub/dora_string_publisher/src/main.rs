use dora_node_api::DoraNode;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut node, mut _events) = DoraNode::init_from_env()?;

    let options = eframe::NativeOptions { ..Default::default() };

    // Run the native eframe application
    eframe::run_native(
        "Dora Text Publisher",
        options,
        Box::new(|cc| Ok(Box::new(dora_string_publisher::DoraUITextPublisher::new(cc, &mut node)))),
    )
    .map_err(|e| {
        eprintln!("Error running eframe application: {}", e);
        Box::new(e) as Box<dyn Error>
    })
}
