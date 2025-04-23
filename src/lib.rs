pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn type_of_test() {
        let x = 1991;
        println!("The type of {} is {}", x, type_of(&x));
    }
}

/// This function returns the type of the argument.
/// 
/// `<T>`                       Generic over any type T                 
/// `_:&T`                      Input is a reference to T, unused (_)   
/// `-> &'static str`           Return a string that lasts forever      
/// `type_name::<T>()`          Get the type name of T                  
/// 
/// "Give me anything. I don't care what it is. I just want to return its type name as text."
pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

/// Checks whether the Esc key was pressed
pub fn esc_key_pressed() -> bool {
    use crossterm::event::{self, Event, KeyCode};

    // poll(0) means it returns immediately with event availability information
    // using while instead of if to clear the pending events are processed (if many keys are pressed between ticks).
    while event::poll(std::time::Duration::from_millis(0)).unwrap() {
        match event::read().unwrap() {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Esc {
                    return true;
                }
            },
            _ => (),
        };
    }
    false
}