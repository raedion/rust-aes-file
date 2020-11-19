use aes::{Aes256, Aes128, NewBlockCipher, BlockCipher};
use aes::cipher::generic_array::GenericArray;
use std::io::{BufReader, BufRead, Read, BufWriter, Write};
use std::fs::File;

const FILE_SIZE: usize = 1024;

fn main() {
    encrypt("Target/src.txt", "Target/dst.txt", "pass");
}

/// 暗号化<br>
/// 第1引数には読み込み元のファイル<br>
/// 第2引数には書き込み先のファイル<br>
/// 第3引数にはパスワード(今後実装する予定)
fn encrypt(src: &str, dst: &str, pass: &str) {
    let src_file = File::open(src).unwrap();                // ファイルを開く
    let mut reader = BufReader::new(&src_file);             // 読み込み用の機能を呼び出し
    let mut block: [u8; FILE_SIZE] = [0u8; FILE_SIZE];      // 空のバイト配列を用意
    reader.read(&mut block);                                // バイト配列にファイル情報を読み出し

    let dst_file = File::create(dst).unwrap();              // 出力先ファイルを指定
    let mut writer = BufWriter::new(&dst_file);             // 書き込み用の機能を呼び出し
    writer.write(&block);                                   // 書き込みを実行
}