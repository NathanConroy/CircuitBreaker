// Copyright 2021 Nathan Conroy All rights reserved.


enum State {
    Open,
    Closed,
}


pub struct CircuitBreaker {
    state: State,
}


pub fn build_cb() -> CircuitBreaker {
    CircuitBreaker {
        state: State::Open,
    }
}


pub fn run() {
    println!("Hello!");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

