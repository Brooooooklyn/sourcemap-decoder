use sourcemap::Error;
use sourcemap::SourceMap;

pub fn parse(sm: &SourceMap, line: u32, column: u32) -> Result<(String, f64), Error> {
  match sm.lookup_token(line - 1, column - 1) {
    Some(s) => {
      let source = s.get_source().unwrap();
      Ok((String::from(source), (s.get_src_line() + 1) as f64))
    },
    None => {
      println!("line: {}, column: {}", line, column);
      panic!("no token from sourcemap");
    },
  }
}
