use minigrep::run;
use minigrep::Config;
use structopt::StructOpt;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Config::from_args();
    println!("{:?}", opt);
    run(&opt)
} // run validates if the path is okay and then searches
  // the string in the file defined by the path
  // if path is invalid, it returns an error
