#![no_std]
#![no_main]

use arduino_hal::port::{mode::Input, mode::PullUp, Pin, PinOps};
use panic_halt as _;

type AnyPullUpPin<PinId> = Pin<Input<PullUp>, PinId>;

fn debounce_pullup<PinId>(button: &AnyPullUpPin<PinId>, delay_ms: u16) -> bool
where
    PinId: PinOps,
{
    if button.is_low() {
        arduino_hal::delay_ms(delay_ms);
        return button.is_low();
    }
    false
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Power gate
    let mut common_power_gate = pins.a3.into_output();
    // Open gates
    let mut open_gate_1 = pins.a1.into_output();
    let mut open_gate_2 = pins.a2.into_output();

    // Buttons
    let open_button: AnyPullUpPin<_> = pins.d12.into_pull_up_input();
    let close_button: AnyPullUpPin<_> = pins.d11.into_pull_up_input();

    // Optical stops
    let open_sensor = pins.d2;
    let close_sensor = pins.d3;
    // Optical stop trigger vars. Initialize as true to make panic_reset safe
    let mut open_stop_triggered: bool = true;
    let mut close_stop_triggered: bool = true;

    let mut idle_counter: u32 = 0;
    let reset_after: u32 = 600; // at 10 cycles per second, this is 1 minute

    // Delay within open / close whiles to limit noise
    let time_delay_ms = 20;
    let soft_stop_delay_ms: u16 = 5;
    let soft_stop_iters: u16 = 10;

    loop {
        if !debounce_pullup(&open_button, time_delay_ms)
            && !debounce_pullup(&close_button, time_delay_ms)
        {
            idle_counter += 1;
            if idle_counter >= reset_after {
                close_stop_triggered = false;
                open_stop_triggered = false;
                idle_counter = 0
            }
        }
        while debounce_pullup(&open_button, time_delay_ms) && !open_stop_triggered {
            open_gate_1.set_high();
            open_gate_2.set_high();
            common_power_gate.set_high();
            if close_sensor.is_high() {
                close_stop_triggered = false;
            }
            if open_sensor.is_high() {
                // Gradually stop the motor
                for _ in 0..soft_stop_iters {
                    common_power_gate.set_low();
                    arduino_hal::delay_ms(soft_stop_delay_ms);
                    common_power_gate.set_high();
                    arduino_hal::delay_ms(soft_stop_delay_ms);
                }
                open_stop_triggered = true;
            }
        }
        open_gate_2.set_low();
        open_gate_1.set_low();
        common_power_gate.set_low();

        while debounce_pullup(&close_button, time_delay_ms) && !close_stop_triggered {
            common_power_gate.set_high();
            if close_sensor.is_high() {
                for _ in 0..soft_stop_iters {
                    common_power_gate.set_low();
                    arduino_hal::delay_ms(soft_stop_delay_ms);
                    common_power_gate.set_high();
                    arduino_hal::delay_ms(soft_stop_delay_ms);
                }
                close_stop_triggered = true;
            }
            if open_sensor.is_high() {
                open_stop_triggered = false;
            }
        }
        common_power_gate.set_low();

        arduino_hal::delay_ms(10);
    }
}
