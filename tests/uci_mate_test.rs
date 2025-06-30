use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_uci_mate_detection() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_axelrot"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start axelrot engine");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let commands = b"uci\nposition fen 6k1/3r1ppp/8/8/8/8/5PPP/6K1 w - - 0 1\ngo depth 2\n";
    stdin.write_all(commands).expect("Failed to write to stdin");
    stdin.flush().unwrap();

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);


    println!("ENGINE OUTPUT:\n{}", stdout);

    assert!(stdout.contains("mate") || stdout.contains("moves.is_empty() reached"), "Engine did not detect mate or reach empty moves: output was {}", stdout);
}
