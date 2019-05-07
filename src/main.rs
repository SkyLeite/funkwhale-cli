use structopt::StructOpt;
use walkdir::WalkDir;

mod config;
mod funkwhale;
mod upload;

#[derive(StructOpt, Debug)]
#[structopt(name = "funkwhale-cli")]
enum Opt {
    #[structopt(name = "upload")]
    Upload {
        #[structopt(short = "i", long = "interactive")]
        interactive: bool,

        #[structopt(parse(from_os_str), help = "Files to be uploaded")]
        files: Vec<std::path::PathBuf>,

        #[structopt(short = "l", long = "library")]
        library: Option<String>,

        #[structopt(short = "m", long = "timeout", default_value = "500")]
        timeout: u64,

        #[structopt(short = "d", long = "depth", default_value = "5")]
        max_depth: u64,
    },
}

fn parse_files(files: Vec<std::path::PathBuf>, max_depth: u64) -> Vec<std::path::PathBuf> {
    if files.len() == 0 {
        println!("No files were supplied.");
        std::process::exit(1);
    }

    let mut all_files: Vec<std::path::PathBuf> = Vec::new();
    for file in files {
        if file.is_dir() {
            let mut files: Vec<std::path::PathBuf> = WalkDir::new(file)
                .follow_links(true)
                .max_depth(max_depth as usize)
                .into_iter()
                .map(|f| f.ok().unwrap().path().to_path_buf())
                .filter(|f| f.is_file())
                .collect();

            all_files.append(&mut files);
        } else {
            all_files.push(file);
        }
    }
    return all_files;
}

fn main() {
    let args = Opt::from_args();
    let config = config::get_config().unwrap();
    let token = config::get_token(&config.instance_url).unwrap();

    if let Opt::Upload {
        interactive,
        files,
        library,
        timeout,
        max_depth,
    } = args
    {
        let mut all_files = parse_files(files, max_depth);
        let allowed_extensions = funkwhale::get_nodeinfo(&config.instance_url)
            .unwrap()
            .metadata
            .supported_upload_extensions;

        match allowed_extensions {
            Some(extensions) => {
                all_files = all_files
                    .iter()
                    .cloned()
                    .filter(|file| match file.extension() {
                        Some(extension) => {
                            return extensions
                                .contains(&String::from(extension.to_str().expect("Noo!")));
                        }
                        None => return false,
                    })
                    .collect::<Vec<_>>();
            }
            None => {}
        }

        match upload::main(
            all_files,
            library,
            config.instance_url,
            token,
            interactive,
            timeout,
        ) {
            Ok(_v) => println!("\nUpload successful!"),
            Err(e) => panic!("{}", e),
        };
    }
}
