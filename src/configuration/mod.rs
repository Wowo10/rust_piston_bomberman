static mut amount: u8 = 0;

pub fn set(value: u8) {
    unsafe {
        amount = value;
    }
}

pub fn read() -> u8 {
    unsafe { amount }
}
