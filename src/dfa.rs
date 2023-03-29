use std::vec;

#[derive(Debug,PartialEq,Clone, Copy)]
enum Action {
    Coin,
    Push
}

#[derive(Debug,Clone,PartialEq)]
enum Trunstile {
    Locked,
    Unlocked
}
#[derive(Debug)]
enum TransitionError {
    MissingCondition,
    Unknown
}

#[derive(Debug)]
struct Transition<T, A>{
    from: T,
    to: T,
    condition: A // todo
}

impl<T, A> Transition<T, A>  where T: Clone {
    fn from(fst : T, scd: T, condition: A) -> Transition<T, A>  {        
        Self {
            from : fst,
            to : scd,
            condition, // todo
        }
    }
}

struct State<T, A> {
    state: T,
    action: A,
    transitions: Vec<Transition<T, A>>,
    table: Vec<Vec<T>> // todo
}

impl<T: Clone + std::cmp::PartialEq + std::fmt::Debug, A: std::cmp::PartialEq + Copy + std::fmt::Debug> State<T, A> {
    fn new(init_state : T, init_action: A) -> State<T, A> {
        Self {
            state : init_state,
            action : init_action,
            transitions : vec![],
            table: vec![]
        }
    }

    fn add_transition(&mut self, transition: Transition<T, A>) {
        self.transitions.push(transition);
    }

    fn build_table(&self) {  
        todo!();
    }

    fn next(&mut self, input : A) -> Result<String, TransitionError> {
        let conditions : Vec<_> = self.transitions.iter().map(|tr| tr.condition).collect();

        if !conditions.contains(&input) {
            return Err(TransitionError::MissingCondition);
        }

        for transitions in self.transitions.iter() {
            if transitions.from == self.state && transitions.condition == input {
                self.state = transitions.to.clone();
                return Ok(format!("state changed to {:?}", self.state));
            }
        }

        return Err(TransitionError::Unknown);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trunstile_generic() {
        //    C - coin
        // L --> U
        //    P - push
        // L --> L 
        //    P - push
        // U --> L
        //    C - coin
        // U --> U

        let f_t = Transition::from(Trunstile::Locked, Trunstile::Unlocked, Action::Coin);
        let s_t = Transition::from(Trunstile::Unlocked, Trunstile::Locked, Action::Push);

        let mut state = State::new(Trunstile::Locked,Action::Push);

        state.add_transition(f_t);
        state.add_transition(s_t);

        assert_eq!(state.state, Trunstile::Locked);
    }

    #[test]
    fn generic() {
        let f = Transition::from(String::from("0"), String::from("1"), 'a');
        let s = Transition::from(String::from("1"), String::from("2"), 'a');
        let t = Transition::from(String::from("2"), String::from("1"), 'b');


        let mut state = State::new(String::from("0"), '_');

        todo!();
    }
}