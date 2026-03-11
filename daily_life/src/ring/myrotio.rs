pub async fn sub_main() {
    //println!("This is a function in the myrotio module 22.");
    let time_gap_minutes = 25; // 25 分钟，单位秒
    println!("每 {} 分钟响一次铃声，按 Ctrl+C 退出", time_gap_minutes);

    loop {
        play_beep();
        println!("铃声已播放 → {}", chrono::Local::now().format("%H:%M:%S"));

        tokio::time::sleep(Duration::from_secs(time_gap_minutes * 60)).await;
    }
}

use core::time;
use rodio::{OutputStream, Sink, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

fn play_beep() {
    // 方式1：播放系统自带铃声（Windows 示例）
    #[cfg(windows)]
    {
        use winapi::um::winuser::MessageBeep;
        unsafe {
            MessageBeep(0);
        } // 简单蜂鸣
    }

    // 方式2：用 rodio 播放一个 .wav 文件（跨平台）
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // 你可以准备一个短铃声音频文件，或者用下面简单正弦波
    // 这里示例用一个文件（需自己放一个 bell.wav）
    // let file = File::open("bell.wav").unwrap();
    // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    // sink.append(source);

    // 或者用 rodio 生成简单蜂鸣（无需文件）
    // let source = rodio::source::SineWave::new(800.0, 44100.0)
    //     .take_duration(Duration::from_millis(400))
    //     .amplify(0.5);
    let source = rodio::source::SineWave::new(800.0) // 只传频率
        .take_duration(Duration::from_millis(1000)) // 限制播放时长
        .amplify(0.5); // 放大/缩小音量（0.0~1.0 常见）
    sink.append(source);

    sink.sleep_until_end(); // 阻塞直到播放完
}
