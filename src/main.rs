mod encryptor;
/// メイン関数<br>
/// ユーザー側はここから処理をする
fn main() {
    println!("Please select operation (0: encrypt, 1: decrypt)");
    let select_command = match input_read().parse::<u8>() {     // 入力された値を数字に変換
        Ok(value) => {value}                                    // 数字なら問題ない
        Err(_) => {                                             // 数字でないなら終了
            println!("Please input valid number!");
            return;
        }
    };
    let exec_result = {
        if select_command == 0 {                                // 暗号化を実行
            encryptor::encrypt(
                {
                    println!("Please select encrypting file");   // 暗号化したいファイルを選択
                    &*(input_read())
                },
                {
                    println!("Please name encrypted file");      // 暗号化されたファイルの出力先を指定
                    &*(input_read())
                },
                {
                    println!("Please input password");           // パスワードの入力
                    &*(input_read())
                }
            )
        }
        else if select_command == 1 {                           // 復号化を実行
            encryptor::decrypt(
                {
                    println!("Please select decrypting file");  // 復号化したいファイルを選択
                    &*(input_read())
                },
                {
                    println!("Please name decrypted file");     // 複号化されたファイルの出力先を指定
                    &*(input_read())
                },
                {
                    println!("Please input password");          // パスワードの入力
                    &*(input_read())
                }
            )
        }
        else {
            println!("Don't nothing");                          // 該当しない操作であったら何もしない
            return;
        }
    };
    if let Ok(_) = exec_result {
        println!("Operation Finished!");
    }
    else {
        println!("{}", exec_result.err().unwrap());
    }
}
/// ユーザーの標準入力を取得<br>
/// 戻り値は入力された文字列のString型
fn input_read() -> String {
    let mut input_read = String::new();
    std::io::stdin().read_line(&mut input_read).unwrap();
    input_read.trim().parse().unwrap()
}