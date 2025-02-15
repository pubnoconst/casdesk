use std::rc::Rc;

pub mod renderer;

trait ShortString {
    fn cut_30(&self) -> String;
}

impl ShortString for &str {
    fn cut_30(&self) -> String {
        self.chars().take(30).collect()
    }
}

trait LongString {
    fn cut_80(&self) -> String;
}

impl LongString for &str {
    fn cut_80(&self) -> String {
        self.chars().take(100).collect()
    }
}

#[derive(Debug)]
pub struct Customer {
    name: Rc<str>,
    contact: Rc<str>,
    address: Rc<str>,
    id_num: Rc<str>
}

impl Customer {
    pub fn new(name: &str, contact: &str, address: &str, id_num: &str) -> Self {
        Customer {
            name: name.cut_30().into(),
            contact: contact.cut_30().into(),
            address: address.cut_80().into(),
            id_num: id_num.cut_30().into()
        }
    }
}

#[derive(Debug)]
pub struct SellableDevice {
    name: Rc<str>,
    color: Rc<str>,
    locked: Rc<str>,
    imei: Rc<str>,
    price: Rc<str>,
}

impl SellableDevice {
    pub fn new(name: &str, color: &str, locked: &str, imei: &str, price: &str) -> Self {
        Self {
            name: name.cut_30().into(),
            color: color.cut_30().into(),
            locked: locked.cut_30().into(),
            imei: imei.cut_30().into(),
            price: price.cut_30().into(),
        }
    }
}

#[derive(Debug)]
pub struct PurchasedDevice {
    name: Rc<str>,
    color: Rc<str>,
    memory: Rc<str>,
    locked: Rc<str>,
    imei: Rc<str>,
}

impl PurchasedDevice {
    pub fn new(name: &str, color: &str, memory: &str, locked: &str, imei: &str) -> Self {
        Self {
            name: name.cut_30().into(),
            color: color.cut_30().into(),
            memory: memory.cut_30().into(),
            locked: locked.cut_30().into(),
            imei: imei.cut_30().into(),
        }
    }
}

#[derive(Debug)]
pub struct LeasedDevice {
    name: Rc<str>,
    color: Rc<str>,
    locked: Rc<str>,
    imei: Rc<str>,
    accessories: Rc<str>,
    condition: Rc<str>
}

impl LeasedDevice {
    pub fn new(name: &str, color: &str, locked: &str, imei: &str, accessories: &str, condition: &str) -> Self {
        Self {
            name: name.cut_30().into(),
            color: color.cut_30().into(),
            locked: locked.cut_30().into(),
            imei: imei.cut_30().into(),
            accessories: accessories.cut_30().into(),
            condition: condition.cut_30().into()
        }
    }
}


