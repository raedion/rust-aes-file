extern crate crypto;
extern crate aesstream;

use std::io::{Result, BufReader, Read, BufWriter, Write};
use aesstream::{AesWriter, AesReader};
use crypto::aessafe::{AesSafe256Encryptor, AesSafe256Decryptor};
use std::fs::File;

const FILE_SIZE: usize = 1024;
const PASSWORD_SIZE: usize = 32;        // パスワードのバイトサイズ. aessafeパッケージのサイズ(128, 192, 256)に応じて変化させる必要がある

/// 暗号化<br>
/// 第1引数には読み込み元のファイル<br>
/// 第2引数には書き込み先のファイル<br>
/// 第3引数にはパスワード<br>
/// 戻り値は実行結果
pub fn encrypt(src: &str, dst: &str, pass: &str) -> Result<()>{
    let src_file = File::open(src)?;                        // ファイルを開く
    let mut reader = BufReader::new(&src_file);             // 読み込み用の機能を呼び出し
    let mut block: [u8; FILE_SIZE] = [0u8; FILE_SIZE];      // 空のバイト配列を用意
    reader.read(&mut block)?;                               // バイト配列にファイル情報を読み出し

    let key = pass.as_bytes();                              // 引数をバイト変換
    if key.len() > PASSWORD_SIZE {
        println!("Too long password!");
        return Err(std::io::Error::from(std::io::ErrorKind::Other));                      // 異常終了なのでエラーを出す
    }
    let mut key_array = [0u8; PASSWORD_SIZE];               // バイト用配列
    for i in 0..key.len() {
        key_array[i] = key[i];                              // スライスから配列へ変換
    }

    let dst_file = File::create(dst)?;                      // 出力先ファイルを指定
    let encryptor = AesSafe256Encryptor::new(&key_array);   // 暗号化の呼び出し
    let mut writer = AesWriter::new(dst_file, encryptor)?;  // ファイルへの暗号書き出しの呼び出し
    writer.write_all(&block)?;                              // 実行
    print!("");
    Ok(())
}

/// 復号化<br>
/// 第1引数には読み込み元のファイル<br>
/// 第2引数には書き込み先のファイル<br>
/// 第3引数にはパスワード<br>
/// 戻り値は実行結果
pub fn decrypt(dst: &str, src: &str, pass: &str) -> Result<()>{
    let key = pass.as_bytes();                              // 引数をバイト変換
    if key.len() > PASSWORD_SIZE {
        println!("Too long password!");
        return Err(std::io::Error::from(std::io::ErrorKind::Other));                      // 異常終了なのでエラーを出す
    }
    let mut key_array = [0u8; PASSWORD_SIZE];               // バイト用配列
    for i in 0..key.len() {
        key_array[i] = key[i];                              // スライスから配列へ変換
    }

    let src_file = File::open(dst)?;                        // ファイルを開く
    let decryptor = AesSafe256Decryptor::new(&key_array);
    let mut reader = AesReader::new(&src_file, decryptor)?; // 読み込み用の機能を呼び出し
    let mut block: [u8; FILE_SIZE] = [0u8; FILE_SIZE];      // 空のバイト配列を用意
    reader.read(&mut block)?;                               // バイト配列にファイル情報を読み出し

    let dst_file = File::create(src)?;                      // 出力先ファイルを指定
    let mut writer = BufWriter::new(&dst_file);             // 書き込み用の機能を呼び出し
    writer.write(&block)?;                                  // 書き込みを実行
    Ok(())
}