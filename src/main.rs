use anyhow::{Context,Result};

// anyhowでエラーの状態をエラーメッセージに記載できるようにする。
fn main() -> Result<()> {
  let path = "test.txt";
  let content = std::fs::read_to_string(path).with_context(|| format!("could not read file {}",path))?;
  println!("file contetn:{}",content);
  Ok(())
}