// this app is very messy but it will be cleaned up soon

use serde_json::Value;

#[cfg(target_os = "windows")]
mod crossplatform;

fn main() {
    // fix for windows terminal closing
    #[cfg(target_os = "windows")]
    crossplatform::fix_terminal();

    // wait for user to enter a number
    crossplatform::path();
    fn main_menu() {
        console::Term::clear_screen(&console::Term::stdout());
        crossplatform::path();
        println!("Enter a number [1-4]:");
        println!("[1] ▶️ Play Current Death Sound\n[2] ☠️ Choose a New Death Sound\n[3] ⬆️ Check for Updates\n[4] ❌ Exit");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut input: u32 = input.trim().parse().expect("Please type a number!");
        handle_input(input)
    }
    main_menu();
    // if user enters 1, play current death sound
    fn handle_input(input: u32) {
        if input == 1 {
            println!("Playing current death sound...");
            crossplatform::play_sound();
            main_menu()
        } else if input == 2 {
            choose_sound()
        } else if input == 3 {
            println!("Checking for updates...");
            // get current version
            let current_version = env!("CARGO_PKG_VERSION");
            println!("Current version: {}", current_version);
            // get latest version from github api
            let client = reqwest::blocking::Client::new();
            let res = client
                .get("https://api.github.com/repos/SegoGithub/oof-is-back-rust/releases/latest")
                .header("User-Agent", "DeathSound")
                .send()
                .unwrap();
            let json: Value = serde_json::from_str(&res.text().unwrap()).unwrap();
            let latest_version = json["tag_name"].to_string();
            let latest_version = latest_version.replace("\"", "");
            println!("Latest version: {}", latest_version);
            // compare versions
            if current_version == latest_version {
                println!("You are up to date!\nPress enter to continue...");
                std::io::stdin()
                    .read_line(&mut String::new())
                    .expect("Failed to read line");
                main_menu()
            } else {
                println!("There is an update available!\nDownload it here: https://github.com/SegoGithub/oof-is-back-rust/releases/latest\nPress enter to continue...");
                std::io::stdin()
                    .read_line(&mut String::new())
                    .expect("Failed to read line");
                main_menu()
            }

        } else if input == 4 {
            println!("Exiting...");
            std::process::exit(0);
        } else {
            println!("Invalid input!\nReturning to main menu in 3 seconds...");
            std::thread::sleep(std::time::Duration::from_secs(3));
            main_menu()
        }
    }
    fn choose_sound() {
        console::Term::clear_screen(&console::Term::stdout());
        println!("Choose a new death sound\nEnter a number [1-9]:\n[1] Oof\n[2] Vine Boom Sound Effect (Bass Boosted)\n[3] GAH DAM\n[4] Half Life\n[5] Old Minecraft Death Sound\n[6] Lego Yoda Death\n[7] AUUUUUUGHHH\n[8] Custom Sound\n[9] Return to main menu");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut input: u32 = input.trim().parse().expect("Please type a number!");
        handle_sound_input(input)
    }
    fn handle_sound_input(input: u32) {
        if input == 1 {
            crossplatform::replace_sound("Oof".to_string(), "oof".to_string());
            println!("Returning to main menu in 2 seconds..");
            std::thread::sleep(std::time::Duration::from_secs(2));
            main_menu()
        } else if input == 2 {
            crossplatform::replace_sound("Vine Boom Sound Effect (Bass Boosted)".to_string(), "vineboom".to_string());
            println!("Returning to main menu in 2 seconds..");
            std::thread::sleep(std::time::Duration::from_secs(2));
            main_menu()
        } else if input == 3 {
            crossplatform::replace_sound("GAH DAM".to_string(), "gahdam".to_string());
            println!("Returning to main menu in 2 seconds..");
            std::thread::sleep(std::time::Duration::from_secs(2));
            main_menu()
        } else if input == 4 {
            crossplatform::replace_sound("Half Life".to_string(), "hl".to_string());
            println!("Returning to main menu in 2 seconds..");
            std::thread::sleep(std::time::Duration::from_secs(2));
            main_menu()
        } else if input == 5 {
            crossplatform::replace_sound("Old Minecraft Death Sound".to_string(), "mc".to_string());
            println!("Returning to main menu in 2 seconds..");
            std::thread::sleep(std::time::Duration::from_secs(2));
            main_menu()
        } else if input == 6 {
            crossplatform::replace_sound("Lego Yoda Death".to_string(), "yoda".to_string());
            println!("Returning to main menu in 2 seconds..");
            std::thread::sleep(std::time::Duration::from_secs(2));
            main_menu()
        } else if input == 7 {
            crossplatform::replace_sound("AUUUUUUGHHH".to_string(), "augh".to_string());
            println!("Returning to main menu in 2 seconds..");
            std::thread::sleep(std::time::Duration::from_secs(2));
            main_menu()
        } else if input == 8 {
            crossplatform::custom_sound();
            println!("Returning to main menu in 2 seconds..");
            std::thread::sleep(std::time::Duration::from_secs(2));
        } else if input == 9 {
            main_menu()
        } else {
            println!("Invalid input!\nReturning to main menu in 3 seconds...");
            std::thread::sleep(std::time::Duration::from_secs(3));
            main_menu()
        }
    }
}
