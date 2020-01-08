use crate::names::{Generator, GeneratorPtr};
use crate::robot::Robot;

pub struct Factory {
    gen: GeneratorPtr,
}

impl Factory {
    pub fn new() -> Self {
        Factory {
            gen: Generator::new(),
        }
    }

    pub fn build(&self) -> Result<Robot, String> {
        Robot::new(self.gen.clone())
    }
}

impl Default for Factory {
    fn default() -> Self {
        Self::new()
    }
}
