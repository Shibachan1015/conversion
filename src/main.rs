// 摂氏華氏変換
// °C = ((°F) -32) ÷ 1.8
// °F = °C × 1.8 + 32

use std::process;
use std::io;
mod my_calc; // calc_ctofなどの関数のモジュール

fn main() {
    main_loop();
}

pub fn main_loop() -> i32 {
    loop {
        first_call();

        let mut select_number = String::new();

        io::stdin()
            .read_line(&mut select_number)
            .expect("行の読み込みに失敗しました。");

        let select_number: i32 = match select_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\n{}を受け付けました。\n", &select_number);

        match select_number {
            e if e == 1 => convert_1(),
            e if e == 2 => convert_2(),
            e if e == 0 => process::exit(0),
            e => println!("{}", e),
        }
    }
}

/*---------------------------------------------------------------------------*/

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

    println!("\n          摂氏を華氏、または華氏を摂氏に変換します。\n");
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

        let current_temperature: f32 = match current_temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let calc_number_ctof: f32 = my_calc::my_calc::calc_ctof(current_temperature);

        println!("\n
            +----------------------------------+\n
            |摂氏{}°Cは華氏{}°Fです。          |\n
            +----------------------------------+\n
            \n
            ",current_temperature, calc_number_ctof);

        break;
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

        let current_temperature: f32 = match current_temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let calc_number_ftoc: f32 = my_calc::my_calc::calc_ftoc(current_temperature);

        println!("\n
            +----------------------------------+\n
            |華氏{}°Fは摂氏{}°Cです。          |\n
            +----------------------------------+\n
            \n
            ",current_temperature, calc_number_ftoc);

        break;
    }
}




