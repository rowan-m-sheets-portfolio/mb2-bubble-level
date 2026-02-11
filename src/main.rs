#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};

//use num_traits::float:Float;

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let mut display = Display::new(board.display_pins);

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer0 = Timer::new(board.TIMER0);
    let mut timer1 = Timer::new(board.TIMER1);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer0,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz50,
        )
        .unwrap();

    let mut fine = false;
    loop {
        if sensor.accel_status().unwrap().xyz_new_data() {
            if button_a.is_low().unwrap() && !button_b.is_low().unwrap() {
                fine = false;
            } else if button_b.is_low().unwrap() {
                fine = true;
            }
            let mut output: [[u8; 5]; 5] = [[0; 5]; 5];
            let (mut x, mut y, z) = sensor.acceleration().unwrap().xyz_mg();
            if z <= 0 {
                if fine {
                    x = x.clamp(-50, 50);
                    y = y.clamp(-50, 50);
                    x = match x {
                        -50..-30 => 4,
                        -30..-10 => 3,
                        -10..10 => 2,
                        10..30 => 1,
                        30..=50 => 0,
                        _ => panic!(),
                    };
                    y = match y {
                        -50..-30 => 0,
                        -30..-10 => 1,
                        -10..10 => 2,
                        10..30 => 3,
                        30..=50 => 4,
                        _ => panic!(),
                    };
                } else {
                    x = x.clamp(-500, 500);
                    y = y.clamp(-500, 500);
                    x = match x {
                        -500..-300 => 4,
                        -300..-100 => 3,
                        -100..100 => 2,
                        100..300 => 1,
                        300..=500 => 0,
                        _ => panic!(),
                    };
                    y = match y {
                        -500..-300 => 0,
                        -300..-100 => 1,
                        -100..100 => 2,
                        100..300 => 3,
                        300..=500 => 4,
                        _ => panic!(),
                    };
                }
                output[y as usize][x as usize] = 1;
                display.show(&mut timer1, output, 200);
            } else {
                display.show(&mut timer1, output, 200);
            }
        }
    }
}
