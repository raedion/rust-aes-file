mod encryptor;
mod args;
mod stdin;
mod read;

/// メイン関数<br>
/// ユーザー側はここから処理をする
fn main() {
    stdin::stdin_main();
}

