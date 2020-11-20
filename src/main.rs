mod encryptor;      // 実行関数を含むモジュールを呼び出す

fn main() {
    let mut select_command_str = String::new();                                 // 入力コマンド
    println!("Please select operation(0: encrypt, 1: decrypt)");                
    std::io::stdin().read_line(&mut select_command_str).unwrap();               // 入力された値を取得
    let select_command = match select_command_str.trim_end().parse::<u32>() {   // 入力された値を数字に変換
        Ok(value) => {value}                                                    // 数字なら問題ない
        Err(_) => {                                                             // 数字でないなら終了
            println!("Please input number!");
            return;
        }
    };

    println!("Please select input file");
    let mut input_file_path = String::new();
    std::io::stdin().read_line(&mut input_file_path).unwrap();
    input_file_path = input_file_path.trim().parse().unwrap();                  // 読み込みファイルの指定

    println!("Please select output file");
    let mut output_file_path = String::new();
    std::io::stdin().read_line(&mut output_file_path).unwrap();
    output_file_path = output_file_path.trim().parse().unwrap();                // 書き込みファイルの指定

    println!("Please input password");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    password = password.trim().parse().unwrap();                                // パスワードの指定

    if select_command == 0 {
        encryptor::encrypt(&*input_file_path, &*output_file_path, &*password);  // 暗号化の実行
        return;
    }
    else if select_command == 1 {
        encryptor::decrypt(&*input_file_path, &*output_file_path, &*password);  // 復号化の実行
        return;
    }
    println!("Don't nothing");                                                  // 該当しない操作であったら何もしない
}

