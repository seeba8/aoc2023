extern crate core;

use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::AddAssign;
const INPUT: &str = include_str!("input.txt");
fn main() {
    let mut modules = parse(INPUT);
    let out = pulse(&mut modules, 1000);
    println!("Day 20 part 1: {}", out.high * out.low);
    let mut modules = parse(INPUT);
    let y = wait_for_rx(&mut modules);
    println!("Iterations: {y}");
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Output {
    high: usize,
    low: usize,
}

impl AddAssign for Output {
    fn add_assign(&mut self, rhs: Self) {
        self.high += rhs.high;
        self.low += rhs.low;
    }
}

impl Output {
    fn increment(&mut self, high: bool) {
        if high {
            self.high += 1;
        } else {
            self.low += 1;
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Module {
    Flipflop(Flipflop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
    Noop(String),
}

impl Module {
    fn add_destination(&mut self, destination: usize) {
        match self {
            Self::Flipflop(f) => {
                f.destinations.push(destination);
            }

            Self::Conjunction(c) => {
                c.destinations.push(destination);
            }
            Self::Broadcaster(b) => {
                b.destinations.push(destination);
            }
            Self::Noop(_) => { panic!("Destination cannot be set for noop") }
        }
    }

    fn pulse(&mut self, signal: &Signal, queue: &mut VecDeque<Signal>) -> Output {
        match self {
            Self::Flipflop(f) => {
                if signal.high {
                    return Output { high: 0, low: 0 };
                }
                f.state = !f.state;
                let mut output = Output { high: 0, low: 0 };
                for destination in &f.destinations {
                    queue.push_back(Signal::new(f.state, signal.destination, *destination));
                    output.increment(f.state);
                }
                output
            }
            Self::Conjunction(c) => {
                c.last_pulse.insert(signal.sender, signal.high);
                let o = !c.last_pulse.values().all(|v| *v);
                let mut output = Output { high: 0, low: 0 };
                for destination in &c.destinations {
                    queue.push_back(Signal::new(o, signal.destination, *destination));
                    output.increment(o);
                }
                output
            }
            Self::Broadcaster(b) => {
                let mut output = Output { high: 0, low: 0 };
                for dest in &b.destinations {
                    queue.push_back(Signal::new(signal.high, signal.destination, *dest));
                    output.increment(signal.high);
                }
                output
            }
            Self::Noop(_) => { Output { high: 0, low: 0 } }
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Flipflop(f) => &f.name,
            Self::Conjunction(c) => &c.name,
            Self::Broadcaster(_) => "broadcaster",
            Self::Noop(s) => s,
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Signal {
    high: bool,
    sender: usize,
    destination: usize,
}

impl Signal {
    const fn new(high: bool, sender: usize, destination: usize) -> Self {
        Self {
            high,
            sender,
            destination,
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq, Debug)]
struct Flipflop {
    destinations: Vec<usize>,
    name: String,
    state: bool,

}


#[derive(Clone, Default, Eq, PartialEq, Debug)]
struct Conjunction {
    destinations: Vec<usize>,
    last_pulse: HashMap<usize, bool>,
    name: String,
}


#[derive(Clone, Default, Eq, PartialEq, Debug)]
struct Broadcaster {
    destinations: Vec<usize>,
}

fn parse(input: &str) -> Vec<Module> {
    let mut out: Vec<Module> = vec![];
    let mut name_index: HashMap<String, usize> = HashMap::new();
    let mut conjunctions: HashSet<String> = HashSet::new();
    for (idx, line) in input.lines().enumerate() {
        let (left, _) = line.split_once(" -> ").unwrap();
        let name = left.trim_start_matches(['&', '%']).to_string();
        name_index.insert(name.clone(), idx);
        if left == "broadcaster" {
            out.push(Module::Broadcaster(Broadcaster::default()));
        } else if left.starts_with('%') {
            let f = Flipflop {
                name,
                ..Default::default()
            };
            out.push(Module::Flipflop(f));
        } else if left.starts_with('&') {
            conjunctions.insert(name.clone());
            let c = Conjunction {
                name,
                ..Default::default()
            };
            out.push(Module::Conjunction(c));
        }
    }
    // First run finished, now we can resolve the dependencies
    for line in input.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        let left = left.trim_start_matches(['%', '&']);
        for target in right.split(", ") {
            if !name_index.contains_key(target) {
                out.push(Module::Noop(target.to_string()));
                name_index.insert(target.to_string(), out.len() - 1);
            }
            out[name_index[left]].add_destination(name_index[target]);
            if conjunctions.contains(target) {
                let Module::Conjunction(c) = &mut out[name_index[target]] else { panic!() };
                c.last_pulse.insert(name_index[left], false);
            }
        }
    }
    out
}

#[allow(unused)]
fn display_signal(signal: &Signal, modules: &[Module]) -> String {
    format!("{} -{}-> {}", modules[signal.sender].name(), if signal.high { "high" } else { "low" }, modules[signal.destination].name())
}

fn pulse(modules: &mut [Module], count: usize) -> Output {
    let mut queue = VecDeque::new();
    let mut pulses = Output { low: 0, high: 0 };
    let broadcaster = modules.iter().position(|p| matches!(p, Module::Broadcaster(_))).unwrap();
    for _ in 0..count {
        queue.push_back(Signal::new(false, 0, broadcaster));
        pulses.low += 1;
        while let Some(signal) = queue.pop_front() {
            //println!("{}", display_signal(&signal, modules));
            pulses += modules[signal.destination].pulse(&signal, &mut queue);
        }
    }
    pulses
}


fn wait_for_rx(modules: &mut [Module]) -> usize {
    let mut queue = VecDeque::new();
    let broadcaster = modules.iter().position(|p| matches!(p, Module::Broadcaster(_))).unwrap();
    let rx = modules.iter().position(|p| p.name() == "rx").unwrap();
    let mut c = 0;
    loop {
        c += 1;
        queue.push_back(Signal::new(false, 0, broadcaster));
        while let Some(signal) = queue.pop_front() {
            if !signal.high && signal.destination == rx {
                return c;
            }
            //println!("{}", display_signal(&signal, modules));
            modules[signal.destination].pulse(&signal, &mut queue);
            // if queue.iter().any(|s| s.destination == rx && !s.high) {
            //     return c;
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example1.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");

    #[test]
    fn it_parses_input() {
        let modules = parse(EXAMPLE1);
        assert_eq!(modules.len(), 5);
        assert!(matches!(modules[0], Module::Broadcaster(_)));
        assert_eq!(modules.iter().position(|p| matches!(p, Module::Broadcaster(_))).unwrap(), 0);
        let Module::Broadcaster(b) = &modules[0] else { panic!() };
        assert_eq!(b.destinations.len(), 3);
        assert_eq!(b.destinations, vec![1, 2, 3]);
        let Module::Conjunction(c) = &modules[4] else { panic!() };
        assert_eq!(c.last_pulse.len(), 1);
    }

    #[test]
    fn it_pulses() {
        let mut modules = parse(EXAMPLE1);
        let out = pulse(&mut modules, 1);
        assert_eq!(out, Output {
            high: 4,
            low: 8,
        });
        let out = pulse(&mut modules, 1000);
        assert_eq!(out.high * out.low, 32_000_000);
    }

    #[test]
    fn it_parses_example2() {
        let modules = parse(EXAMPLE2);
        assert_eq!(modules.len(), 6);
    }

    #[test]
    fn it_pulses_example2() {
        let mut modules = parse(EXAMPLE2);
        assert_eq!(pulse(&mut modules, 1), Output {low: 4, high: 4});
        assert_eq!(pulse(&mut modules, 1), Output {low: 4, high: 2});
        assert_eq!(pulse(&mut modules, 1), Output {low: 5, high: 3});
        assert_eq!(pulse(&mut modules, 1), Output {low: 4, high: 2});
        let mut modules = parse(EXAMPLE2);
        let out = pulse(&mut modules, 1000);
        assert_eq!(out.high * out.low, 11_687_500);
    }
}