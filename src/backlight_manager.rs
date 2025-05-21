use ch32_hal::timer::{simple_pwm::SimplePwm, Channel, GeneralInstance16bit};
use level::BrightnessLevel;

pub struct BacklightManager<'a, T: GeneralInstance16bit> {
    inner: SimplePwm<'a, T>,
    channel: Channel,
    brightness: BrightnessLevel,
}

impl<'a, T: GeneralInstance16bit> BacklightManager<'a, T> {
    pub fn new(pwm: SimplePwm<'a, T>, channel: Channel) -> Self {
        Self {
            inner: pwm,
            channel,
            brightness: BrightnessLevel::default(),
        }
    }

    pub fn current_brightness(&self) -> BrightnessLevel {
        self.brightness
    }
}

impl<'a, T: GeneralInstance16bit> BacklightManager<'a, T> {
    pub fn enable(&mut self) {
        self.inner.enable(self.channel);
        self.set_brightness(self.brightness);
    }

    fn set_duty(&mut self, duty: u32) {
        self.inner.set_duty(self.channel, duty);
    }

    pub fn set_brightness(&mut self, level: BrightnessLevel) {
        self.brightness = level;

        let max_duty = self.inner.get_max_duty();
        let max_weight = BrightnessLevel::Max.weight();
        let duty = max_duty / max_weight * level.weight();
        self.set_duty(duty);
    }
}

pub mod level {
    #[allow(unused)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum BrightnessLevel {
        Off,
        Low0,
        Low1,
        Low2,
        Medium0,
        Medium1,
        Medium2,
        High0,
        High1,
        High2,
        Max,
    }

    impl Default for BrightnessLevel {
        fn default() -> Self {
            BrightnessLevel::Max
        }
    }

    impl BrightnessLevel {
        pub fn weight(&self) -> u32 {
            match self {
                BrightnessLevel::Off => 0,
                BrightnessLevel::Low0 => 1,
                BrightnessLevel::Low1 => 2,
                BrightnessLevel::Low2 => 3,
                BrightnessLevel::Medium0 => 4,
                BrightnessLevel::Medium1 => 5,
                BrightnessLevel::Medium2 => 6,
                BrightnessLevel::High0 => 7,
                BrightnessLevel::High1 => 8,
                BrightnessLevel::High2 => 9,
                BrightnessLevel::Max => 10,
            }
        }
    }

    impl BrightnessLevel {
        pub fn next(&self) -> Self {
            match self {
                BrightnessLevel::Off => BrightnessLevel::Low0,
                BrightnessLevel::Low0 => BrightnessLevel::Low1,
                BrightnessLevel::Low1 => BrightnessLevel::Low2,
                BrightnessLevel::Low2 => BrightnessLevel::Medium0,
                BrightnessLevel::Medium0 => BrightnessLevel::Medium1,
                BrightnessLevel::Medium1 => BrightnessLevel::Medium2,
                BrightnessLevel::Medium2 => BrightnessLevel::High0,
                BrightnessLevel::High0 => BrightnessLevel::High1,
                BrightnessLevel::High1 => BrightnessLevel::High2,
                BrightnessLevel::High2 => BrightnessLevel::Max,
                BrightnessLevel::Max => BrightnessLevel::Off,
            }
        }
    }
}
