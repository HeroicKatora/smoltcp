use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use super::{Device, DeviceCapabilities};

pub struct KillSwitch<P> {
    inner: P,
    switch: Rc<RefCell<Config>>,
}

#[derive(Clone)]
pub struct Switch {
    switch: Rc<RefCell<Config>>,
}

#[derive(Default)]
struct Config {
    no_rx: bool,
    no_tx: bool,
}

impl<P> KillSwitch<P> {
    pub fn new(device: P) -> Self {
        KillSwitch {
            inner: device,
            switch: Rc::default(),
        }
    }

    pub fn switch(&self) -> Switch {
        Switch {
            switch: self.switch.clone(),
        }
    }
}

impl Switch {
    pub fn kill_rx(&self, killed: bool) -> bool {
        core::mem::replace(&mut self.switch.borrow_mut().no_rx, killed)
    }

    pub fn kill_tx(&self, killed: bool) -> bool {
        core::mem::replace(&mut self.switch.borrow_mut().no_tx, killed)
    }
}

impl<'a, P> Device<'a> for KillSwitch<P>
    where P: Device<'a>
{
    type RxToken = P::RxToken;
    type TxToken = P::TxToken;

    fn receive(&'a mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        if self.switch.borrow().no_rx {
            None
        } else {
            self.inner.receive()
        }
    }

    fn transmit(&'a mut self) -> Option<Self::TxToken> {
        if self.switch.borrow().no_tx {
            None
        } else {
            self.inner.transmit()
        }
    }

    fn capabilities(&self) -> DeviceCapabilities {
        self.inner.capabilities()
    }
}

impl<P> Deref for KillSwitch<P> {
    type Target = P;

    fn deref(&self) -> &P {
        &self.inner
    }
}
