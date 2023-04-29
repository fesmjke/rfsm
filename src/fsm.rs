#[derive(Debug)]
pub enum State {
    Locked,
    Unlocked,
}

pub enum Action {
    Push,
    Coin,
}

pub struct Turnstile {
    pub state: State,
}

impl Turnstile {
    fn new() -> Turnstile {
        Self {
            state: State::Locked,
        }
    }

    fn next(&mut self, action: Action) -> String {
        match action {
            Action::Push => match self.state {
                State::Locked => {
                    return String::from("LOCKED");
                }
                State::Unlocked => {
                    self.state = State::Locked;
                    return String::from("PASS");
                }
            },
            Action::Coin => match self.state {
                State::Locked => {
                    self.state = State::Unlocked;
                    return String::from("UNLOCKED");
                }
                State::Unlocked => {
                    return String::from("COIN");
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turnstile_is_locked_push() {
        let mut turnstile = Turnstile::new();

        assert_eq!("LOCKED", turnstile.next(Action::Push));
    }

    #[test]
    fn turnstile_is_unlocked_push() {
        let mut turnstile = Turnstile::new();

        turnstile.state = State::Unlocked;

        assert_eq!("PASS", turnstile.next(Action::Push));
    }

    #[test]
    fn turnstile_is_locked_coin() {
        let mut turnstile = Turnstile::new();

        turnstile.state = State::Locked;

        assert_eq!("UNLOCKED", turnstile.next(Action::Coin));
    }

    #[test]
    fn turnstile_is_unlocked_coin() {
        let mut turnstile = Turnstile::new();

        turnstile.state = State::Unlocked;

        assert_eq!("COIN", turnstile.next(Action::Coin));
    }
}
