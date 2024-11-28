use std::process::Command;
slint::include_modules!(); // Import your Slint UI module

// This function simulates the shutdown process
fn shutdown_system(shutdown_time: u64) {
    println!("Shutting down in {} seconds...", shutdown_time);

    // Sleep for the shutdown time (optional, to delay shutdown)
    std::thread::sleep(std::time::Duration::from_secs(shutdown_time));

    // Execute the shutdown command based on the operating system
    if cfg!(target_os = "windows") {
        // Windows shutdown command
        Command::new("shutdown")
            .arg("/s")
            .arg("/f") // Force close applications
            .arg("/t")
            .arg(shutdown_time.to_string()) // Time in seconds
            .spawn()
            .expect("Failed to start shutdown command");
    } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        // Linux or macOS shutdown command
        Command::new("shutdown")
            .arg(format!("-h +{}", shutdown_time)) // Shutdown in shutdown_time minutes
            .spawn()
            .expect("Failed to start shutdown command");
    } else {
        println!("Unsupported OS for shutdown command.");
    }
}

fn main() {
    // Create the UI instance using the generated AppWindow component
    let ui = AppWindow::new().unwrap_or_else(|_| {
        panic!("Failed to create AppWindow");
    });

    // Handle the start_shutdown callback
    ui.on_start_shutdown({
        // Clone the UI reference into the closure
        let ui_clone = ui.as_weak();

        move || {
            if let Some(ui) = ui_clone.upgrade() {
                // Access the shutdown time from the UI property
                let shutdown_time_str = ui.get_shutdown_time();
                if let Ok(shutdown_time) = shutdown_time_str.parse::<u64>() {
                    // Call the shutdown system function with the provided time
                    shutdown_system(shutdown_time);
                } else {
                    println!("Invalid time input. Please enter a valid number.");
                }
            } else {
                println!("Failed to access UI.");
            }
        }
    });

    // Run the Slint UI loop
    ui.run().unwrap_or_else(|_| {
        panic!("Failed to run the application");
    });
}
