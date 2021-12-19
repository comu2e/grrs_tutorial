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
fn main() {
    // CLI引数を取得
    let pattern = std::env::args().nth(1).expect("no pattern give");
    let path = std::env::args().nth(2).expect("no path given");
    // Cli構造体に値を設定
    /*
     * これだけでも動作するが使いにくい。
     * --pattern="foo" or --pattern "foo"に対応できない
     * $ cargo run pattern=1 test
     * >> pattern=1+1
     */
    let args = Cli{
      pattern:pattern,
      path:std::path::PathBuf::from(path),
    };
    println!("{}+1",args.pattern);
}

struct Cli {
  pattern: String,
  path : std::path::PathBuf,
}




