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

    exec_result = {
        if select_command == 0 {
            encryptor::encrypt(&*input_file_path, &*output_file_path, &*password)
        }
        else if select_command == 1 {
            encryptor::decrypt(&*input_file_path, &*output_file_path, &*password)
        }
        else {
            println!("Don't nothing");                                                  // 該当しない操作であったら何もしない
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
fn input_read() -> String {
    let mut input_read = String::new();
    stdin().read_line(&mut input_read).unwrap();
    input_read.trim().parse().unwrap()
}
