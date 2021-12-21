use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;
use ansi_term::Colour;
#[derive(StructOpt)]
struct Cli {
  pattern: String,
  #[structopt(parse(from_os_str))]
  path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
  let args = Cli::from_args();

  let content = std::fs::read_to_string(&args.path)
    .with_context(|_| format!("Could not read file :{:?}", &args.path))?;
  for line in content.lines() {
    if line.contains(&args.pattern) {
      println!("{}", Colour::Red.paint(line));
    }else{
      println!("{}",Colour::Green.paint(line));
    }
  }
  Ok(())
}
