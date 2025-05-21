use ch32_hal::{
    gpio::{AnyPin, Level, Output},
    mode::Mode,
    spi::{Instance, Spi},
};
use embassy_time::Delay;
use embedded_graphics::{
    draw_target::DrawTarget,
    image::{ImageDrawable, ImageRaw},
    pixelcolor::{raw::LittleEndian, Rgb565},
};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use st7735_lcd::{Orientation, ST7735};

pub struct DisplayManager<'a, T: Instance, M: Mode> {
    inner: DisplayInner<'a, T, M>,
}

impl<'a, T: Instance, M: Mode> DisplayManager<'a, T, M> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dc: AnyPin,
        rst: AnyPin,
        cs: AnyPin,
        spi: Spi<'a, T, M>,
        display_rgb: bool,
        display_inverted: bool,
        display_width: u32,
        display_height: u32,
    ) -> Self {
        let dc = Output::new(dc, Level::Low, Default::default());
        let rst = Output::new(rst, Level::Low, Default::default());
        let cs = Output::new(cs, Level::Low, Default::default());

        let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();

        let display = st7735_lcd::ST7735::new(
            spi_device,
            dc,
            rst,
            display_rgb,
            display_inverted,
            display_width,
            display_height,
        );

        Self { inner: display }
    }
}

impl<'a, T: Instance, M: Mode> DisplayManager<'a, T, M> {
    pub fn init(&mut self) {
        self.inner.init(&mut Delay).unwrap();
    }

    pub fn clear(&mut self, color: Rgb565) {
        self.inner.clear(color).unwrap();
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.inner.set_orientation(&orientation).unwrap();
    }

    pub fn set_offset(&mut self, x: u16, y: u16) {
        self.inner.set_offset(x, y);
    }

    pub fn draw_image(&mut self, image: &ImageRaw<Rgb565, LittleEndian>) {
        image.draw(&mut self.inner).unwrap();
    }
}

type DisplayInner<'a, T, M> =
    ST7735<ExclusiveDevice<Spi<'a, T, M>, Output<'a>, NoDelay>, Output<'a>, Output<'a>>;
