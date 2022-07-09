// 摂氏華氏変換
// °C = ((°F) -32) ÷ 1.8
// °F = °C × 1.8 + 32

// 最終目標　println!("摂氏と華氏双方向に温度を変換できます。"); 現状双方向ではない。
// 正常終了を実装したい。use std::process::exit();
// matchでの実装ができなかったためifでお茶を濁した。
// 初期画面から０を入力で終了できるようにはなった。各関数へ移行したあとはまだ。

use std::process;
mod my_calc; // calc_ctofなどの関数のモジュール
use std::io;

pub fn main() {
    main_loop();
}

pub fn main_loop() -> i32 {
    loop {
        first_call();

        let mut c_or_f = String::new();

        io::stdin()
            .read_line(&mut c_or_f)
            .expect("行の読み込みに失敗しました。");

        let c_or_f: i32 = match c_or_f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}を受け付けました。\n", &c_or_f);

        match c_or_f {
            e if e == 1 => convert_1(),
            e if e == 2 => convert_2(),
            e if e == 0 => process::exit(0),
            e => println!("{}", e),
        }
    }
}

/*---------------------------------------------------------------------------*/
/*
pub fn parse_input(num: i32) -> Result<i32, exitcode::ExitCode> {
    match num.parse::<i32>() {
        Ok(i) => ok(i),
        Err(_) => Err(exitcode::USAGE)
    }
}
*/

fn first_call() {
    let disp_info = "\n
        +-------------------------------------------------+\n
        |摂氏(°C)から華氏(°F)に変換する場合は数値 1 を入力|\n
        +-------------------------------------------------+\n
        |華氏(°F)から摂氏(°C)に変換する場合は数値 2 を入力|\n
        +-------------------------------------------------+\n
        |この画面から抜ける場合は数値 0 を入力            |\n
        +-------------------------------------------------+\n
        ";

    println!("摂氏を華氏、または華氏を摂氏に変換します。\n");
    println!("{}", disp_info);
}

fn convert_1() {
    println!("摂氏を華氏に変換します。\n");

    loop {
        println!("変換したい温度を入力してください。\n");

        let mut current_temperature = String::new();

        io::stdin()
            .read_line(&mut current_temperature)
            .expect("行の読み込みに失敗しました。");

        let current_temperature: f64 = match current_temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let calc_number_ctof: f64 = my_calc::my_calc::calc_ctof(current_temperature);
        println!("摂氏{}°Cは", current_temperature);
        println!("華氏{}°Fです。\n", calc_number_ctof);
    }
}

fn convert_2() {
    println!("華氏を摂氏に変換します。");

    loop {
        println!("変換したい温度を入力してください。");

        let mut current_temperature = String::new();

        io::stdin()
            .read_line(&mut current_temperature)
            .expect("行の読み込みに失敗しました。");

        let current_temperature: f64 = match current_temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let calc_number_ftoc: f64 = my_calc::my_calc::calc_ftoc(current_temperature);

        println!("華氏{}°Fは", current_temperature);
        println!("摂氏{}°Cです。\n", calc_number_ftoc);
    }
}




