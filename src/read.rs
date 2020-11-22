/// ユーザーの標準入力を取得<br>
/// 戻り値は入力された文字列のString型
pub fn input_read() -> String {
    let mut input_read = String::new();
    std::io::stdin().read_line(&mut input_read).unwrap();
    input_read.trim().parse().unwrap()
}