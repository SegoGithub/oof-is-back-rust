// WINDOWS ⊞

#[cfg(target_os = "windows")]
pub fn path() {
    use console::style;
    use std::{env, fs};
    static USERNAME: &str = "USERNAME";
    // list all the files in %localappdata%\Roblox\Versions
    let mut versions = vec![];
    for entry in fs::read_dir(
        "C:\\Users\\".to_string()
            + &env::var(USERNAME).unwrap()
            + "\\AppData\\Local\\Roblox\\Versions",
    )
    .unwrap()
    {
        versions.push(entry.unwrap().path().to_str().unwrap().to_string());
    }
    // sort the list of versions
    versions.sort();
    // reverse the list of versions
    versions.reverse();
    // print the list of versions
    for version in versions {
        // check if RobloxPlayerBeta.exe exists
        if fs::metadata(version.clone() + "\\RobloxPlayerBeta.exe").is_ok() {
            println!(
                "{} Roblox Path was detected as: {}",
                style("info").cyan(),
                version
            );
            return;
        }
    }
}

#[cfg(target_os = "windows")]
pub fn play_sound() {
    use std::{env, fs};

    static USERNAME: &str = "USERNAME";
    // list all the files in %localappdata%\Roblox\Versions
    let mut versions = vec![];
    for entry in fs::read_dir(
        "C:\\Users\\".to_string()
            + &env::var(USERNAME).unwrap()
            + "\\AppData\\Local\\Roblox\\Versions",
    )
    .unwrap()
    {
        versions.push(entry.unwrap().path().to_str().unwrap().to_string());
    }
    // sort the list of versions
    versions.sort();
    // reverse the list of versions
    versions.reverse();
    // print the list of versions
    for version in versions {
        // check if RobloxPlayerBeta.exe exists
        if fs::metadata(version.clone() + "\\RobloxPlayerBeta.exe").is_ok() {
            use rodio::{source::Source, Decoder, OutputStream};
            use std::fs::File;
            use std::io::BufReader;

            // Get a output stream handle to the default physical sound device
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(
                File::open(version.to_string() + &"\\content\\sounds\\ouch.ogg").unwrap(),
            );
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();
            // Play the sound directly on the device
            stream_handle.play_raw(source.convert_samples());

            // The sound plays in a separate audio thread,
            // so we need to keep the main thread alive while it's playing.
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }
}

#[cfg(target_os = "windows")]
pub fn replace_sound(friendly_name: String, sound: String) {
    println!("You chose {}", friendly_name);
    use std::{
        env,
        fs::{self, File},
        io,
    };
    static USERNAME: &str = "USERNAME";
    // list all the files in %localappdata%\Roblox\Versions
    let mut versions = vec![];
    for entry in fs::read_dir(
        "C:\\Users\\".to_string()
            + &env::var(USERNAME).unwrap()
            + "\\AppData\\Local\\Roblox\\Versions",
    )
    .unwrap()
    {
        versions.push(entry.unwrap().path().to_str().unwrap().to_string());
    }
    // sort the list of versions
    versions.sort();
    // reverse the list of versions
    versions.reverse();
    // print the list of versions
    for version in versions {
        // check if RobloxPlayerBeta.exe exists
        if fs::metadata(version.clone() + "\\RobloxPlayerBeta.exe").is_ok() {
            // delete a file
            fs::remove_file(version.clone() + "\\content\\sounds\\ouch.ogg").unwrap();
            println!("[1/2] Deleted existing death sound");
            // copy a file
            fs::copy(
                "sounds\\".to_string() + &sound + "\\ouch.ogg",
                version.clone() + "\\content\\sounds\\ouch.ogg",
            )
            .unwrap();
            println!("[2/2] Copied {} sound", friendly_name);
            println!("✅ Done!");
            // ASK USER A Y/N QUESTION
            println!("Would you like to prevent Roblox from replacing your Oof sound? (y/n)");
            let mut autostart = String::new();
            io::stdin().read_line(&mut autostart).unwrap();
            if autostart.trim() == "y" {
                fs::File::create(version.clone() + "/content/sounds/.ouch").unwrap();
                // copy a file
                // check if file exists
                if fs::metadata(
                    "C:\\Users\\".to_string()
                        + &env::var(USERNAME).unwrap()
                        + "\\AppData\\Roaming\\oof-is-back\\ouch.ogg",
                )
                .is_ok()
                {
                    // delete a file
                    fs::remove_file(
                        "C:\\Users\\".to_string()
                            + &env::var(USERNAME).unwrap()
                            + "\\AppData\\Roaming\\oof-is-back\\ouch.ogg",
                    )
                    .unwrap();
                }

                fs::copy(
                    "sounds\\".to_string() + &sound + "\\ouch.ogg",
                    "C:\\Users\\".to_string()
                        + &env::var(USERNAME).unwrap()
                        + "\\AppData\\Roaming\\oof-is-back\\ouch.ogg",
                );

                if !fs::metadata("C:\\Users\\".to_string()
                + &env::var("USERNAME").unwrap()
                + "\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\oof-is-back-autostart.exe").is_ok() {
                    let mut resp = reqwest::blocking::get(
                        "https://github.com/SegoGithub/oof-is-back-autostart/releases/download/v2.0.0/oof-is-back.exe",
                    )
                    .unwrap();
                    let mut out = File::create(
                        "C:\\Users\\".to_string()
                            + &env::var("USERNAME").unwrap()
                            + "\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\oof-is-back-autostart.exe",
                    )
                    .unwrap();
                    io::copy(&mut resp, &mut out).unwrap();
                }
                println!("✅ Done!");
                // wait 2 seconds
                std::thread::sleep(std::time::Duration::from_secs(1));
            } else {
                println!("✅ Done!");
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub fn custom_sound() {
    println!("You chose a custom sound");
    use std::{
        env,
        fs::{self, File},
        io,
    };
    static USERNAME: &str = "USERNAME";
    // list all the files in %localappdata%\Roblox\Versions
    let mut versions = vec![];
    for entry in fs::read_dir(
        "C:\\Users\\".to_string()
            + &env::var(USERNAME).unwrap()
            + "\\AppData\\Local\\Roblox\\Versions",
    )
    .unwrap()
    {
        versions.push(entry.unwrap().path().to_str().unwrap().to_string());
    }
    // sort the list of versions
    versions.sort();
    // reverse the list of versions
    versions.reverse();
    // print the list of versions
    for version in versions {
        // check if RobloxPlayerBeta.exe exists
        if fs::metadata(version.clone() + "\\RobloxPlayerBeta.exe").is_ok() {
            // create a folder in ./
            fs::create_dir_all("custom_sound").unwrap();
            println!("A file has been created in the current directory called custom_sound\nPlease put your custom sound in there and name it ouch.ogg");
            // ASK USER A Y/N QUESTION
            println!("Have you put your OGG audio file in the custom_sound folder and named it ouch.ogg? (y/n)");
            let mut custom = String::new();
            io::stdin().read_line(&mut custom).unwrap();

            // delete a file
            fs::remove_file(version.clone() + "\\content\\sounds\\ouch.ogg").unwrap();
            println!("[1/2] Deleted existing death sound");
            // copy a file
            fs::copy(
                "custom_sound/ouch.ogg",
                version.clone() + "\\content\\sounds\\ouch.ogg",
            )
            .unwrap();
            println!("[2/2] Copied custom sound");
            println!("✅ Done!");
            // ASK USER A Y/N QUESTION
            println!("Would you like to prevent Roblox from replacing your Oof sound? (y/n)");
            let mut autostart = String::new();
            io::stdin().read_line(&mut autostart).unwrap();
            if autostart.trim() == "y" {
                fs::File::create(version.clone() + "/content/sounds/.ouch").unwrap();
                // copy a file
                // check if file exists
                if fs::metadata(
                    "C:\\Users\\".to_string()
                        + &env::var(USERNAME).unwrap()
                        + "\\AppData\\Roaming\\oof-is-back\\ouch.ogg",
                )
                .is_ok()
                {
                    // delete a file
                    fs::remove_file(
                        "C:\\Users\\".to_string()
                            + &env::var(USERNAME).unwrap()
                            + "\\AppData\\Roaming\\oof-is-back\\ouch.ogg",
                    )
                    .unwrap();
                }
                if !fs::metadata("C:\\Users\\".to_string()
                + &env::var("USERNAME").unwrap()
                + "\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\oof-is-back-autostart.exe").is_ok() {
                    let mut resp = reqwest::blocking::get(
                        "https://github.com/SegoGithub/oof-is-back-autostart/releases/download/v2.0.0/oof-is-back.exe",
                    )
                    .unwrap();
                    let mut out = File::create(
                        "C:\\Users\\".to_string()
                            + &env::var("USERNAME").unwrap()
                            + "\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\oof-is-back-autostart.exe",
                    )
                    .unwrap();
                    io::copy(&mut resp, &mut out).unwrap();
                }
                fs::copy(
                    "custom_sound/ouch.ogg",
                    "C:\\Users\\".to_string()
                        + &env::var(USERNAME).unwrap()
                        + "\\AppData\\Roaming\\oof-is-back\\ouch.ogg",
                )
                .unwrap();
                println!("✅ Done!");
                std::thread::sleep(std::time::Duration::from_secs(1));
            } else {
                println!("✅ Done!");
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub fn fix_terminal() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}