use std::{time::Duration, sync::{Arc, RwLock}, thread, pin, borrow::BorrowMut};

use crate::{BmbeGpioOut, BmbeDriverErr};

pub struct BmbePwm {
    // 频率
    frequency: u128,
    // 占空比
    duty: f64,
    // 高电平周期
    high_period: Arc<RwLock<u128>>,
    // 低电平周期
    low_period: Arc<RwLock<u128>>,
    // pwm正在运行
    pwm_active: Arc<RwLock<bool>>,
    
    // 输出电平
    out_pin: Arc<RwLock<BmbeGpioOut>>
}

impl BmbePwm {
    pub fn new(pin_num:u64,frequency: u128, duty: f64) -> Result<BmbePwm,BmbeDriverErr> {
        let pin_out = BmbeGpioOut::new(pin_num)?;
        let mut pwm = BmbePwm {
            frequency: 0,
            duty: 0.0,
            high_period: Arc::new(RwLock::new(0)),
            low_period: Arc::new(RwLock::new(0)),
            pwm_active: Arc::new(RwLock::new(true)),
            out_pin:Arc::new(RwLock::new(pin_out))
        };
        pwm.change_frequency_duty(frequency, duty);
        Ok(pwm)
    }

    pub fn start(&mut self) {
        *self.pwm_active.write().unwrap() = true;
        let pwm_active = self.pwm_active.clone();
        let high_period = self.high_period.clone();
        let low_period = self.low_period.clone();
        let out_pin = self.out_pin.clone();
        thread::spawn(move || {
            out_pin.write().unwrap().set_low();
            while *pwm_active.read().unwrap() {
                out_pin.write().unwrap().set_low();
                thread::sleep(Duration::from_nanos(*high_period.read().unwrap() as u64));
                out_pin.write().unwrap().set_high();
                thread::sleep(Duration::from_nanos(*low_period.read().unwrap() as u64));
            }
            out_pin.write().unwrap().set_low();
        });
    }

    pub fn stop(&self) {
        *self.pwm_active.write().unwrap() = false;
        println!("stop pwm");
    }

    fn count_period(&self) {
        // 一秒钟换成纳秒
        let one_sec_nanos = Duration::from_secs(1).as_nanos();
        // 一个时间周期 
        let mut peroid = one_sec_nanos.clone();

        if self.frequency > 0 {
            peroid = one_sec_nanos / self.frequency;
        }else{
            peroid = 0;
        }
        let mut high_peroid = peroid.clone();
        if self.duty > 0.0 {
            high_peroid = (peroid as f64 * self.duty) as u128;
        }else{
            high_peroid = 0;
        }
        println!("higth_priod:{}",high_peroid);
        let low_period = peroid.clone() - high_peroid;
        *self.high_period.write().unwrap() = high_peroid;
        *self.low_period.write().unwrap() = low_period;
    }

    pub fn change_duty(&mut self, duty: f64) {
        self.duty = duty;
        self.count_period();
    }
    pub fn change_frequency(&mut self, frequency: u128) {
        self.frequency = frequency;
        self.count_period();
    }

    pub fn change_frequency_duty(&mut self, frequency: u128, duty: f64) {
        self.frequency = frequency;
        self.duty = duty;
        self.count_period();
    }
}
