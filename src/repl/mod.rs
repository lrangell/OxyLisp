use crate::env::init_env;
use crate::evaluator::eval_str;
use reedline::{DefaultPrompt, FileBackedHistory, Reedline, Signal};

pub fn init() {
    let env = init_env();
    let history = Box::new(
        FileBackedHistory::with_file(50, "~/.cache/oxy_history.txt".into())
            .expect("Error configuring history with file"),
    );

    let mut line_editor = Reedline::create().with_history(history);
    let prompt = DefaultPrompt::new(
        reedline::DefaultPromptSegment::Basic("λ ".to_string()),
        reedline::DefaultPromptSegment::Empty,
    );

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) if buffer.is_empty() => {
                match eval_str(&buffer, env.clone()) {
                    Ok(primitive) => println!("{}", primitive),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            _ => {}
        }
    }
}
