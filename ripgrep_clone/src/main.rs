use std::env ;
use std::process ;
use ripgrep_clone::Config;
fn main() {

 let config = Config::build(env::args()).unwrap_or_else(|err| {
  eprintln!("Problem occured wile compiling : {err}");
  process::exit(1);
});
if let Err(e) = ripgrep_clone::run(config){
  eprintln!(" the eroor is {e}");
  process::exit(1);
}

}