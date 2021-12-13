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

impl CircuitBreaker {
    pub fn tell_state(self) {
        match self.state {
            State::Open => println!("I'm open"),
            State::Closed => println!("I'm closed"),
        }
    }
}


pub fn run() {
    let cb = build_cb();
    cb.tell_state();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

