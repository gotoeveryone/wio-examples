//! 6-2 LEDとボタン/GPIOのサンプルコードです。
//! ボタン1 (一番右のボタン) を押している間、ユーザーLEDが点灯します。
//! LEDドライバとボタンドライバを導入したバージョンです。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-2-led_and_button_driver
//! ```

#![no_std]
#![no_main]
#![allow(dead_code)] // 使用しないメソッドでコンパイラが警告を出さないようにする

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::hal::gpio::*; // GPIOの構造体やトレイトをインポートする
use wio::pac::Peripherals;
use wio::prelude::*; // 主要な構造体やトレイトをインポートする

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut pins = wio::Pins::new(peripherals.PORT);
    // LED ドライバオブジェクトを初期化する
    let mut led = Led::new(pins.user_led, &mut pins.port);
    // ボタンドライバオブジェクトを初期化する
    let button1 = Button1::new(pins.button1, &mut pins.port);

    // TODO: ボタン1を押している間、LEDが点灯するコードを書く

    loop {
        if button1.is_pressed() {
            led.turn_on()
        } else {
            led.turn_off()
        }
    }
}

// Wio Terminalのボタン1ドライバ
// TODO: Button1 を実装する
struct Button1 {
    pin: Pc26<Input<Floating>>,
}

impl Button1 {
    // PC26 ピンを入力モードに設定する
    fn new(pin: Pc26<Input<Floating>>, port: &mut Port) -> Button1 {
        Button1 {
            pin: pin.into_floating_input(port),
        }
    }
    fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }
    fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}

// Wio TerminalのユーザーLEDドライバ
pub struct Led {
    pin: Pa15<Output<PushPull>>,
}

impl Led {
    pub fn new(pin: Pa15<Input<Floating>>, port: &mut Port) -> Led {
        Led {
            pin: pin.into_push_pull_output(port),
        }
    }
    pub fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }
    pub fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}
