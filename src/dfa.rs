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
        //        b
        // --------------<
        // |             |
        // |   a     a   |
        // 0 ---> 1 ---> 2
        // |             |
        // |      c      |
        // >--------------
        let zero_to_one = Transition::from(String::from("0"), String::from("1"), 'a');
        let zero_to_two = Transition::from(String::from("0"), String::from("2"), 'c');
        let one_to_two = Transition::from(String::from("1"), String::from("2"), 'a');
        let two_to_zero = Transition::from(String::from("2"), String::from("0"), 'b');

        let mut state = State::new(String::from("0"), '_');

        state.add_transition(zero_to_one);
        state.add_transition(zero_to_two);
        state.add_transition(one_to_two);
        state.add_transition(two_to_zero);

        state.next('b');

        assert_eq!(state.state, "0");

        state.next('a');

        assert_eq!(state.state, "1");

        state.next('a');

        assert_eq!(state.state, "2");
    }

    #[test]
    fn generic_result() {
        //        b
        // --------------<
        // |             |
        // |   a     a   |
        // 0 ---> 1 ---> 2
        // |             |
        // |      c      |
        // >--------------
        let zero_to_one = Transition::from(String::from("0"), String::from("1"), 'a');
        let zero_to_two = Transition::from(String::from("0"), String::from("2"), 'c');
        let one_to_two = Transition::from(String::from("1"), String::from("2"), 'a');
        let two_to_zero = Transition::from(String::from("2"), String::from("0"), 'b');

        let mut state = State::new(String::from("0"), '_');

        state.add_transition(zero_to_one);
        state.add_transition(zero_to_two);
        state.add_transition(one_to_two);
        state.add_transition(two_to_zero);

        match state.next('a') {
            Ok(answer) => println!("{:?}", answer),
            Err(why) => println!("{:?}",why),
        }
    }
}