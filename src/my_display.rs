use ch32_hal::{gpio::{AnyPin, Level, Output}, mode::Mode, spi::{Instance, Spi}};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use st7735_lcd::ST7735;

pub struct MyDisplay<'a, T: Instance, M: Mode> {
    inner: ST7735<ExclusiveDevice<Spi<'a, T, M>, Output<'a>, NoDelay>, Output<'a>, Output<'a>>,
}

impl<'a, T: Instance, M: Mode> MyDisplay<'a, T, M> {
    fn new(
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
