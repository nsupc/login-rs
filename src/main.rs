use reqwest::Client;
use structopt::StructOpt;
use std::{fs::File, io, path::PathBuf};
use std::io::Read;
use std::time::{Instant, Duration};
use std::collections::VecDeque;
use tokio::sync::Semaphore;

const INTERVAL: Duration = Duration::from_secs(30);

#[derive(Debug, StructOpt)]
#[structopt(name="login")]
struct Opt {
    /// your main nation or ns email address
    #[structopt(name="user", short, long)]
    user_agent: Option<String>,
    /// the number of requests per 30 seconds, maximum of 45
    #[structopt(short, long, default_value = "30")]
    speed: usize,
    /// the location of the file containing your nations
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

async fn ratelimiter(limit: &usize, semaphore: &Semaphore, request_times: &mut VecDeque<Instant>, nation: &str, password: &str, client: &Client, user: &str) {
    let _permit = semaphore.acquire().await;

    while &request_times.len() >= limit {
        let elapsed = Instant::now().duration_since(request_times[0]);
        if elapsed >= INTERVAL {
            request_times.pop_front();
        } else {
            tokio::time::sleep(INTERVAL - elapsed).await;
        }
    }
    request_times.push_back(Instant::now());

    match request(client, user, nation, password).await {
        Ok(res) => {
            println!("Nation: {}, Status Code: {}", nation, res.status());
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

async fn request(client: &Client, user: &str, nation: &str, password: &str) -> Result<reqwest::Response, reqwest::Error> {
    let res = client
        .get(format!("https://www.nationstates.net/cgi-bin/api.cgi?nation={}&q=ping", nation))
        .header("User-Agent", format!("UPC's Login-rs, used by {}", user))
        .header("X-Password", password)
        .send()
        .await?;

    Ok(res)
}

#[tokio::main]
async fn main() {
    let mut opt = Opt::from_args();

    // can't have a default value for the user-agent obviously, so if one is not set via
    // arguments, the user can input one when the script runs
    if opt.user_agent.is_none() {
        opt.user_agent = Some(get_input());
    }

    // the maximum number of requests to ns that the script will make in 30 seconds
    let limit: usize = if opt.speed > 45 { 45 } else { opt.speed };

    // trying to open the filepath provided for nations, exits if the path cannot be found
    let input: String = match open_file(&opt.path) {
        Ok(val) => val,
        Err(e) => {
            println!("{:?}", e);
            return
        }
    };

    // the semaphore manages the number of concurrent requests we can have, we do one because that's
    // all ns allows :,)
    let semaphore = Semaphore::new(1);
    // this vecdeque (a vector that is easy to push to from both ends) holds the times for our
    // requests. we use the length of the vec and the time of the first request to manage
    // our ratelimit compliance
    let mut request_times: VecDeque<Instant> = VecDeque::new();

    let client = Client::new();

    for record in input.split('\n') {
        let fields: Vec<&str> = record
            .split(',')
            .map(|x| (*x).trim())
            .filter(|x| !x.is_empty())
            .collect();

        let nation: String = match fields.get(0) {
            Some(val) => (*val).trim().to_lowercase().replace(' ', "_").to_owned(),
            None => continue
        };

        let password: String = match fields.get(1) {
            Some(val) => (*val).trim().to_owned(),
            None => continue
        };

        if let Some(user) = &opt.user_agent {
            ratelimiter(&limit, &semaphore, &mut request_times, &nation, &password, &client, user).await;
        }
    }

}
