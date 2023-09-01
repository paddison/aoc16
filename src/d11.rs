// state is represented with matrix:
// 4 rows and num elements * 2 cols
// generators and chips are always next to each other
// generators are at even, chips at odd numbers.
// To determine the value of the state,
// take the sum of the number of items * the floor number

use std::collections::{ VecDeque, HashMap };

// solve in reverse order
macro_rules! impl_new_state {
    ($n:expr, $f:ident) => {
        #[allow(dead_code)]
        impl State<$n> {
            fn new() -> Self {
                State { floors: $f, level: 3 }
            } 
        }
    };
}

static MOVE_UP: usize = usize::MAX; // overflow to subtract one
static MOVE_DOWN: usize = 1;

const TEST_INPUT: [[u32; 4]; 4] = [
                [0, 0, 0, 0],
                [0, 0, 1, 0],
                [1, 0, 0, 0],
                [0, 1, 0, 1],
            ];

const INPUT: [[u32; 10]; 4] = [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 1, 0, 1, 0, 1],
                [0, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                [1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            ];

const INPUT_14: [[u32; 14]; 4] = [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0],
                [0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0],
                [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
            ];

pub(crate) fn get_solution_1() -> usize {
    find_cheapest_bfs(State::<10>::new(), &mut HashMap::new()) 
}
 
pub(crate) fn get_solution_2() -> usize {
    find_cheapest_bfs(State::<14>::new(), &mut HashMap::new()) 
}

// Generators are at even columns, chips at odd.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct State<const N: usize> {
    floors: [[u32; N]; 4],
    level: usize,
}

impl_new_state!(4, TEST_INPUT);
impl_new_state!(10, INPUT);
impl_new_state!(14, INPUT_14);

impl<const N: usize> State<N> {
    fn can_move_items(self, item_1: usize, item_2: Option<usize>, direction: usize) -> bool {
        // if we're at the top level
        if direction == MOVE_UP && self.level == 0 {
                return false;
        }
        // if we're at the bottom level
        if direction == MOVE_DOWN && self.level == 3 {
                return false;
        }
        // if there is no item at this position, don't move
        if self.floors[self.level][item_1] == 0 {
            return false;
        }
        // if we want to move two items
        if let Some(item_2) = item_2 {
            // if there is no item at this position
            if self.floors[self.level][item_2] == 0 {
                return false;
            }
            // if item_1 is generator and item_2 is not generator and item_2 is chip not belonging to that generator)
            if item_1 % 2 == 0 && item_2 % 2 != 0 && item_2 - item_1 != 1 {
                return false;
            }
        }
        // we can move the item
        true
    }

    fn all_floors_below_elevator_empty(&self) -> bool {
        self.floors.iter().rev().take(3 - self.level).all(|f| f.iter().all(|e| e == &0))
    }

    fn move_item(mut self, item_1: usize, item_2: Option<usize>, direction: usize) -> Self {
        self.floors[self.level][item_1] = 0;
        if let Some(item_2) = item_2 {
            self.floors[self.level][item_2] = 0;
        }
        self.level = self.level.overflowing_add(direction).0;
        self.floors[self.level][item_1] = 1;
        if let Some(item_2) = item_2 {
            self.floors[self.level][item_2] = 1;
        }
        self
    }

    fn is_valid_state(&self) -> bool {
        // if one generator on level -> every chip on level needs its generator
        // if there is no generator or no chip, it is valid by default
        self.floors.iter().all(|f| {
            if Self::has_generator(f) {
                !Self::any_chips_fried(f)
            } else {
                true
            }
        })
    }

    #[inline(always)]
    fn has_generator(floor: &[u32]) -> bool {
        floor.iter().step_by(2).any(|g| g == &1)
    }
    
    #[inline(always)]
    fn any_chips_fried(floor: &[u32]) -> bool {
        floor.iter()
            .step_by(2)
            .zip(floor.iter().skip(1).step_by(2))
            .any(|(g, c)| g == &0 && c == &1)
    }
    
