# rust-aes-file
最近の暗号化技術の中ではマイブームであるAESを使って、ファイルを暗号化してみようという試みをしました。

通常ファイルの読み込みと書き込みには、高速化を図るためにBufReaderとBufWriterを採用しました。

暗号変換については、Crateで提供されているAesWriterとAesReaderを用いました。

## Usage
関数として、
- encrypt
- decrypt

の二つを用意しています。この二つの役割は以下のようになっています。

| 関数名  | 役割                       |
| ------- | -------------------------- |
| encrypt | 対象のファイルを暗号化する |
| decrypt | 対象のファイルを復号化する |

実行時には以下のようにコマンドを入力してください。
```bash
cargo run 
```
最初の実行ではCrateのパッケージを取得するため多少時間がかかります。

## Note

この実装では、パスワードの配列に関して大きく制約を課しているので、必要であれば適宜変更をお願いします。また、コントリビュートは大歓迎です。
