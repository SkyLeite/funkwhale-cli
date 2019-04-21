#[path = "./funkwhale.rs"]
mod funkwhale;

fn get_library(instance_url: &String, token: &String, library: Option<String>, interactive: bool) -> Result<String, Box<std::error::Error>> {
    let libraries = match funkwhale::get_libraries(&instance_url, &token) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let library_uuid: String;

    if let Some(new_library) = library {
        let found_library = libraries
            .into_iter()
            .find(|x| x.name.to_lowercase() == new_library.to_lowercase());

        if let Some(new_new_library) = found_library {
            library_uuid = new_new_library.uuid;
        } else {
            println!("No library found with that name");
            std::process::exit(1);
        }
    } else {
        if interactive == true {
            if libraries.len() == 0 {
                println!("You don't have libraries in your account");
                std::process::exit(1);
            }

            let selections = libraries
                .clone()
                .into_iter()
                .map(|x| format!("{}", x.name))
                .collect::<Vec<String>>();

            println!("Please select a library:");
            let selection = dialoguer::Select::new()
                .default(0)
                .items(&selections[..])
                .interact()
                .unwrap();

            library_uuid = libraries[selection].clone().uuid;
        } else {
            println!("No library specified and interactive mode disabled");
            std::process::exit(1);
        }
    }

    Ok(library_uuid)
}

pub fn main(files: Vec<std::path::PathBuf>, library: Option<String>, instance_url: String, token: String, interactive: bool) -> Result<(), Box<std::error::Error>> {
    let found_library = get_library(&instance_url, &token, library, interactive);

    match found_library {
        Ok(library) => funkwhale::upload(files, library, instance_url, token),
        Err(e) => return Err(e),
    }
}
