use esp_idf_hal::{
    gpio::{AnyInputPin, InputPin},
    pcnt::{
        Pcnt, PcntChannel, PcntChannelConfig, PcntControlMode, PcntCountMode, PcntDriver, PinIndex,
    },
    peripheral::Peripheral,
};
use log::info;

pub struct FreqMeter<'a> {
    pcnt: PcntDriver<'a>,
    freq: i32,
}

impl<'a> FreqMeter<'a> {
    /// Провести расчет частоты в соотвествии с кол-во подстчитанных импульсов
    /// Функция должна вызыватся по таймеру через определенный промежуток времени
    pub fn measure_fr(&mut self) {
        let cv = self
            .pcnt
            .get_counter_value()
            .expect("Failed to get counter value");

        self.pcnt.counter_clear().expect("Failed to reset counter");

        let f = cv as i32;
        self.freq = f;
        // время в миллисекундах
        // Частота 1/c
    }

    pub fn get_fr(&mut self) -> i32 {
        self.freq
    }

    pub fn new<PCNT, PIN>(
        pcnt_unit: impl Peripheral<P = PCNT> + 'a,
        pin: impl Peripheral<P = PIN> + 'a,
    ) -> Self
    where
        PCNT: Pcnt + 'a,
        PIN: InputPin + 'a,
    {
        info!("Инициализация измерителя частоты");

        let mut counter = PcntDriver::new(
            pcnt_unit,
            Some(pin),
            Option::<AnyInputPin>::None,
            Option::<AnyInputPin>::None,
            Option::<AnyInputPin>::None,
        )
        .expect("Failed to init PcntDriver");

        counter
            .channel_config(
                PcntChannel::Channel0,
                PinIndex::Pin0,
                PinIndex::Pin1,
                &PcntChannelConfig {
                    lctrl_mode: PcntControlMode::Keep,
                    hctrl_mode: PcntControlMode::Keep,
                    pos_mode: PcntCountMode::Increment,
                    neg_mode: PcntCountMode::Hold,
                    counter_h_lim: 32767,
                    counter_l_lim: 0,
                },
            )
            .unwrap();

        counter.set_filter_value(1023).unwrap();
        counter.counter_clear().unwrap();
        counter.filter_enable().unwrap();
        counter.counter_resume().unwrap();

        let fm = FreqMeter {
            pcnt: counter,
            freq: 0,
        };

        fm
    }
}
