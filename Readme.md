# Command Line ApplicationsをRustで作るのサンプル
https://rust-cli.github.io/book/tutorial/index.html

# Version
cargo 1.56.0

# コマンド引数の取得方
```
std::env:nth()
```
上記でもいいが下記のように書くのがいい。

```use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli{
    pattern: String ,
    #[structopt(parse(from_os_str))]
    path::std::path::Path::Buf
}

fn main(){
 let content = std::fs::read_to_string()
}
```

# test

# 動作方法
```$ cargo run print` src```

![image](https://user-images.githubusercontent.com/5231283/146948034-76a985f1-5210-41da-9274-c7ebaee59df7.png)
