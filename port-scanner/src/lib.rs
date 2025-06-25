use std::io::{BufReader,BufRead};
use std::error::Error;
use std::process::Command;
use std::fs::File;
use url::Url;
use reqwest::blocking::Client;


// Argument parsing 
#[derive(Debug)]
pub struct Config {
	pub url: Url,
	pub dictionary: File,
} 

impl Config {
	pub fn build(args: &Vec<String>) -> Result<Config, Box<dyn Error>> {
		if args.len() < 3 {
			return Err("Not enough arguments. Use: ./port-scanner $url $path-to-dictionary".into());
		}
		let url = Url::parse(&args[1])?;
		let dictionary = File::open(&args[2])?;

		Ok(Config{url, dictionary})
	}
}

// Program logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
	// Checking arguments
	let domain = config.url.domain().ok_or("No domain found in URL")?; 

	// Computation
	print_title();	
	ping_domain(&domain)?;	
	scan(&config);	
	Ok(())
}

// Private funcitons
fn print_title() {
	// Made with $> figlet -f slant "port scanner"
	let green = "\x1b[1;32m";
    let reset = "\x1b[0m";
	println!("{green}");
    println!("                      __                                          ");
    println!("    ____  ____  _____/ /_   ______________ _____  ____  ___  _____");
    println!("   / __ \\/ __ \\/ ___/ __/  / ___/ ___/ __ `/ __ \\/ __ \\/ _ \\/ ___/");
    println!("  / /_/ / /_/ / /  / /_   (__  ) /__/ /_/ / / / / / / /  __/ /    ");
    println!(" / .___/\\____/_/   \\__/  /____/\\___/\\__,_/_/ /_/_/ /_/\\___/_/     ");
    println!("/_/                                                            ");
	println!("{reset}");
	println!("Made in Rust 🦀 by @jofmar00!");
	println!();
}

fn ping_domain (domain: &str) -> Result<(), Box<dyn Error>> {
	print!("Checking if {domain} is up...");
	
	let output = Command::new("ping").arg("-c").arg("1").arg(domain).output()?;
	
	if output.status.success() {
		println!("OK");	
		Ok(())
	}	
	else {
		println!("ERROR");
		Err("Ping falló".into())
	}
}

fn scan(config: &Config) {
	let reader = BufReader::new(&config.dictionary);
	let client = Client::new();

	for line in reader.lines() {
		let word= match line {
			Ok(l) => l,
			Err(_) => return,
		};

		let new_url = config.url.to_string().replace("FUZZ", &word);

		print!("{} -> ", new_url);

		// Send request
		let response = client.get(new_url).send();
		match response {
			Ok(msg) => {
				println!("{}", msg.status());
			}
			Err(e) => println!("ERROR: {}", e),
		};
	}
}
