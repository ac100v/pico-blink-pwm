//! Raspberry Pi Pico でLチカ (PWM)
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_hal::PwmPin;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::init_clocks_and_plls,
    pac,
    pwm::{FreeRunning, Slices},
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let _clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let pwm_slices = Slices::new(pac.PWM, &mut pac.RESETS);
    // RP2040のPWMは8つのスライスに分かれていて、各スライスが
    // 2つのピンに対応している。
    // LED (GPIO25) はPWM4のチャネルB (RP2040 Datasheet 4.5.2)
    let mut pwm = pwm_slices.pwm4;
    pwm.set_ph_correct();
    pwm.enable();

    // PWMをフリーランモードで動作させる。
    // 125MHz / 62500 / 200 = 10Hz のPWM信号をGPIO28に出力。
    let mut pwm = pwm.into_mode::<FreeRunning>();
    const PWM_PERIOD: u16 = 62500; // PWMの1周期
    pwm.set_top(PWM_PERIOD - 1);
    pwm.set_div_int(200); // システムクロックの1/200をPWMに使用する
    pwm.set_div_frac(0);
    let mut channel_b = pwm.channel_b;
    let _channel_pin_b = channel_b.output_to(pins.led);
    channel_b.set_duty(PWM_PERIOD / 2); // デューティ 50%

    // メインループでは何もしない
    loop {}
}
