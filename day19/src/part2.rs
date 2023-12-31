use crate::{WorkflowEngine, Outcome, Category, Condition};

/// Ranges are exclusive ranges, not inclusive. 
/// That means, `(1, 4001)` contains the integers {1,2,3,...,4000}
#[derive(Clone, Debug)]
pub struct Range {
    pub x: (usize, usize),
    pub m: (usize, usize),
    pub a: (usize, usize),
    pub s: (usize, usize),
    pub outcome: Outcome,
}

impl Range {
    pub const fn product(&self) -> usize {
        (self.x.1 - self.x.0) 
        * (self.m.1 - self.m.0)
        * (self.a.1 - self.a.0)
        * (self.s.1 - self.s.0)
    }
    fn new(parent: Option<usize>, outcome: Outcome, ranges: &[Self]) -> Self {
        if let Some(p) = parent && let Some(p) = ranges.get(p) {
            Self {
                outcome,
                x: p.x,
                m: p.m,
                a: p.a,
                s: p.s,
            }
        } else {
            Self {
                outcome,
                x: (1,4001),
                m: (1,4001),
                a: (1,4001),
                s: (1, 4001),
            }
        }
        
    }

    fn apply(&mut self, c: Condition, inverse: &mut Self) {
        match c.op {
            crate::Operator::GreaterThan => {
                self.set(c.lhs,c.rhs + 1, 4001); 
                inverse.set(c.lhs, 1, c.rhs + 1);               
            },
            crate::Operator::LessThan => {
                self.set(c.lhs, 1, c.rhs);
                inverse.set(c.lhs, c.rhs, 4001);
            },
        }
    }
    fn set(&mut self, category: Category, min: usize, max: usize) {
        match category {
            Category::X => {
                self.x.0 = self.x.0.max(min);
                self.x.1 = self.x.1.min(max);
            },
            Category::M => {
                self.m.0 = self.m.0.max(min);
                self.m.1 = self.m.1.min(max);
            },
            Category::A =>{
                self.a.0 = self.a.0.max(min);
                self.a.1 = self.a.1.min(max);
            },
            Category::S =>{
                self.s.0 = self.s.0.max(min);
                self.s.1 = self.s.1.min(max);
            },
        }
    }
}

pub fn build_ranges(wfs: &WorkflowEngine) -> Vec<Range> {
    let ranges = vec![Range::new(None, Outcome::Reject, &[])];
    build_range(wfs, &wfs.entry,0, ranges)
}

fn build_range(wfs: &WorkflowEngine, current: &str, mut parent: usize, mut ranges: Vec<Range>) -> Vec<Range> {
    let Some(wf) = wfs.workflows.get(current) else {panic!()};
    let mut inverse = Range::new(Some(parent), Outcome::Reject, &ranges);

    for rule in &wf.rules {
        let mut range = inverse.clone();
        range.outcome = rule.outcome.clone();
        if let Some(c) = rule.condition {
            range.apply(c, &mut inverse);
        }
        ranges.push(range);
            parent = ranges.len() - 1;
            if let Outcome::GoTo(target) = &rule.outcome {
                ranges = build_range(wfs, target, parent, ranges);
            }     
    }
    ranges
}