mod hyprwire;

use crate::hyprwire::hyprpaper::HyprpaperIPC;

fn main() {
    match HyprpaperIPC::set_wallpaper("/path/to/wallpaper.jpg", "DP-2") {
        Ok(_) => println!("Complete."),
        Err(e) => println!("Error: {}", e),
    }
}
