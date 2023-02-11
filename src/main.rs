use structopt::StructOpt;
use std::{fs::File, io, path::PathBuf};
use std::io::Read;
use std::time::Duration;

#[derive(Debug, StructOpt)]
#[structopt(name="login")]
struct Opt {
    /// your main nation or ns email address
    #[structopt(name="user", short, long)]
    user_agent: Option<String>,
    /// the number of requests per 30 seconds, maximum of 50
    #[structopt(short, long, default_value = "30")]
    speed: u8,
    /// the path of the file containing your nations
    #[structopt(short, long, default_value = "nations.txt")]
    path: PathBuf

}

/// accepts a non-empty input from the user (will probably only be used to set the user-agent)
fn get_input() -> String {
    let mut buffer = String::new();

    let mut valid_input = false;

    while !valid_input {
        println!("Please enter your user agent: ");

        if let Ok(_val) = io::stdin().read_line(&mut buffer) {
            buffer = buffer.trim().to_owned();

            if !buffer.is_empty() {
                valid_input = true;
            }
        }
    }

    buffer
}

fn open_file(path: &PathBuf) -> Result<String, io::Error> {
    let mut buffer = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn main() {
    let mut opt = Opt::from_args();

    // can't have a default value for the user-agent obviously, so if one is not set via
    // arguments, the user can input one when the script runs
    if let None = opt.user_agent {
        opt.user_agent = Some(get_input());
    }

    println!("{:#?}", opt);

    match open_file(&opt.path) {
        Ok(val) => println!("{:?}", val),
        Err(e) => println!("{:?}", e)
    };
}
