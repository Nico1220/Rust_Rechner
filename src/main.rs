mod main_window;
use main_window::MainWindow;
use gtk4::{
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::WidgetExt,
};
use libadwaita::Application;
fn build_ui(application: &Application) {
    let window = MainWindow::new(application);
    window.show();
}
pub fn main() {
    let application = Application::new(
        Some("com.example.gtk-rss-reader"), Default::default()); 
    application.connect_activate(build_ui); 
    application.run();
}