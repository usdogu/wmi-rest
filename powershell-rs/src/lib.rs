mod util;
use std::io::{Write, BufRead, Read, BufReader};

use util::*;
pub struct Shell {
    handle: std::process::Child,
}

impl Shell {
    pub fn new() -> Self {
        let handle = std::process::Command::new("powershell.exe")
            .arg("-NoLogo")
            .arg("-NoExit")
            .arg("-Command")
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start powershell.exe");

        Self { handle }
    }
    // TODO: unwrap
    pub fn execute(&mut self, cmd: impl AsRef<str>) -> Result<(String, String), Box<dyn std::error::Error>>  {
        let out_boundary = create_boundary();
        let err_boundary = create_boundary();
        let full = format!("{}; echo '{}'; [Console]::Error.WriteLine('{}')\r\n", cmd.as_ref(), out_boundary, err_boundary);
        let stdin = self.handle.stdin.as_mut().unwrap();
        let stdout = self.handle.stdout.as_mut().unwrap();
        let stderr = self.handle.stderr.as_mut().unwrap();
        stdin.write_all(full.as_bytes())?;
        let sout = read_streaming(stdout, &out_boundary)?;
        let serr = read_streaming(stderr, &err_boundary)?;
        Ok((sout, serr))
    }
}

fn read_streaming<T: Read>(stream: T, boundary: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let mut stream = BufReader::new(stream);
    loop {
        let mut line = String::new();
        stream.read_line(&mut line)?;
        if line.contains(boundary) {
            break;
        }
        buf.push_str(&line);
    }
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pwsh() {
        let mut shell = Shell::new();
        let (sout, serr) = shell.execute("echo 1").unwrap();
        assert_eq!(sout, "1\r\n");
        assert_eq!(serr, "");
    }
    #[test]
    fn test_pwsh_fail() {
        let mut shell = Shell::new();
        let (sout, serr) = shell.execute("this_command_does_not_exist_hopefully").unwrap();
        assert_eq!(sout, "");
        assert_ne!(serr, "");
    }
}