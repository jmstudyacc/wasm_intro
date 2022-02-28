use seed::window;

fn create_window() {
    let window = window();
    window
        .alert_with_message("Hello World! This will appear in a window in the browser :)")
        .expect("Error");
}
