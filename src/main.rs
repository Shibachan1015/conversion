// 摂氏華氏変換
// °C = ((°F) -32) ÷ 1.8
// °F = °C × 1.8 + 32

    // 最終目標　println!("摂氏と華氏双方向に温度を変換できます。"); 現状双方向ではない。
    // exit()などで正常終了を実装したい。 yet
    // 次は華氏から摂氏に変換を実装する。 ok
    // |idea| if or match で　摂氏から華氏か華氏から摂氏を判定してそこでctof関数かftocを呼びだせば良いのではないか。 matchでの実装ができなかったためifでお茶を濁した。
    /*   print(choice c=1 or choice f=2)
         | |---if choice c(){};
         |      |---
         |
         |---if choice f(){};
    */

mod my_calc;  // calc_ctofなどの関数のモジュール
use std::io;

fn main() {

    // ｃからf（１）の場合
    // let convert1 = convert_ctof_1();
    // ｆからｃ（２）の場合
    // let convert2 = convert_ftoc_2();
    
    let disp_info = 
        "\n
        +-------------------------------------------------+\n
        |摂氏(°C)から華氏(°F)に変換する場合は数値 1 を入力|\n
        +-------------------------------------------------+\n
        |華氏(°F)から摂氏(°C)に変換する場合は数値 2 を入力|\n
        +-------------------------------------------------+\n
        ";

    
        println!("摂氏を華氏、または華氏を摂氏に変換します。\n");
        println!("{}", disp_info);

    loop {
        let mut c_or_f = String::new();

        io::stdin()
            .read_line(&mut c_or_f)
            .expect("行の読み込みに失敗しました。");

        let c_or_f: u32 = match c_or_f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("{}を受け付けました。\n", &c_or_f);
        
        if c_or_f == 1 {
            convert_ctof_1();
        } else if c_or_f == 2 {
            convert_ftoc_2();
        } else {
            break;
        } 

        /*
        match c_or_f {
            1 => convert1,
            2 => convert2,
            _ => continue
        } 
        */
    }

}
/*---------------------------------------------------------------------------*/
fn convert_ctof_1 () {
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

fn convert_ftoc_2 () {
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





