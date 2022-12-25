use bmbe_driver::{BmbePwm, BmbeGpioOut, BmbeDriverErr};

pub struct BmbeCarWhell{
    frequency: u128,
    duty:f64,
    front_wheel_pwm: BmbePwm,
    front_left_wheel_pin_ain1:BmbeGpioOut,
    front_right_wheel_pin_ain2:BmbeGpioOut,
    back_wheel_pwm:BmbePwm,
    back_left_wheel_pin_bin1:BmbeGpioOut,
    back_right_wheel_pin_bin2:BmbeGpioOut,
}

impl BmbeCarWhell{
    pub fn new()->Result<Self,BmbeDriverErr>{
        let front_wheel_pwm = BmbePwm::new(18,0,0.0)?;
        let front_left_wheel_pin_ain1 = BmbeGpioOut::new(22)?;
        let front_right_wheel_pin_ain2 = BmbeGpioOut::new(27)?;
        let back_wheel_pwm = BmbePwm::new(23,0,0.0)?;
        let back_left_wheel_pin_bin1 = BmbeGpioOut::new(25)?;
        let back_right_wheel_pin_bin2 = BmbeGpioOut::new(24)?;
        let wheel =  BmbeCarWhell{
            frequency: 0u128,
            duty: 0.0,
            front_wheel_pwm, //  18
            front_left_wheel_pin_ain1, // 22
            front_right_wheel_pin_ain2, // 27
            back_wheel_pwm, // 23
            back_left_wheel_pin_bin1, // 25
            back_right_wheel_pin_bin2, //24
        };
        Ok(wheel)
    }
}


impl BmbeCarWhell{
    
    // hlhl 前进
    pub fn go_go(&mut self,frequncy:u128,duty:f64)->Result<(),BmbeDriverErr>{
        self.front_wheel_pwm.change_frequency_duty(frequncy, duty);
        self.front_wheel_pwm.start();
        self.back_wheel_pwm.change_frequency_duty(frequncy, duty);
        self.back_wheel_pwm.start();

        self.front_left_wheel_pin_ain1.set_high(); // 左侧正转
        self.front_right_wheel_pin_ain2.set_low();
        self.back_left_wheel_pin_bin1.set_high();  // 右侧正转
        self.back_right_wheel_pin_bin2.set_low();        
        Ok(()) 
    }

    
 
    pub fn go_back(&mut self,frequncy:u128,duty:f64)->Result<(),BmbeDriverErr>{
        self.front_wheel_pwm.change_frequency_duty(frequncy, duty);
        self.front_wheel_pwm.start();
        self.back_wheel_pwm.change_frequency_duty(frequncy, duty);
        self.back_wheel_pwm.start();

        self.front_right_wheel_pin_ain2.set_high(); // 左侧倒转
        self.front_left_wheel_pin_ain1.set_low();        
        self.back_right_wheel_pin_bin2.set_high();  // 右侧倒转   
        self.back_left_wheel_pin_bin1.set_low();  
             
        Ok(()) 
    }

    
    pub fn go_left(&mut self,frequncy:u128,duty:f64)->Result<(),BmbeDriverErr>{
        self.front_wheel_pwm.change_frequency_duty(frequncy, duty);
        self.front_wheel_pwm.start();
        self.back_wheel_pwm.change_frequency_duty(frequncy, duty);
        self.back_wheel_pwm.start();
        self.front_right_wheel_pin_ain2.set_high(); // 左侧倒转
        self.front_left_wheel_pin_ain1.set_low();        
        self.back_left_wheel_pin_bin1.set_high();  // 右侧正转 
        self.back_right_wheel_pin_bin2.set_low();    
        Ok(()) 
    }

  
    pub fn go_right(&mut self,frequncy:u128,duty:f64)->Result<(),BmbeDriverErr>{
        self.front_wheel_pwm.change_frequency_duty(frequncy, duty);
        self.front_wheel_pwm.start();
        self.back_wheel_pwm.change_frequency_duty(frequncy, duty);
        self.back_wheel_pwm.start();
        self.front_left_wheel_pin_ain1.set_high(); // 左侧正转
        self.front_right_wheel_pin_ain2.set_low();       
        self.back_right_wheel_pin_bin2.set_high();  // 右侧倒转   
        self.back_left_wheel_pin_bin1.set_low();  
        Ok(()) 
    }
    pub fn stop(&mut self)->Result<(),BmbeDriverErr>{
        self.front_wheel_pwm.change_frequency_duty(0, 0.0);
        self.front_wheel_pwm.stop();
        self.front_left_wheel_pin_ain1.set_low();
        self.front_right_wheel_pin_ain2.set_low();

        self.back_wheel_pwm.change_frequency_duty(0, 0.0);
        self.back_wheel_pwm.stop(); 
        self.back_left_wheel_pin_bin1.set_low();
        self.back_right_wheel_pin_bin2.set_low();  
        Ok(()) 
    }


    pub fn go_test(&mut self,frequncy:u128,duty:f64)->Result<(),BmbeDriverErr>{
        self.go_right(frequncy, duty)
    }
}