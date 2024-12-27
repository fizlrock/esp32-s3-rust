use anyhow::Context;

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{
        adc::{
            attenuation::DB_11,
            oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver},
            Resolution, ADC2,
        },
        gpio::Gpio12,
        prelude::Peripherals,
    },
    wifi::EspWifi,
};

use esp_idf_hal::{gpio::AnyInputPin, pcnt::*};

use esp_idf_sys::nvs_flash_init;
use log::info;

use crate::{freq_meter::FreqMeter, led::WS2812RMT, wifi};

pub struct Devices<'a> {
    pub led: WS2812RMT<'a>,
    pub wifi: EspWifi<'a>,
    pub adc_pin: AdcChannelDriver<'a, Gpio12, AdcDriver<'a, ADC2>>,
    pub pcnt: PcntDriver<'a>,
    pub freq_meter: FreqMeter<'a>,
}

pub fn init_device() -> Devices<'static> {
    // Basic init
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("System started!");

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();

    // Счетчик

    info!("Инициализация счетчика");
    let mut counter = PcntDriver::new(
        peripherals.pcnt0,
        Some(peripherals.pins.gpio47),
        Option::<AnyInputPin>::None,
        Option::<AnyInputPin>::None,
        Option::<AnyInputPin>::None,
    )
    .context("Failed to init PcntDriver")
    .unwrap();

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

    // Freq metter

    info!("Инициализация измерителя частоты");
    let freq_meter = FreqMeter::new(peripherals.pcnt1, peripherals.pins.gpio20);

    // Init status led

    info!("Инициализация RGB светодиода");
    let status_led_pin = peripherals.pins.gpio48;
    let channel = peripherals.rmt.channel0;
    let mut status_led = WS2812RMT::new(status_led_pin, channel).unwrap();

    status_led.set(0, 0, 100);

    info!("Инициализация WiFi");
    // Init wifi
    unsafe { nvs_flash_init() };
    let _wifi = wifi::wifi("SpecialForYou", "GtuuhHI7Gg", peripherals.modem, sysloop).unwrap();
    // let _wifi = wifi::wifi("Redmi", "12345678", peripherals.modem, sysloop).unwrap();

    // Init ADC
    info!("Инициализация ADC");

    let adc = AdcDriver::new(peripherals.adc2).unwrap();

    // 0 dB - 1.1 V
    // 2.5 dB - 1.5 V
    // 6 dB - 2.2 V
    // 11 dB - 3.1 V

    // 12 bit = 4096 LSB
    //  Погрешность +- 4 LSB ~ 0.004 V ~ 4 mV

    let config = AdcChannelConfig {
        attenuation: DB_11,
        resolution: Resolution::Resolution12Bit,
        calibration: true,
    };
    let adc_pin = AdcChannelDriver::new(adc, peripherals.pins.gpio12, &config).unwrap();

    // Измените GPIO на нужный

    // Настройка счетчика импульсов
    // pcnt.set_filter_value(10)?; // Фильтрация дребезга (если нужно)
    // pcnt.set_count_mode(PcntCountMode::RisingEdge)?; // Подсчет на фронтах
    // pcnt.clear()?;

    Devices {
        led: status_led,
        wifi: *_wifi,
        adc_pin,
        pcnt: counter,
        freq_meter: freq_meter,
    }
}
