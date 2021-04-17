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

struct GateState {
    pub gate: Gate,
    pub inverted: bool,
}

pub struct GateStack {
    stack: Vec<GateState>,
    state: bool,
}

impl Default for GateStack {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            state: true,
        }
    }
}

impl GateStack {
    pub fn new() -> Self {
        Self::default()
    }
    fn compute_state(&mut self) {
        self.state = self.stack.iter().all(|gs| gs.gate.is_open())
    }
    pub fn push(&mut self, gate: Gate) {
        self.state = self.state && gate.is_open();
        self.stack.push(GateState {
            gate,
            inverted: false,
        });
    }
    pub fn pop(&mut self) -> Option<Gate> {
        let gs_opt = self.stack.pop();
        if let Some(gs) = gs_opt.as_ref() {
            if !gs.gate.is_open() {
                self.compute_state();
            }
        }
        gs_opt.map(|x| x.gate)
    }
    pub fn is_open(&self) -> bool {
        self.state
    }
    pub fn invert_last(&mut self) -> Result<bool, ()> {
        let last = self.stack.last_mut().ok_or(())?;
        if !last.inverted {
            last.inverted = true;
            Ok(match &mut last.gate {
                Gate::Known(_, value) => {
                    *value = !*value;
                    if *value {
                        self.compute_state();
                    } else {
                        self.state = false;
                    }
                    true
                }
                Gate::Unknown => false,
            })
        } else {
            Err(()) // already inverted
        }
    }
}
