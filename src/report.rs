pub struct Report {
    issues: Vec<String>,
}

impl Report {
    pub fn new() -> Self {
        Self { issues: vec![] }
    }

    pub fn add(&mut self, msg: String) {
        self.issues.push(msg);
    }

    pub fn print(&self) {
        if self.issues.is_empty() {
            println!("No issues found");
        } else {
            println!("Issues:");
            for i in &self.issues {
                println!("- {}", i);
            }
        }
    }
}
