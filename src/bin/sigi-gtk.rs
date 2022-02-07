/// Run the GUI
fn main() {
    let exit_code = sigi_gtk::gui::run();
    std::process::exit(exit_code);
}
