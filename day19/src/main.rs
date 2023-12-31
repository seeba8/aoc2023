#![feature(let_chains)]
mod part2;

use std::{collections::HashMap, str::FromStr};

use nom::{
    branch::alt,
    character::complete::{alpha1, digit1},
    combinator::{map_res, value, map, all_consuming},
    multi::separated_list1,
        bytes::complete::tag, sequence::{tuple, delimited, preceded}, error::Error, Finish,
};

use crate::part2::build_ranges;
fn main() {
    let (workflows, parts) = include_str!("input.txt").split_once("\n\n").unwrap();
    let workflows: WorkflowEngine = workflows.parse().unwrap();
    let parts: Vec<Part> = parts.lines().map(Part::from_str).collect::<Result<Vec<_>,_>>().unwrap();
    let s: usize = parts.iter().filter(|p| workflows.is_accepted(p)).map(Part::sum).sum();
    println!("Day 19 part 1: {s}");
    let ranges = build_ranges(&workflows);
    let s: usize = ranges.iter().filter(|r| r.outcome == Outcome::Accept).map(part2::Range::product).sum();
    println!("Day 19 part 2: {s}");
}
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    const fn get(&self, category: Category) -> usize {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    const fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = all_consuming(
            delimited(
                tag::<_, _, Error<_>>("{"),
                 tuple((
                    preceded(tag("x="), map_res(digit1, usize::from_str)),
                    preceded(tag(",m="), map_res(digit1, usize::from_str)),
                    preceded(tag(",a="), map_res(digit1, usize::from_str)),
                    preceded(tag(",s="),map_res(digit1, usize::from_str)),
                 )), 
                 tag("}")))(s).finish().unwrap().1;
        Ok(Self {
            x: x.0,
            m: x.1,
            a: x.2,
            s: x.3,
        })            
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct WorkflowEngine {
    workflows: HashMap<String, Workflow>,
    entry: String,
}
impl FromStr for WorkflowEngine {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut workflows = HashMap::new();
        for line in s.lines() {
            let wf: Workflow = line.parse()?;
            workflows.insert(wf.name.clone(), wf);
        }
        Ok(Self {
            workflows,
            entry: "in".to_owned(),
        })
    }
}

impl WorkflowEngine {
    fn is_accepted(&self, part: &Part) -> bool {
        let mut outcome = self.workflows[&self.entry].apply(part);
        loop {
            match outcome {
                Outcome::Accept => {
                    return true;
                }
                Outcome::Reject => {
                    return false;
                }
                Outcome::GoTo(s) => {
                    outcome = self.workflows[s].apply(part);
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn apply(&self, part: &Part) -> &Outcome {
        for rule in &self.rules {
            match rule.condition {
                Some(condition) => {
                    if condition.applies(part) {
                        return &rule.outcome;
                    }
                }
                None => {
                    return &rule.outcome;
                }
            }
        }
        unreachable!("The last rule should not have a condition")
    }
}
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Rule {
    condition: Option<Condition>,
    outcome: Outcome,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Condition {
    lhs: Category,
    op: Operator,
    rhs: usize,
}

impl Condition {
    const fn applies(&self, part: &Part) -> bool {
        match self.op {
            Operator::GreaterThan => part.get(self.lhs) > self.rhs,
            Operator::LessThan => part.get(self.lhs) < self.rhs,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Operator {
    GreaterThan,
    LessThan,
}
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum Outcome {
    Accept,
    Reject,
    GoTo(String),
}
impl FromStr for Workflow {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outcome = |i| alt((
            value(Outcome::Accept, tag::<_, _, Error<_>>("A")),
            value(Outcome::Reject, tag::<_, _, Error<_>>("R")),
            map(alpha1, |v: &str| Outcome::GoTo(v.to_string())),
        ))(i);
        let category = |i| alt((
            value(Category::X, tag("x")),
            value(Category::M, tag("m")),
            value(Category::A, tag("a")),
            value(Category::S, tag("s"))
        ))(i);
        let operator = |i|  alt((
            value(Operator::GreaterThan, tag(">")),
            value(Operator::LessThan, tag("<")),
        ))(i);
    
        let rule_with_condition = |i| map(
                tuple((
                category,
                operator,
                map_res(digit1, |d: &str| d.parse::<usize>()),
                tag(":"),
                outcome,
                )), 
            |(lhs, op, rhs, _, o )| Rule{
                condition: Some(Condition {
                    lhs,
                    op,
                    rhs,
                }),
                outcome: o,
            })(i);
        let mut workflow =map(tuple((
            alpha1,
            tag("{"),
            separated_list1(tag(","), rule_with_condition),
            tag(","),
            outcome,
            tag("}"),
        )), |(n, _,mut c, _, e, _)| {
            c.push(Rule { condition: None, outcome: e });
            Self {
                name: n.to_string(),
                rules: c,
            }
        });
        
        let x = workflow(s).finish().unwrap().1;
        Ok(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
#[test]
    fn it_parses_workflow() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        let workflow: Workflow = input.parse().unwrap();
        assert_eq!(workflow,
        Workflow { name: "px".to_owned(), 
            rules: vec![
                Rule { condition: Some(Condition { lhs: Category::A, op: Operator::LessThan, rhs: 2006 }), outcome: Outcome::GoTo("qkq".to_owned()) },
                //Rule { condition: Some(Condition { lhs: Category::A, op: Operator::LessThan, rhs: 2006 }), outcome: Outcome::GoTo("qkq".to_owned()) },
                Rule{ condition: Some(Condition { lhs: Category::M, op: Operator::GreaterThan, rhs: 2090 }), outcome: Outcome::Accept },
                Rule { condition: None, outcome: Outcome::GoTo("rfg".to_owned()) }]
        });

    }
    #[test]
    fn it_parses_part() {
        let input = "{x=787,m=2655,a=1222,s=2876}";
        let part: Part = input.parse().unwrap();
        assert_eq!(part, Part { x: 787, m: 2655, a: 1222, s: 2876 });
    }

    #[test]
    fn it_tests_part() {
        let (workflows, parts) = EXAMPLE.split_once("\n\n").unwrap();
        let workflows: WorkflowEngine = workflows.parse().unwrap();
        let parts: Vec<Part> = parts.lines().map(Part::from_str).collect::<Result<Vec<_>,_>>().unwrap();
        assert!(workflows.is_accepted(&parts[0]));
        assert!(!workflows.is_accepted(&parts[1]));
    }

    #[test]
     fn it_gets_sum() {
        let (workflows, parts) = EXAMPLE.split_once("\n\n").unwrap();
        let workflows: WorkflowEngine = workflows.parse().unwrap();
        let parts: Vec<Part> = parts.lines().map(Part::from_str).collect::<Result<Vec<_>,_>>().unwrap();
        let s: usize = parts.iter().filter(|p| workflows.is_accepted(p)).map(Part::sum).sum();
        assert_eq!(s, 19114);
     }

     #[test]
     fn it_gets_ranges() {
        let (workflows, parts) = EXAMPLE.split_once("\n\n").unwrap();
        let workflows: WorkflowEngine = workflows.parse().unwrap();
        let ranges = build_ranges(&workflows);
    //dbg!(&ranges);
    
    let s: usize = ranges.iter().filter(|r| r.outcome == Outcome::Accept).map(|r| r.product()).sum();
    assert_eq!(s, 167409079868000);
     }
}