// 
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

fn main() {
    // CLI引数を取得
     // Cli構造体に値を設定
    /*
     * これだけでも動作するが使いにくい。
     * --pattern="foo" or --pattern "foo"に対応できない
     * $ cargo run pattern=1 test
     * >> pattern=1+1
     */
    
     /* sturctoptライブラリを使用する。
     *sturctopt=0.3.13 をCargo.tomlに記載
     * use structopt::StructOpt
     * #[derive(StructOpt)]をstruct Cli上に記載
     * これでCli構造体はStructOptを継承(Derive)する。
      */

    let args = Cli::from_args();
    // Exercide for the reader:
    //引数が正しいか確認してみる。
    print!("{}",args.pattern);
    print!("{:?}",args.path);
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




