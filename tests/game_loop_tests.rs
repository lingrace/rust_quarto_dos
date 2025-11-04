use rust_quarto_dos::game_loop::{self, GameIO};

struct MockIO {
    inputs: Vec<String>,
    pub outputs: Vec<String>,
}

impl MockIO {
    fn new(inputs: Vec<String>) -> Self {
        Self {
            inputs,
            outputs: vec![],
        }
    }
}

impl GameIO for MockIO {
    fn input(&mut self) -> String {
        // TODO: add size validation
        self.inputs.remove(0) // Pop off first input
    }

    fn output(&mut self, str: String) {
        self.outputs.push(str);
    }
}

#[test]
fn test_first_loop_test() {
    let mut mock_io = MockIO::new(
        vec!["p1", "p2", "quit"]
            .iter()
            .map(|f| f.to_string())
            .collect(),
    );

    game_loop::game_loop(&mut mock_io);

    println!("{:?}", mock_io.outputs);
}

// TODO: write more game loop tests
