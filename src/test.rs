// not sure how to test this without nocap, i'll have to update this in the future
#[cfg(test)]
mod inputs_test {
    use std::thread;

    use crate::input::{Input, Vk};

    #[test]
    fn input_test() {
        let mut input = Input::new();
        loop {
            thread::sleep(std::time::Duration::from_millis(100));
            input.update();
            if input.is_key_pressed(Vk::Escape) {
                break;
            }
            if input.is_key_pressed(Vk::A) {
                print!("A");
            }
            if input.is_key_pressed(Vk::B) {
                print!("B");
            }
            println!();
        }
    }
}
