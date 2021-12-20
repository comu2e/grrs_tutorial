/**
 * https://rust-cli.github.io/book/tutorial/cli-args.html
 * $ grrs foobar test.txt
 * CLI引数の取り方：
 * std::env::args().nth(1...).expect(msg)
 * データ型としてのCLI引数
 * structの導入により、型指定する。
 * PathbufはStringのようなものだが、クロスプラットフォームで動作するファイルシステムパスのためのもの
 * 
 * 
 * 
***/


use structopt::StructOpt;

fn main()-> Result<(),Box<dyn std::error::Error>> {

    let content = std::fs::read_to_string("test.xtx")?;
          println!("file content: {}",content);
          Ok(())
    
}

/*
  deriveすることでmain内に
  std::env::nth(..)を定義してCli構造体に渡す必要な無くなって便利
  let args = Cli::from_args()だけでよいはず。
  他の引数もどんどん追加しても簡素に書けるので便利。 
 */
#[derive(StructOpt)]
struct Cli {
  pattern: String,
  #[structopt(parse(from_os_str))]
  path : std::path::PathBuf,
}




