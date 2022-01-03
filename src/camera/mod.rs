//use edsdk::wrap;
use edsdk::types;

#[allow(dead_code)]
pub fn convert_iso(iso_str: u32)->types::ISOSpeed{
    match iso_str{
        50=>{types::ISOSpeed::ISO50}
        100=>{types::ISOSpeed::ISO100}
        200=>{types::ISOSpeed::ISO200}
        400=>{types::ISOSpeed::ISO400}
        800=>{types::ISOSpeed::ISO800}
        1600=>{types::ISOSpeed::ISO160}
        3200=>{types::ISOSpeed::ISO320}
        _=>{types::ISOSpeed::ISO100}
    }
}

#[allow(dead_code)]
pub fn convert_av(av_str: &str)->types::ApertureValue{
    match av_str{
        "1.0"=>{types::ApertureValue::Av1_0}
        "1.1"=>{types::ApertureValue::Av1_1}
        "1.2"=>{types::ApertureValue::Av1_2}
        "1.4"=>{types::ApertureValue::Av1_4}
        "1.6"=>{types::ApertureValue::Av1_6}
        "1.8"=>{types::ApertureValue::Av1_8}
        "2.0"=>{types::ApertureValue::Av2_0}
        "2.2"=>{types::ApertureValue::Av2_2}
        "2.5"=>{types::ApertureValue::Av2_5}
        "2.8"=>{types::ApertureValue::Av2_8}
        "3.2"=>{types::ApertureValue::Av3_2}
        "3.5"=>{types::ApertureValue::Av3_5}
        "4.0"=>{types::ApertureValue::Av4_0}
        "4.5"=>{types::ApertureValue::Av4_5}
        "5.0"=>{types::ApertureValue::Av5_0}
        "5.6"=>{types::ApertureValue::Av5_6}
        "6.3"=>{types::ApertureValue::Av6_3}
        "6.7"=>{types::ApertureValue::Av6_7}
        "7.1"=>{types::ApertureValue::Av7_1}
        "8.0"=>{types::ApertureValue::Av8_0}
        "9.0"=>{types::ApertureValue::Av9_0}
        "9.5"=>{types::ApertureValue::Av9_5}
        "10.0"=>{types::ApertureValue::Av10_0}
        "11.0"=>{types::ApertureValue::Av11_0}
        "13.0"=>{types::ApertureValue::Av13_0}
        "14.0"=>{types::ApertureValue::Av14_0}
        "16.0"=>{types::ApertureValue::Av16_0}
        "18.0"=>{types::ApertureValue::Av18_0}
        "19.0"=>{types::ApertureValue::Av19_0}
        "20.0"=>{types::ApertureValue::Av20_0}
        "22.0"=>{types::ApertureValue::Av22_0}
        "25.0"=>{types::ApertureValue::Av25_0}
        "27.0"=>{types::ApertureValue::Av27_0}
        "29.0"=>{types::ApertureValue::Av29_0}
        "32.0"=>{types::ApertureValue::Av32_0}
        _=>{types::ApertureValue::Av4_0}
    }
}

#[allow(dead_code)]
pub fn convert_tv(tv_str: &str)->types::ShutterSpeed{
    match tv_str{
        "3"=>{types::ShutterSpeed::Tv3}
        "2.5"=>{types::ShutterSpeed::Tv2_5}
        "2"=>{types::ShutterSpeed::Tv2}
        "1.6"=>{types::ShutterSpeed::Tv1_6}
        "1.5"=>{types::ShutterSpeed::Tv1_5}
        "1.3"=>{types::ShutterSpeed::Tv1_3}
        "1"=>{types::ShutterSpeed::Tv1}
        "0.8"=>{types::ShutterSpeed::Tv0_8}
        "0.7"=>{types::ShutterSpeed::Tv0_7}
        "0.6"=>{types::ShutterSpeed::Tv0_6}
        "0.5"=>{types::ShutterSpeed::Tv0_5}
        "0.4"=>{types::ShutterSpeed::Tv0_4}
        "0.3"=>{types::ShutterSpeed::Tv0_3}
        "1/4"=>{types::ShutterSpeed::Tv1_4th}
        "1/5"=>{types::ShutterSpeed::Tv1_5th}
        "1/6"=>{types::ShutterSpeed::Tv1_6th}
        "1/8"=>{types::ShutterSpeed::Tv1_8th}
        "1/10"=>{types::ShutterSpeed::Tv1_10th}
        "1/13"=>{types::ShutterSpeed::Tv1_13th}
        "1/15"=>{types::ShutterSpeed::Tv1_15th}
        "1/20"=>{types::ShutterSpeed::Tv1_20th}
        "1/25"=>{types::ShutterSpeed::Tv1_25th}
        "1/30"=>{types::ShutterSpeed::Tv1_30th}
        "1/40"=>{types::ShutterSpeed::Tv1_40th}
        "1/45"=>{types::ShutterSpeed::Tv1_45th}
        "1/50"=>{types::ShutterSpeed::Tv1_50th}
        "1/60"=>{types::ShutterSpeed::Tv1_60th}
        "1/80"=>{types::ShutterSpeed::Tv1_80th}
        "1/90"=>{types::ShutterSpeed::Tv1_90th}
        "1/100"=>{types::ShutterSpeed::Tv1_100th}
        "1/125"=>{types::ShutterSpeed::Tv1_125th}
        "1/160"=>{types::ShutterSpeed::Tv1_160th}
        "1/180"=>{types::ShutterSpeed::Tv1_180th}
        "1/200"=>{types::ShutterSpeed::Tv1_200th}
        "1/250"=>{types::ShutterSpeed::Tv1_250th}
        _=>{types::ShutterSpeed::Tv1_15th}
    }
}

#[allow(dead_code)]
pub fn convert_tv_denominator(denominator: u32)->u32{
    return match denominator{
        15=>16,
        30=>32,
        60=>64,
        125=>128,
        250=>256,
        500=>512,
        1000=>1024,
        2000=>2048,
        4000=>4096,
        8000=>8192,
        16000=>16384,
        32000=>32768,
        64000=>65536,
        _=>denominator
    };
}
