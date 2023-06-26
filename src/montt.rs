use std::{collections::VecDeque, error::Error, fs};

use crate::{cli::TaskCriticalPathArgs, statistics::log_normal_from_estimates};
use rand;
use rand_distr::{Distribution, LogNormal};

#[derive(Default, Debug)]
pub struct Montt {
    adjacency: Vec<Vec<i64>>,
    tasks: Vec<Task>,
}

pub struct CriticalPath {
    tasks: Vec<Task>,
}

impl CriticalPath {
    pub fn duration(&self) -> f64 {
        self.tasks.iter().map(|task| task.estimate).sum()
    }
}

impl Montt {
    pub fn log_normal(&self) -> LogNormalMontt {
        LogNormalMontt {
            adjacency: self.adjacency.clone(),
            tasks: self.tasks.iter().map(|task| task.log_normal()).collect(),
        }
    }

    /// Calculates the critical path of the project.
    /// Based on https://stackoverflow.com/questions/6007289/calculating-the-critical-path-of-a-graph
    pub fn critical_path(&self) -> CriticalPath {
        let mut indegrees = self
            .adjacency
            .iter()
            .map(|row| row.iter().filter(|&&x| x == -1).count())
            .collect::<Vec<_>>();

        let mut q = VecDeque::new();
        for &indegree in &indegrees {
            if indegree == 0 {
                q.push_back(indegree);
            }
        }

        let mut distances = vec![0.0; self.tasks.len()];
        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            for (u, &direction) in self.adjacency[v].iter().enumerate() {
                distances[u] = (distances[v] + self.tasks[v].estimate).max(distances[u]);
                indegrees[u] -= 1;
                if indegrees[u] == 0 {
                    q.push_back(u);
                }
            }
        }

        CriticalPath {
            tasks: self.tasks.iter().map(|&task| task.clone()).collect(),
        }
    }
}

pub trait Sample {
    fn sample(&self) -> f64;
}

impl Sample for Montt {
    fn sample(&self) -> f64 {
        self.critical_path().duration()
    }
}

struct LogNormalMontt {
    adjacency: Vec<Vec<i64>>,
    tasks: Vec<LogNormal<f64>>,
}

impl Sample for LogNormalMontt {
    fn sample(&self) -> f64 {
        0.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct Task {
    name: String,
    estimate: f64,
    q95: f64,
}

impl Task {
    fn log_normal(&self) -> LogNormal<f64> {
        log_normal_from_estimates(self.estimate, self.q95)
    }
}

impl Sample for Task {
    fn sample(&self) -> f64 {
        self.estimate
    }
}

impl Sample for LogNormal<f64> {
    fn sample(&self) -> f64 {
        Distribution::sample(&self, &mut rand::thread_rng())
    }
}

#[derive(Default, Debug)]
struct ParsedTask {
    name: String,
    estimate: f64,
    q95: f64,
    before: Vec<String>,
    resource: String,
}

#[derive(Default, Debug)]
struct ParsedResource {
    name: String,
    capacity: f64,
}

enum ParseState {
    Task(ParsedTask),
    Resource(ParsedResource),
    Skip,
}

const TASK: &str = "task";
const RESOURCE: &str = "resource";
const RESOURCES: &str = ":resources";
const CAPACITY: &str = ":capacity";
const ESTIMATE: &str = ":estimate";
const Q95: &str = ":q95";
const BEFORE: &str = ":before";

impl<'parse> Montt {
    pub fn parse(file: &'parse str) -> Result<Self, Box<dyn Error>> {
        let mut contents: &str = &fs::read_to_string(file)?;

        let mut tasks = Vec::<ParsedTask>::new();
        let mut resources = Vec::<ParsedResource>::new();
        while contents.len() > 0 {
            let line = contents.lines().next().unwrap();

            let parsed_line = Self::parse_line(line)?;
            match parsed_line {
                ParseState::Task(task) => tasks.push(task),
                ParseState::Resource(resource) => resources.push(resource),
                ParseState::Skip => (),
            }
            contents = &contents[line.len() + 1..];
        }

        let mut montt = Montt::default();
        montt.adjacency = vec![vec![0; tasks.len()]; tasks.len()];
        let mut task_map = std::collections::HashMap::<String, usize>::new();
        for (i, task) in tasks.iter().enumerate() {
            montt.tasks.push(Task {
                name: task.name.clone(),
                estimate: task.estimate,
                q95: task.q95,
            });
            task_map.insert(task.name.clone(), i);
        }

        for task in tasks.iter() {
            for before in task.before.iter() {
                let i = task_map.get(before).unwrap();
                montt.adjacency[*task_map.get(&task.name).unwrap()][*i] = 1;
                montt.adjacency[*i][*task_map.get(&task.name).unwrap()] = -1;
            }
        }

        Ok(montt)
    }

    fn parse_line(line: &str) -> Result<ParseState, Box<dyn Error>> {
        let split = line.split_whitespace();
        let split = split.filter(|s| !s.is_empty());
        if line.starts_with(TASK) {
            let task = Self::parse_task(split.skip(1));
            return Ok(ParseState::Task(task));
        }

        if line.starts_with(RESOURCE) {
            return Ok(ParseState::Resource(ParsedResource::default()));
        }

        Ok(ParseState::Skip)
    }

    fn parse_task<'a, I>(it: I) -> ParsedTask
    where
        I: Iterator<Item = &'a str>,
    {
        struct Acc<'a> {
            keyword: &'a str,
        }
        let mut task = ParsedTask::default();
        it.fold(Acc { keyword: TASK }, |mut acc, token| {
            if Self::is_keyword(token) {
                acc.keyword = token;
                return acc;
            }

            match acc.keyword {
                TASK => task.name = token.to_string(),
                ESTIMATE => task.estimate = token.parse::<f64>().unwrap(),
                Q95 => task.q95 = token.parse::<f64>().unwrap(),
                BEFORE => task.before.push(token.to_string()),
                RESOURCE => task.resource = token.to_string(),
                _ => (),
            }

            acc
        });

        task
    }

    fn is_keyword(token: &str) -> bool {
        matches!(
            token,
            TASK | RESOURCE | RESOURCES | CAPACITY | ESTIMATE | Q95 | BEFORE
        )
    }
}
