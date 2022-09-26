// ファイル操作のライブラリーを読む
use std::fs;
fn main() {
  // ファイル名を指定
  let afile = "./fizzbuzz_python.txt";
  let bfile = "./fizzbuzz_rust.txt";

  // ファイルを文字列として読み込む
  let astr = fs::read_to_string(afile).unwrap();
  let bstr = fs::read_to_string(bfile).unwrap();

  // 年のためトリム
  let astr = astr.trim();
  let bstr = bstr.trim();

  // 比較
  if astr == bstr {
    println!("OK");
  } else {
    println!("NG");
  }
}

