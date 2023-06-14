use std::collections::HashSet;
use std::{error::Error, fs};

#[derive(Default, Debug)]
pub struct Montt {
    adjacency: Vec<Vec<i64>>,
    tasks: Vec<Task>,
}

#[derive(Default, Debug)]
pub struct Task {
    name: String,
    estimate: f64,
    q95: f64,
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
