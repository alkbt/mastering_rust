use crate::names;

pub struct Robot {
    name_: String,
    gen: names::GeneratorPtr,
}

impl Robot {
    pub fn new(gen: names::GeneratorPtr) -> Result<Robot, String> {
        let name = gen.lock().unwrap().generate()?;
        Ok(Robot { name_: name, gen })
    }

    pub fn name(&self) -> &str {
        self.name_.as_ref()
    }

    pub fn reset(&mut self) -> Result<(), String> {
        self.name_ = self.gen.lock().unwrap().generate()?;
        Ok(())
    }
}
