use std::io;
use std::io::Write;
use std::sync::{Arc, RwLock};

struct State {
    progress: usize,
}

pub struct ProgressBar {
    name: String,
    goal: usize,
    state: Arc<RwLock<State>>,
}

impl ProgressBar {
    pub fn new(name: &str, goal: usize) -> Self {
        Self {
            name: String::from(name),
            goal,
            state: Arc::new(RwLock::new(State { progress: 0 }))
        }
    }

    fn percent(&self, progress: usize) -> usize {
        if progress >= self.goal {
            return 100;
        }

        progress * 100 / self.goal
    }

    pub fn step(&self) -> &Self {
        {
            let mut state = self.state.write().unwrap();
            state.progress += 1;
        }
        self
    }

    pub fn print(&self) {
        let progress = self.state.read().unwrap().progress;
        let percent_progress = self.percent(progress);

        if progress > 0 && self.percent(progress) > self.percent(progress - 1) {
            print!("\r{}: {}%", self.name, percent_progress);
            io::stdout().flush().expect("Failed to write");
        }

        if percent_progress == 100 {
            println!();
        }
    }
}
