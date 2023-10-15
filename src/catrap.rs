

use pathfinding::prelude::astar;

use super::catrap::state::State;

mod state;
mod block;
mod stage;
mod direction;


pub fn solve_catrap( state: &State ) -> Option<Vec<State>> {

    let successors = |st:&State|{
        st.reachable_states_one_move().into_iter().map( |s| (s,1) )
    };
    let heuristic = |_st:&State| 0;
    let success = |st:&State| st.ghosts_count() == 0;
    let ret = astar( state, successors, heuristic, success );

    ret.map( |(states,_cost)| states )
}


#[cfg(test)]
mod tests {
    use crate::catrap::solve_catrap;
    use super::state::State;

    #[test]
    fn simple_test_only_right(){
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let strings = vec![
            "WWWWWWWWW",
            "W@ G    W",
            "WWWW    W",
            "W       W",
            "WWWWWWWWW",
        ];
        let state = State::from_lines(strings);

        match solve_catrap(&state){
            None => assert!(false),
            Some(states) => {
                states.iter().for_each( |s| s.dump_stdout() );
            }
        }
    }

    #[test]
    fn simple_test_two_ghosts(){
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let strings = vec![
            "WWWWWWWWW",
            "W@ G    W",
            "WWWW    W",
            "WG      W",
            "WWWWWWWWW",
        ];
        let state = State::from_lines(strings);

        match solve_catrap(&state){
            None => assert!(false),
            Some(states) => {
                states.iter().for_each( |s| s.dump_stdout() );
            }
        }
    }

    #[test]
    fn level_80(){
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let strings = vec![
            "@==========",
            "**********=",
            "**FFR*RFR*=",
            "**F*F*F*F*=",
            "**RRR*R*R*=",
            "**F*F*F*F*=",
            "**RRF*FFR*=",
            "**********="
        ];
        let state = State::from_lines(strings);
        state.dump_stdout();

        match solve_catrap(&state){
            None => assert!(false),
            Some(states) => {
                states.iter().for_each( |s| s.dump_stdout() );
            }
        }
    }

}
