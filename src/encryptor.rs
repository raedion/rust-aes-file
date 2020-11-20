extern crate crypto;
extern crate aesstream;

use std::io::{BufReader, Read, BufWriter, Write};
use aesstream::{AesWriter, AesReader};
use crypto::aessafe::{AesSafe128Encryptor, AesSafe128Decryptor};
use std::fs::File;

const FILE_SIZE: usize = 1024;

/// 暗号化<br>
/// 第1引数には読み込み元のファイル<br>
/// 第2引数には書き込み先のファイル<br>
/// 第3引数にはパスワード
pub fn encrypt(src: &str, dst: &str, pass: &str) {
    let src_file = File::open(src).unwrap();                // ファイルを開く
    let mut reader = BufReader::new(&src_file);             // 読み込み用の機能を呼び出し
    let mut block: [u8; FILE_SIZE] = [0u8; FILE_SIZE];      // 空のバイト配列を用意
    reader.read(&mut block);                                // バイト配列にファイル情報を読み出し

    let key = pass.as_bytes();                              // 引数をバイト変換
    let mut key_array = [0u8; 16];                          // バイト用配列
    for i in 0..key.len() {
        key_array[i] = key[i];                              // スライスから配列へ変換
    }

    let dst_file = File::create(dst).unwrap();                      // 出力先ファイルを指定
    let encryptor = AesSafe128Encryptor::new(&key_array);           // 暗号化の呼び出し
    let mut writer = AesWriter::new(dst_file, encryptor).unwrap();  // ファイルへの暗号書き出しの呼び出し
    writer.write_all(&block);                                       // 実行
}

/// 復号化<br>
/// 第1引数には読み込み元のファイル<br>
/// 第2引数には書き込み先のファイル<br>
/// 第3引数にはパスワード
pub fn decrypt(dst: &str, src: &str, pass: &str) {
    let key = pass.as_bytes();                              // 引数をバイト変換
    let mut key_array = [0u8; 16];                          // バイト用配列
    for i in 0..key.len() {
        key_array[i] = key[i];                              // スライスから配列へ変換
    }

    let src_file = File::open(dst).unwrap();                        // ファイルを開く
    let decryptor = AesSafe128Decryptor::new(&key_array);
    let mut reader = AesReader::new(&src_file, decryptor).unwrap(); // 読み込み用の機能を呼び出し
    let mut block: [u8; FILE_SIZE] = [0u8; FILE_SIZE];              // 空のバイト配列を用意
    reader.read(&mut block);                                        // バイト配列にファイル情報を読み出し

    let dst_file = File::create(src).unwrap();              // 出力先ファイルを指定
    let mut writer = BufWriter::new(&dst_file);             // 書き込み用の機能を呼び出し
    writer.write(&block);                                   // 書き込みを実行
}