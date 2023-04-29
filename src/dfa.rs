#[derive(Debug)]
enum TransitionError {
    MissingCondition,
    Unknown,
}

#[derive(Debug)]
struct Transition<T, A> {
    from: T,
    to: T,
    condition: A,
}

impl<T, A> Transition<T, A>
where
    T: Clone,
{
    fn from(fst: T, scd: T, condition: A) -> Transition<T, A> {
        Self {
            from: fst,
            to: scd,
            condition,
        }
    }
}

struct State<T, A> {
    state: T,
    action: A,
    transitions: Vec<Transition<T, A>>,
}

impl<T, A> State<T, A>
where
    T: std::cmp::PartialEq + Clone + std::fmt::Debug,
    A: std::cmp::PartialEq + Copy + std::fmt::Debug,
{
    fn new(init_state: T, init_action: A) -> State<T, A> {
        Self {
            state: init_state,
            action: init_action,
            transitions: vec![]
        }
    }

    fn add_transition(&mut self, transition: Transition<T, A>) {
        self.transitions.push(transition);
    }

    fn next(&mut self, input: A) -> Result<T, TransitionError> {
        let conditions: Vec<_> = self.transitions.iter().map(|tr| tr.condition).collect();

        if !conditions.contains(&input) {
            return Err(TransitionError::MissingCondition);
        }
        
        for transitions in self.transitions.iter() {
            if transitions.from == self.state && transitions.condition == input {
                self.state = transitions.to.clone();
                return Ok(self.state.clone());
            }
        }

        return Err(TransitionError::Unknown);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            Err(why) => println!("{:?}", why),
        }
    }
}
