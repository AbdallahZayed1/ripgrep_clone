use std::fs ;
use std::error::Error;
use std::env ;
pub struct Config{
   pub query : String,
   pub file_path : String,
   pub ignore_case : Option<bool>
  }
  impl Config {
    pub fn build (mut args:  impl  Iterator<Item = String>)-> Result<Config, & 'static str>{
      args.next();

      let query = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a query string"),
      };

      let file_path = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a file path"),
      };
      
      let ignore_case: Option<bool> = match args.next() {
          Some(strin)=> match strin.as_str() {
              "sens"=> Some(false),
              "insens"=> Some(true) ,
              _ => Some(env::var("IGNORE_CASE").is_ok()),
          },
          None => Some(env::var("IGNORE_CASE").is_ok())
      };
      Ok(Config {
          query,
          file_path,
          ignore_case  ,

      })
    }
  }
  
  
  pub fn run (config : Config) -> Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case == Some(true){
        search_case_insensetive(&config.query, &content)
    }else{
      search(&config.query, &content)
    };
   for line in results {
       println!("{line}")
   }
   Ok(())
  }

  pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
  .lines()
  .filter(|line| line.contains(&query))
  .collect()
}
pub fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
  .lines()
  .filter(| line|line.to_lowercase().contains(&query.to_lowercase()))
  .collect()
}

  #[cfg(test)]
  mod tests{
    use super::* ;
    #[test]
    fn one_result () {
      let query = "duct";
      let contents = "\
Rust:
safe, fast, productive.
Pick three.";

      assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
  #[test]
  fn case_sensitive () {
    let query = "dUcT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search_case_insensetive(query, contents));
}
  }

