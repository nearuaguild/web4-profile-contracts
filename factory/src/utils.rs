use near_sdk::{
    env
};

pub fn assert_condition<S: AsRef<str>>(condition: bool, message: S) {
    if condition {
        return;
    }

    env::panic_str(message.as_ref());
}