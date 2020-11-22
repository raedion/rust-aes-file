use crate::read::input_read;
use crate::encryptor;

/// 実行時引数で処理を行う<br>
/// ビルドされた実行ファイルを第一引数<br>
/// 入力元となるファイルパスを第二引数<br>
/// 出力先となるファイルパスを第三引数として処理する
pub fn args_main() {
    let args_result = args_collect();   // 実行の成否、および読み込んだ文字列のベクタをもつタプル生成
    if !args_result.0 {                 // 実行時引数の読み込みがうまくいかなかったとき
        return;                         // 何もせずに終了
    }
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
                &*args_result.1[1],                             // 入力パス
                &*args_result.1[2],                             // 出力パス
                {
                    println!("Please input password");          // パスワードの入力
                    &*(input_read())
                }
            )
        }
        else if select_command == 1 {                           // 復号化を実行
            encryptor::decrypt(
                &*args_result.1[1],                             // 入力パス
                &*args_result.1[2],                             // 出力パス
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

/// 実行時引数を受け取る<br>
/// 引数の数が合致しない場合は結果を失敗として実行を終了
/// 戻り値: (実行の成否, 読み取った文字列の集合) のタプル
fn args_collect() -> (bool, Vec<String>) {
    let args: Vec<String> = std::env::args().collect();                         // 実行時引数読み込み実行
    if args.len() != 3 {                                                        // 該当するものでなかったら
        println!("Usage: <Program Path> <Input File Path> <Output File Path>");
        return (false, vec!["".to_string()]);                                   // 失敗として戻り値を渡す
    }
    else {
        return (true, args);                                                    // 成功として戻り値を渡す
    }
}