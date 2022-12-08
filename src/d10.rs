use std::collections::HashMap;

const N_BOTS: usize = 210;
const N_OUTS: usize = 21;

type ValueInstr = (usize, usize); // (bot, val)
type BotInstr = HashMap<usize, (Target, Target)>; // (bot, low, high)
type Bot = [Option<usize>; 2];

fn parse(input: &str) -> (Vec<ValueInstr>, BotInstr) {
    let mut values = Vec::new();
    let mut bots = HashMap::new();

    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["value", val, _, _, "bot", bot, ..] => values.push((bot.parse().unwrap(), val.parse().unwrap())),
            ["bot", bot, _, _, _, low_target, lt_id, _, _, _, high_target, ht_id, ..] => {
                let (bot, lt_id, ht_id) = (
                    bot.parse().unwrap(), 
                    lt_id.parse().unwrap(),
                    ht_id.parse().unwrap()
                );
                let (id, low, high) = Target::from_parts(low_target, high_target, bot, lt_id, ht_id);
                assert!(!bots.contains_key(&id));
                bots.insert(id, (low, high));
            }, 
            _ => unreachable!()
        }
    }

    (values, bots)
}

fn execute_value_instr(instructions: Vec<ValueInstr> ) -> [Bot; N_BOTS] {
    let mut bots = [[None, None]; N_BOTS];
    
    for (id, val) in instructions {
        let vals = bots.get_mut(id).unwrap();
        match vals {
            [None, None] => vals[0] = Some(val),
            _ => vals[1] = Some(val)
        }
    }
    
    bots
}

fn execute_bot_instr(bots: &mut [Bot], bot_instr: &mut BotInstr, outputs: &mut [Option<usize>]) -> (usize, Bot) {
    let (id, mut next_values) = bots.iter()
                                    .enumerate()
                                    .find(|(_, [l, r])| l.is_some() && r.is_some())
                                    .map(|(idx, vals)| (idx, vals.clone()))
                                    .unwrap();
    bots[id] = [None, None];
    next_values.sort();
    
    match bot_instr.remove(&id).unwrap() {
        (Target::Bot(id_l), Target::Bot(id_r)) => (add_value(bots, id_l, next_values[0]), add_value(bots, id_r, next_values[1])),
        (Target::Bot(id_l), Target::Output(id_r)) => (add_value(bots, id_l, next_values[0]), outputs[id_r] = next_values[1]),
        (Target::Output(id_l), Target::Bot(id_r)) => (outputs[id_l] = next_values[0], add_value(bots, id_r, next_values[1])),
        (Target::Output(id_l), Target::Output(id_r)) => (outputs[id_l] = next_values[0], outputs[id_r] = next_values[1]),
    };

    (id, next_values)
}

fn add_value(bots: &mut [Bot], id: usize, val: Option<usize>) {
    let vals = bots.get_mut(id).unwrap();
    match vals {
        [Some(_), None] => vals[1] = val,
        _ => vals[0] = val,
    }
}

pub fn get_solution_1() -> usize {
    let (val_instr, mut bot_instr) = parse(include_str!("../data/d10.txt"));
    let mut bots = execute_value_instr(val_instr);
    let mut outputs = [None; N_OUTS];

    while !bot_instr.is_empty() {
        if let (id, [Some(17), Some(61)]) = execute_bot_instr(&mut bots, &mut bot_instr, &mut outputs) {
            return id;
        }
        
    }

    unreachable!();
}

pub fn get_solution_2() -> usize {
    let (val_instr, mut bot_instr) = parse(include_str!("../data/d10.txt"));
    let mut bots = execute_value_instr(val_instr);
    let mut outputs = [None; N_OUTS];

    while !bot_instr.is_empty() {
        let _ = execute_bot_instr(&mut bots, &mut bot_instr, &mut outputs);
    } 

    outputs[0..3].iter().map(|n| n.unwrap()).product::<usize>()
}

enum Target {
    Bot(usize),     // id
    Output(usize),  // id
}

impl Target {
    fn from_parts(low: &str, high: &str, id: usize, lt_id: usize, ht_id: usize) -> (usize, Self, Self) {
        let (low, high) = match (low, high) {
            ("bot", "bot") => (Self::Bot(lt_id), Self::Bot(ht_id)),
            ("bot", "output") => (Self::Bot(lt_id), Self::Output(ht_id)),
            ("output", "bot") => (Self::Output(lt_id), Self::Bot(ht_id)),
            ("output", "output") => (Self::Output(lt_id), Self::Output(ht_id)), 
            _ => unreachable!()
        };

        (id, low, high)
    }
}