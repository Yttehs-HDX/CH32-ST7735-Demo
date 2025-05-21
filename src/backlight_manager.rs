use ch32_hal::timer::{simple_pwm::SimplePwm, Channel, GeneralInstance16bit};

pub struct BacklightManager<'a, T: GeneralInstance16bit> {
    inner: SimplePwm<'a, T>,
    channel: Channel,
}

impl<'a, T: GeneralInstance16bit> BacklightManager<'a, T> {
    pub fn new(pwm: SimplePwm<'a, T>, channel: Channel) -> Self {
        Self {
            inner: pwm,
            channel,
        }
    }

    pub fn inner_mut(&mut self) -> &mut SimplePwm<'a, T> {
        &mut self.inner
    }
}

impl<'a, T: GeneralInstance16bit> BacklightManager<'a, T> {
    pub fn enable(&mut self) {
        self.inner.enable(self.channel);
    }

    pub fn set_duty(&mut self, duty: u32) {
        self.inner.set_duty(self.channel, duty);
    }
}
