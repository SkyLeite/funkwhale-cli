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

        #[structopt(short = "f", long = "file", parse(from_os_str))]
        file: std::path::PathBuf,

        #[structopt(short = "l", long = "library")]
        library: Option<String>,

        #[structopt(short = "m", long = "timeout", default_value = "500")]
        timeout: u64,

        #[structopt(short = "d", long = "depth", default_value = "5")]
        max_depth: u64,
    },
}

fn parse_file(file: std::path::PathBuf, max_depth: u64) -> Vec<std::path::PathBuf> {
    if !file.exists() {
        panic!("File or directory doesn't exist");
    }

    if file.is_dir() {
        let files: Vec<std::path::PathBuf> = WalkDir::new(file)
            .follow_links(true)
            .max_depth(max_depth as usize)
            .into_iter()
            .map(|f| f.ok().unwrap().path().to_path_buf())
            .filter(|f| f.is_file())
            .collect();

        return files;
    } else {
        return vec![file];
    }
}

fn main() {
    let args = Opt::from_args();
    let config = config::get_config().unwrap();
    let token = config::get_token(&config.instance_url).unwrap();

    if let Opt::Upload {
        interactive,
        file,
        library,
        timeout,
        max_depth,
    } = args
    {
        let all_files = parse_file(file, max_depth);
        let allowed_extensions = funkwhale::get_nodeinfo(&config.instance_url)
            .unwrap()
            .metadata
            .supported_upload_extensions;

        let filtered_files = all_files
            .iter()
            .cloned()
            .filter(|file| match file.extension() {
                Some(extension) => {
                    return allowed_extensions
                        .contains(&String::from(extension.to_str().expect("Noo!")));
                }
                None => return false,
            })
            .collect::<Vec<_>>();

        match upload::main(
            filtered_files,
            library,
            config.instance_url,
            token,
            interactive,
            timeout,
        ) {
            Ok(_v) => println!("\nUpload successful!"),
            Err(e) => panic!("{}", e),
        }
    }
}
