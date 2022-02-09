fn wait_for_input() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}

fn main() {
    match std::env::var("LOCALAPPDATA") {
        Ok(v) =>{
            let mut path = v.to_owned();
            path.push_str("\\Roblox\\GlobalBasicSettings_13.xml");

            let exists = std::path::Path::new(&path).exists();

            if exists == true {
                match std::fs::remove_file(&path) {
                    Ok(_) => {
                        println!("Successfully Removed file, the error should be fixed now!");
                        println!("Press Enter to exit!");
                        wait_for_input();
                    },
                    Err(e) => {
                        println!("Failed to remove file!\nTested File: {}\nError: {}", &path, e);
                        println!("Press Enter to exit!");
                        wait_for_input();
                    }
                }
            } else {
                println!("File doesn't exist!\nTested File: {}", &path);
                println!("Press Enter to exit!");
                wait_for_input();
            }
        },
        Err(e) => {
            println!("Exited with error: {}", e);
            println!("Press Enter to exit!");
            wait_for_input();
        }
    }
}
