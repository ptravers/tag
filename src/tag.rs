extern crate matrix_display;
extern crate rand;

use matrix_display::*;
use rand::Rng;

struct RangeOfMotion {
    possible_moves: Vec<Action>,
}

impl RangeOfMotion {
    pub fn get_random_movement(&self) -> Action {
        let i = rand::thread_rng().gen_range(0, self.possible_moves.len());

        self.possible_moves.get(i).unwrap_or(&Action::Stay).clone()
    }
}

#[derive(Clone, PartialEq)]
enum Action {
    Stay,
    Tag,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone)]
struct Agent {
    id: usize,
    is_it: bool,
}

impl Agent {
    pub fn new(id: usize, is_it: bool) -> Agent {
        Agent { id, is_it }
    }

    pub fn untagged(&mut self) {
        self.is_it = false
    }

    pub fn tagged(&mut self) {
        self.is_it = true
    }
}

pub struct Tag {
    grid: Vec<Vec<Agent>>,
    m: usize,
    n: usize,
}

impl Tag {
    pub fn new() -> Tag {
        let mut grid = vec![vec![]; 16];
        let mut agents = vec![];

        for i in 0..4 {
            agents.push(Agent::new(i, i == 0));
        }

        grid.insert(0, agents);

        Tag { grid, m: 4, n: 4 }
    }

    fn is_top(&self, index: usize) -> bool {
        index < self.n
    }

    fn is_left(&self, index: usize) -> bool {
        index % self.n == 0
    }

    fn is_right(&self, index: usize) -> bool {
        index % self.n == self.n - 1
    }

    fn is_bottom(&self, index: usize) -> bool {
        index >= (self.m - 1) * self.n
    }

    //TODO: Should move to Range of motion or refactor doesn't really need to
    //be here
    fn get_range_of_motion(&self, index: usize) -> RangeOfMotion {
        if self.is_top(index) && self.is_left(index) {
            RangeOfMotion {
                possible_moves: vec![Action::Stay, Action::Right, Action::Down],
            }
        } else if self.is_top(index) && self.is_right(index) {
            RangeOfMotion {
                possible_moves: vec![Action::Stay, Action::Left, Action::Down],
            }
        } else if self.is_bottom(index) && self.is_left(index) {
            RangeOfMotion {
                possible_moves: vec![Action::Stay, Action::Right, Action::Up],
            }
        } else if self.is_bottom(index) && self.is_right(index) {
            RangeOfMotion {
                possible_moves: vec![Action::Stay, Action::Left, Action::Up],
            }
        } else if self.is_top(index) {
            RangeOfMotion {
                possible_moves: vec![Action::Stay, Action::Right, Action::Left, Action::Down],
            }
        } else if self.is_left(index) {
            RangeOfMotion {
                possible_moves: vec![Action::Stay, Action::Right, Action::Up, Action::Down],
            }
        } else if self.is_right(index) {
            RangeOfMotion {
                possible_moves: vec![Action::Stay, Action::Left, Action::Up, Action::Down],
            }
        } else if self.is_bottom(index) {
            RangeOfMotion {
                possible_moves: vec![Action::Stay, Action::Left, Action::Up, Action::Right],
            }
        } else {
            RangeOfMotion {
                possible_moves: vec![
                    Action::Stay,
                    Action::Left,
                    Action::Down,
                    Action::Up,
                    Action::Right,
                ],
            }
        }
    }

    fn get_next_index(n: usize, action: &Action, index: &usize) -> Option<usize> {
        match action {
            Action::Stay | Action::Tag => None,
            Action::Left => Some(index - 1),
            Action::Right => Some(index + 1),
            Action::Up => Some(index - n),
            Action::Down => Some(index + n),
        }
    }

