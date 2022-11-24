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
            return (version.to_string(), vec![]);
        }
    }
    return ("".to_string(), vec![]);
}

#[cfg(target_os = "windows")]
pub fn play_sound() {
    open::that(path().0.to_string() + &"\\content\\sounds\\ouch.ogg").unwrap();
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

// Linux
#[cfg(target_os = "linux")]
pub fn path() -> (String, Vec<()>) {
    use std::fs;
    // list all the files in ~/.local/share/grapejuice/prefixes/player/drive_c/users/USERNAME/AppData/Local/Roblox/Versions
    let mut versions = vec![];
    for entry in fs::read_dir(
        "/home/".to_string()
            + &whoami::username()
            + "/.local/share/grapejuice/prefixes/player/drive_c/users/"
            + &whoami::username()
            + "/AppData/Local/Roblox/Versions",
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
        if fs::metadata(version.clone() + "/RobloxPlayerBeta.exe").is_ok() {
            return (version.to_string(), vec![]);
        }
    }
    return ("".to_string(), vec![]);
    // I DO NOT KNOW RUST
    // I DO NOT KNOW WHY THIS WORKS
}

#[cfg(target_os = "linux")]
pub fn play_sound() {
    open::that(path().0.to_string() + &"/content/sounds/ouch.ogg").unwrap();
}

#[cfg(target_os = "linux")]
pub fn replace_sound(friendly_name: String, sound: String) {
    println!("You chose {}", friendly_name);
    use std::{
        fs::{self, File},
        io::{self, Write},
        os::unix::prelude::PermissionsExt,
        process::Command,
    };
    // delete a file
    fs::remove_file(path().0.to_string() + "/content/sounds/ouch.ogg").unwrap();
    println!("[1/2] Deleted existing death sound");
    // copy a file
    fs::copy(
        "sounds/".to_string() + &sound + "/ouch.ogg",
        path().0.to_string() + "/content/sounds/ouch.ogg",
    )
    .unwrap();
    println!("[2/2] Copied {} sound", friendly_name);
    println!("✅ Done!");
    // ASK USER A Y/N QUESTION
    println!("Would you like to prevent Roblox from replacing your Oof sound? (y/n)");
    let mut autostart = String::new();
    io::stdin().read_line(&mut autostart).unwrap();
    if autostart.trim() == "y" {
        if !fs::metadata("/home/".to_string() + &whoami::username() + "/.config/systemd/user")
            .is_ok()
        {
            fs::create_dir_all(
                "/home/".to_string() + &whoami::username() + "/.config/systemd/user",
            )
            .unwrap();
        }

        if !fs::metadata("/home/".to_string() + &whoami::username() + "/.oof-is-back").is_ok() {
            fs::create_dir_all("/home/".to_string() + &whoami::username() + "/.oof-is-back")
                .unwrap();
        }

        // check if file exists
        if !fs::metadata(path().0.to_string() + "/content/sounds/.ouch").is_ok() {
            fs::File::create(path().0.to_string() + "/content/sounds/.ouch").unwrap();
        }

        if fs::metadata("/home/".to_string() + &whoami::username() + "/.oof-is-back/ouch.ogg")
            .is_ok()
        {
            // delete a file
            fs::remove_file("/home/".to_string() + &whoami::username() + "/.oof-is-back/ouch.ogg")
                .unwrap();
        }

        fs::copy(
            "sounds/".to_string() + &sound + "/ouch.ogg",
            "/home/".to_string() + &whoami::username() + "/.oof-is-back/ouch.ogg",
        )
        .unwrap();

        if !fs::metadata(
            "/home/".to_string() + &whoami::username() + &"/.oof-is-back/oof-is-back2".to_string(),
        )
        .is_ok()
        {
            let mut resp = reqwest::blocking::get(
                "https://github.com/SegoGithub/oof-is-back-autostart/releases/download/v2.1.0/oof-is-back2",
            )
            .unwrap();
            let mut out = File::create(
                "/home/".to_string()
                    + &whoami::username()
                    + &"/.oof-is-back/oof-is-back2".to_string(),
            )
            .unwrap();
            io::copy(&mut resp, &mut out).unwrap();
            // make the file executable
            fs::set_permissions(
                "/home/".to_string()
                    + &whoami::username()
                    + &"/.oof-is-back/oof-is-back2".to_string(),
                fs::Permissions::from_mode(0o755),
            )
            .unwrap();
        };
        if !fs::metadata("/usr/bin/oof-is-back/icon.png".to_string()).is_ok() {
            let mut resp = reqwest::blocking::get(
                "https://github.com/SegoGithub/oof-is-back/raw/main/icon.png",
            )
            .unwrap();
            let mut out = File::create(
                "/home/".to_string() + &whoami::username() + &"/.oof-is-back/icon.png".to_string(),
            )
            .unwrap();
            io::copy(&mut resp, &mut out).unwrap();
        };
        if !fs::metadata(
            "/home/".to_string()
                + &whoami::username()
                + "/.config/systemd/user/oof-is-back.service",
        )
        .is_ok()
        {
            let mut file = File::create(
                "/home/".to_string()
                    + &whoami::username()
                    + "/.config/systemd/user/oof-is-back.service",
            )
            .unwrap();
            let systemd = "[Unit]\nDescription=oof-is-back, preventing Roblox from replacing your oof sound\nAfter=network.target\n\n[Service]\nType=simple\nExecStart=/home/".to_string() + &whoami::username().to_string() +"/.oof-is-back/oof-is-back2";
            // write systemd file
            file.write_all(systemd.as_bytes()).unwrap();
        };
        Command::new("systemctl")
            .arg("--user")
            .arg("enable")
            .arg("oof-is-back")
            .output()
            .expect("failed to run: systemctl --user enable oof-is-back");
        // start the oof-is-back service
        Command::new("systemctl")
            .arg("--user")
            .arg("start")
            .arg("oof-is-back")
            .output()
            .expect("failed to run: systemctl --user start oof-is-back");
        println!("You can disable this feature by running `systemctl --user disable oof-is-back`");
    };

    println!("✅ Done!");
    println!("Press any key to continue...");
    let mut autostart = String::new();
    io::stdin().read_line(&mut autostart).unwrap();
}

#[cfg(target_os = "linux")]
pub fn custom_sound() {
    println!("You chose a custom sound");
    use std::{
        fs::{self, File},
        io::{self, Write},
        os::unix::prelude::PermissionsExt,
        process::Command,
    };
    // create a folder in ./
    fs::create_dir_all("custom_sound").unwrap();
    println!("A file has been created in the current directory called custom_sound\nPlease put your custom sound in there and name it ouch.ogg");
    // ASK USER A Y/N QUESTION
    println!(
        "Have you put your OGG audio file in the custom_sound folder and named it ouch.ogg? (y/n)"
    );
    let mut custom = String::new();
    io::stdin().read_line(&mut custom).unwrap();

    // delete a file
    fs::remove_file(path().0 + "/content/sounds/ouch.ogg").unwrap();
    println!("[1/2] Deleted existing death sound");
    // copy a file
    fs::copy(
        "custom_sound/ouch.ogg",
        path().0 + "/content/sounds/ouch.ogg",
    )
    .unwrap();
    println!("[2/2] Copied custom sound");
    println!("✅ Done!");
    // ASK USER A Y/N QUESTION
    println!("Would you like to prevent Roblox from replacing your Oof sound? (y/n)");
    let mut autostart = String::new();
    io::stdin().read_line(&mut autostart).unwrap();
    if autostart.trim() == "y" {
        if !fs::metadata("/home/".to_string() + &whoami::username() + "/.config/systemd/user")
            .is_ok()
        {
            fs::create_dir_all(
                "/home/".to_string() + &whoami::username() + "/.config/systemd/user",
            )
            .unwrap();
        }

        if !fs::metadata("/home/".to_string() + &whoami::username() + "/.oof-is-back").is_ok() {
            fs::create_dir_all("/home/".to_string() + &whoami::username() + "/.oof-is-back")
                .unwrap();
        }

        // check if file exists
        if !fs::metadata(path().0.to_string() + "/content/sounds/.ouch").is_ok() {
            fs::File::create(path().0.to_string() + "/content/sounds/.ouch").unwrap();
        }

        if fs::metadata("/home/".to_string() + &whoami::username() + "/.oof-is-back/ouch.ogg")
            .is_ok()
        {
            // delete a file
            fs::remove_file("/home/".to_string() + &whoami::username() + "/.oof-is-back/ouch.ogg")
                .unwrap();
        }

        fs::copy(
            "custom_sound/ouch.ogg",
            "/home/".to_string() + &whoami::username() + "/.oof-is-back/ouch.ogg",
        )
        .unwrap();

        if !fs::metadata(
            "/home/".to_string() + &whoami::username() + &"/.oof-is-back/oof-is-back2".to_string(),
        )
        .is_ok()
        {
            let mut resp = reqwest::blocking::get(
                "https://github.com/SegoGithub/oof-is-back-autostart/releases/download/v2.1.0/oof-is-back2",
            )
            .unwrap();
            let mut out = File::create(
                "/home/".to_string()
                    + &whoami::username()
                    + &"/.oof-is-back/oof-is-back2".to_string(),
            )
            .unwrap();
            io::copy(&mut resp, &mut out).unwrap();
            // make the file executable
            fs::set_permissions(
                "/home/".to_string()
                    + &whoami::username()
                    + &"/.oof-is-back/oof-is-back2".to_string(),
                fs::Permissions::from_mode(0o755),
            )
            .unwrap();
        };
        if !fs::metadata("/usr/bin/oof-is-back/icon.png".to_string()).is_ok() {
            let mut resp = reqwest::blocking::get(
                "https://github.com/SegoGithub/oof-is-back/raw/main/icon.png",
            )
            .unwrap();
            let mut out = File::create(
                "/home/".to_string() + &whoami::username() + &"/.oof-is-back/icon.png".to_string(),
            )
            .unwrap();
            io::copy(&mut resp, &mut out).unwrap();
        };
        if !fs::metadata(
            "/home/".to_string()
                + &whoami::username()
                + "/.config/systemd/user/oof-is-back.service",
        )
        .is_ok()
        {
            let mut file = File::create(
                "/home/".to_string()
                    + &whoami::username()
                    + "/.config/systemd/user/oof-is-back.service",
            )
            .unwrap();
            let systemd = "[Unit]\nDescription=oof-is-back, preventing Roblox from replacing your oof sound\nAfter=network.target\n\n[Service]\nType=simple\nExecStart=/home/".to_string() + &whoami::username().to_string() +"/.oof-is-back/oof-is-back2";
            // write systemd file
            file.write_all(systemd.as_bytes()).unwrap();
        };
        Command::new("systemctl")
            .arg("--user")
            .arg("enable")
            .arg("oof-is-back")
            .output()
            .expect("failed to run: systemctl --user enable oof-is-back");
        // start the oof-is-back service
        Command::new("systemctl")
            .arg("--user")
            .arg("start")
            .arg("oof-is-back")
            .output()
            .expect("failed to run: systemctl --user start oof-is-back");
        println!("You can disable this feature by running `systemctl --user disable oof-is-back`");
    }
    println!("✅ Done!");
    println!("Press any key to continue...");
    let mut autostart = String::new();
    io::stdin().read_line(&mut autostart).unwrap();
}
