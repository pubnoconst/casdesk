use std::rc::Rc;

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

pub struct Customer {
    name: Rc<str>,
    contact: Rc<str>,
    address: Rc<str>,
    id_num: Rc<str>
}

impl Customer {
    pub fn new(name: &str, contact: &str, address: &str, id_num: &str) -> Customer {
        Customer {
            name: name.cut_30().into(),
            contact: contact.cut_30().into(),
            address: address.cut_80().into(),
            id_num: id_num.cut_30().into()
        }
    }
}