use std::io::stdin;

mod encryptor;

fn main() {
    println!("Please select operation(0: encrypt, 1: decrypt)");
    let select_command = match input_read().parse::<u8>() {   // 入力された値を数字に変換
        Ok(value) => {value}                                                    // 数字なら問題ない
        Err(_) => {                                                             // 数字でないなら終了
            println!("Please input valid number!");
            return;
        }
    };

    println!("Please select input file");
    let input_file_path = input_read();                  // 読み込みファイルの指定

    println!("Please select output file");
    let output_file_path = input_read();                // 書き込みファイルの指定

    println!("Please input password");
    let password = input_read();                                // パスワードの指定

    if select_command == 0 {
        let enc_result = encryptor::encrypt(&*input_file_path, &*output_file_path, &*password);
        if let Ok(_) = enc_result {
            println!("Encrypt Finished!");
        }  // 復号化の実行
        else {
            println!("{}", enc_result.err().unwrap());
        }
        return;
    }
    else if select_command == 1 {
        let dec_result = encryptor::decrypt(&*input_file_path, &*output_file_path, &*password);
        if let Ok(_) = dec_result {
            println!("Decrypt succeeded!");
        }  // 復号化の実行
        else {
            println!("{}", dec_result.err().unwrap());
        }
        return;
    }
    println!("Don't nothing");                                                  // 該当しない操作であったら何もしない
}
fn input_read() -> String {
    let mut input_read = String::new();
    stdin().read_line(&mut input_read).unwrap();
    input_read.trim().parse().unwrap()
}
