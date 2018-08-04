use std::io;
use std::io::Write;

pub struct ProgressBar {
    name : String,
    goal : usize,
    progress : usize
}

impl ProgressBar {
    pub fn new(name : &str, goal : usize) -> Self {
        Self { name : String::from(name), goal, progress : 0 }
    }

    fn percent(&self, progress : usize) -> usize {
        if progress >= self.goal {
            return 100;
        }

        progress * 100 / self.goal
    }

    pub fn step(&mut self) -> &mut Self {
        self.progress += 1;
        self
    }

    pub fn print(&self) -> &Self {
        let percent_progress = self.percent(self.progress);

        if self.progress > 0 && self.percent(self.progress) > self.percent(self.progress - 1) {
            print!("\r{}: {}%", self.name, percent_progress);
            io::stdout().flush().expect("Failed to write");
        }

        if percent_progress == 100 {
            println!();
        }

        self
    }
}