//! Old startup preferences and explicit commands must never restore update prompts.
#[path = "common/mod.rs"]
mod common;

use egui_kittest::kittest::Queryable as _;
use gridcraft_engine::Session;
use serde_json::json;

#[test]
fn centrally_managed_updates_never_open_a_dialog() {
    let mut session = Session::new();
    session.new_workbook();
    let mut h = common::os_harness(session, egui::os::OperatingSystem::Windows);
    let app = h.state_mut();
    app.start_update_check();
    app.check_for_updates();
    app.start_pending_update();
    for id in ["app.checkForUpdates", "help.checkForUpdates"] {
        app.run(id, json!({})).unwrap();
    }
    h.run_steps(12);
    for label in ["Check for Updates", "Checking for updates", "Update and Restart", "is available"] {
        assert!(h.query_by_label_contains(label).is_none(), "unexpected update prompt: {label}");
    }
}