    //TODO: Revisit we do an egregeous number of clones could make a lot of this
    // statically assigned and possibly leverage static lifetimes?
    pub fn update(&mut self) {
        let mut agent_whose_it: (usize, usize) = (0, 0);
        // I split out the creation of the action from the performing of the action to make state
        // management simpler further down the line.
        let actions_by_index: Vec<(usize, Vec<Action>)> =
            self.grid
                .iter()
                .enumerate()
                .fold(vec![], |mut actions, (key, agents)| {
                    let range_of_motion = self.get_range_of_motion(key);

                    actions.push((
                        key,
                        agents
                            .iter()
                            .enumerate()
                            .map(|(index, agent)| {
                                if agent.is_it && agents.len() > 1 {
                                    agent_whose_it = (key, index);
                                    Action::Tag
                                } else {
                                    range_of_motion.get_random_movement()
                                }
                            })
                            .collect(),
                    ));
                    actions
                });

        let mut new_grid = vec![vec![]; 16];

        // First run the actions of the player whose it as it requires mutating
        // the state of another agent
        let (it_key, it_index) = agent_whose_it;
        match self.grid.get_mut(it_key) {
            Some(agents) if agents.len() > 1 => {
                let tagged_index = (it_index + 1) % agents.len();
                agents.get_mut(tagged_index).unwrap().tagged();
                agents.get_mut(it_index).unwrap().untagged();
            }
            Some(_) => {}
            None => panic!("Invalid State: Cannot find any agents at expected index"),
        }

        // Resolve the actions. It is assumed at this point that all actions present
        // are valid as they were checked at creation.
        for (key, actions) in &actions_by_index {
            match self.grid.get_mut(*key) {
                Some(agents) => {
                    let mut agents_staying: Vec<Agent> = vec![];

                    for (agent, action) in agents.clone().iter().zip(actions) {
                        if let Some(new_index) = Tag::get_next_index(self.n, action, key) {
                            new_grid.get_mut(new_index).unwrap().push(agent.clone());
                        } else {
                            agents_staying.push(agent.clone());
                        }

                        new_grid.get_mut(*key).unwrap().append(&mut agents_staying);
                    }
                }
                None => panic!("Invalid State: Cannot find any agents at expected index"),
            }
        }

        self.grid = new_grid;
    }

    pub fn get_display_matrix(&self) -> matrix::Matrix<cell::Cell<usize>> {
        let output_grid = self
            .grid
            .iter()
            .map(|agents| {
                let mut has_it = false;
                for agent in agents.iter() {
                    has_it = has_it || agent.is_it
                }

                if has_it {
                    cell::Cell::new(agents.len(), 2, 8)
                } else {
                    cell::Cell::new(agents.len(), 1, 10)
                }
            })
            .collect::<Vec<_>>();

        matrix::Matrix::new(4, output_grid)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tag_created_non_empty() {
        let tag = Tag::new();

        assert!(!tag.grid.is_empty())
    }

    #[test]
    fn test_grid_contains_agents() {
        let tag = Tag::new();
        let number_of_agents = tag
            .grid
            .iter()
            .fold(0 as usize, |count, agents| count + agents.len());

        assert_eq!(number_of_agents, 4)
    }

    #[test]
    fn test_grid_constains_agent_tagged_as_it() {
        let tag = Tag::new();

        let has_agent_tagged_as_it = tag.grid.iter().fold(false, |acc, agents| {
            acc || agents
                .iter()
                .fold(false, |is_it, agent| is_it || agent.is_it)
        });

        assert!(has_agent_tagged_as_it)
    }

    #[test]
    fn test_update_should_move_agents() {
        let mut tag = Tag::new();

        tag.update();

        assert!(tag.grid.get(0).unwrap_or(&vec![]).len() < 4)
    }

    #[test]
    fn test_update_should_retain_all_agents() {
        let mut tag = Tag::new();

        for _ in 1..10 {
            tag.update();
        }

        let number_of_agents = tag
            .grid
            .iter()
            .fold(0 as usize, |count, agents| count + agents.len());

        assert_eq!(number_of_agents, 4)
    }

    #[test]
    fn test_update_should_have_one_agent_be_it() {
        let mut tag = Tag::new();

        for _ in 1..1000 {
            tag.update();
        }

        let number_of_agents = tag.grid.iter().fold(0 as usize, |count, agents| {
            let mut has_it = false;
            for agent in agents.iter() {
                has_it = has_it || agent.is_it
            }

            if has_it {
                count + 1
            } else {
                count
            }
        });

        assert_eq!(number_of_agents, 1)
    }

    #[test]
    fn test_another_agent_should_be_tagged() {
        let mut tag = Tag::new();

        tag.update();

        let updated_agent_tagged_as_it =
            tag.grid
                .iter()
                .flatten()
                .fold(None, |acc, agent| match acc {
                    None if agent.is_it => Some(agent),
                    None => None,
                    Some(agent) => Some(agent),
                });

        assert!(updated_agent_tagged_as_it.is_some());

        println!("{:?}", updated_agent_tagged_as_it.unwrap().id);

        assert!(updated_agent_tagged_as_it.unwrap().id != 0)
    }
}
