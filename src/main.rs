use structopt::StructOpt;
use walkdir::WalkDir;

mod config;
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

        #[structopt(short = "t", long = "token-file", parse(from_os_str))]
        token_file: std::path::PathBuf,

        #[structopt(short = "m", long = "timeout", default_value = "500")]
        timeout: u64,

        #[structopt(short = "d", long = "depth", default_value = "5")]
        max_depth: u64,
    },
}

fn parse_token_file(path: std::path::PathBuf) -> String {
    let file = std::fs::read_to_string(path).expect("Could not read the token file :(");
    return file.trim().to_string();
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

    if let Opt::Upload {
        interactive,
        file,
        library,
        token_file,
        timeout,
        max_depth,
    } = args
    {
        let token = parse_token_file(token_file);
        let all_files = parse_file(file, max_depth);

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
        }
    }
}
