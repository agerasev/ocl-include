pub enum Gate {
    Known(String, bool),
    Unknown,
}

impl Gate {
    pub fn is_open(&self) -> bool {
        match self {
            Gate::Known(_, value) => *value,
            Gate::Unknown => true,
        }
    }
}

pub struct GateStack {
    stack: Vec<Gate>,
    state: bool,
}

impl Default for GateStack {
    fn default() -> Self {
        Self { stack: Vec::new(), state: true }
    }
}

impl GateStack {
    pub fn new() -> Self {
        Self::default()
    }
    fn compute_state(&mut self) {
        self.state = self.stack.iter().all(Gate::is_open)
    }
    pub fn push(&mut self, gate: Gate) {
        self.state = self.state && gate.is_open();
        self.stack.push(gate);
    }
    pub fn pop(&mut self) -> Option<Gate> {
        let gate_opt = self.stack.pop();
        if let Some(gate) = gate_opt.as_ref() {
            if !gate.is_open() {
                self.compute_state();
            }
        }
        gate_opt
    }
    pub fn is_open(&self) -> bool {
        self.state
    }
}
