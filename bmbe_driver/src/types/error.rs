#[derive(Clone, Debug)]
pub enum BmbeDriverErrKind {
    Pwm,
    I2C,
    PinPwm,
    Pin,
}

#[derive(Clone, Debug)]
pub struct BmbeDriverErr {
    kind: BmbeDriverErrKind,
    msg: String,
}

impl BmbeDriverErr {
    pub fn new(msg: String, kind: BmbeDriverErrKind)->Self {
        BmbeDriverErr{
            msg,
            kind
        }
    }

    pub fn kind(&self) -> &BmbeDriverErrKind {
        &self.kind
    }

    pub fn msg(&self) -> &String {
        &self.msg
    }
}
