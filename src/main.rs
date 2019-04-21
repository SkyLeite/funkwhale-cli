extern crate structopt;
extern crate reqwest;
extern crate dialoguer;
extern crate spinners;
extern crate chrono;

use structopt::StructOpt;

mod upload;

#[derive(StructOpt, Debug)]
#[structopt(name = "funkwhale-cli")]
enum Opt {
    #[structopt(name = "upload")]
    Upload {
        #[structopt(short = "i", long = "interactive")]
        interactive: bool,

        #[structopt(short = "f", long = "files", parse(from_os_str))]
        files: Vec<std::path::PathBuf>,

        #[structopt(short = "l", long = "library")]
        library: Option<String>,

        #[structopt(short = "u", long = "instance-url")]
        instance_url: String,

        #[structopt(short = "t", long = "token-file", parse(from_os_str))]
        token_file: std::path::PathBuf
    },
}

fn parse_token_file(path: std::path::PathBuf) -> String {
    let file = std::fs::read_to_string(path).expect("Could not read the token file :(");
    return file.trim().to_string();
}

fn main() {
    let args = Opt::from_args();

    if let Opt::Upload { interactive, files, library, instance_url, token_file } = args {
        let token = parse_token_file(token_file);
        match upload::main(files, library, instance_url, token, interactive) {
            Ok(v) => println!("\nUpload successful!"),
            Err(e) => panic!("{}", e),
        }
    }
}