    #[inline(always)]
    fn is_done(&self) -> bool {
        self.floors[0].iter().all(|e| e == &1)
    }
}

// Use Breadth First Search to find the minimum steps
fn find_cheapest_bfs<const N: usize>(state: State<N>, visited: &mut HashMap<State<N>, usize>) -> usize {
    let mut queue = VecDeque::from([(state, 0)]);
    while let Some((state, steps)) = queue.pop_front() {
        if let Some(best_steps) = visited.get(&state) {
            if steps >= *best_steps {
                continue;
            }
        } else {
            visited.insert(state, steps);
        }

        if state.is_done() {
            return steps;
        }

        if !move_two_items(state, steps, &mut queue, MOVE_UP) { 
            move_one_item(state, steps, &mut queue, MOVE_UP);
        }

        if state.all_floors_below_elevator_empty() {
            continue;
        }

        if !move_one_item(state, steps, &mut queue, MOVE_DOWN) { 
            move_two_items(state, steps, &mut queue, MOVE_DOWN);
        }
    }
    unreachable!();
}

// Check if we can create a new state from moving the two specified items
fn verify_next<const N: usize>(state: State<N>, item_1: usize, item_2: Option<usize>, direction: usize) -> Option<State<N>> {
    if !state.can_move_items(item_1, item_2, direction) {
        return None;
    }
    let new_state = state.move_item(item_1, item_2, direction);
    if !new_state.is_valid_state() {
        return None;
    }

    Some(new_state)
}

fn move_one_item<const N: usize>(state: State<N>, steps: usize, queue: &mut VecDeque<(State<N>, usize)>, direction: usize) -> bool {
    let mut moved = 0;
    // try to move 
    for i in [0, 1] {
        for item_1 in (i..N).step_by(2) {
            if let Some(state) = verify_next(state, item_1, None, direction) {
                queue.push_back((state, steps + 1));
                moved += 1;
                break;
            }
        }
    }
    moved > 0
}

fn move_two_items<const N: usize>(state: State<N>, steps: usize, queue: &mut VecDeque<(State<N>, usize)>, direction: usize) -> bool {
    // move generator and chip
    let mut moved = 0;

    for i1 in 0..N - 1 {
        for i2 in i1 + 1..N {
            if let Some(state) = verify_next(state, i1, Some(i2), direction) {
                queue.push_back((state, steps + 1));
                moved += 1;
            }
        }
    }
    moved > 0
}


#[test]
fn find_cheapest() {
    let state = State::<14>::new();
    let res = find_cheapest_bfs(state, &mut HashMap::new());
    println!("res: {res}");
}



#[cfg(test)]
mod test {
    use crate::d11::State;

    #[test]
    fn floor_is_valid_no_generators() {
        let floor = [0, 1, 0, 1, 0, 0];
        assert!(!State::<6>::has_generator(&floor));
    }

    #[test]
    fn floor_is_valid_one_generator() {
        let floor = [0, 0, 1, 1, 0, 0];
        assert!(!State::<6>::any_chips_fried(&floor));
    }

    #[test]
    fn floor_is_valid_two_generators() {
        let floor = [0, 0, 1, 1, 1, 0];
        assert!(!State::<6>::any_chips_fried(&floor));
    }

    #[test]
    fn floor_is_invalid() {
        let floor = [0, 0, 1, 1, 0, 1];
        assert!(State::<6>::any_chips_fried(&floor));
    }

    #[test]
    fn state_is_valid() {
        let s = State::<4> {
            floors: [
                [0, 0, 0, 0],
                [0, 1, 0, 1],
                [1, 1, 0, 0],
                [0, 0, 1, 0],
            ],
            level: 3,
        };
        assert!(s.is_valid_state())
    }

    #[test]
    fn state_is_invalid() {
        let s = State::<4> {
            floors: [
                [0, 0, 0, 0],
                [0, 1, 0, 1],
                [1, 1, 0, 0],
                [1, 0, 0, 1],
            ],
            level: 3,
        };
        assert!(!s.is_valid_state())
    }
}
