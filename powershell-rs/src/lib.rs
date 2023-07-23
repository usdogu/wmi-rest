mod util;

use deadpool::unmanaged::Pool;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader},
    process::{Child, Command},
};
use util::*;
use anyhow::{Result, Context};

pub struct Shell {
    handle_pool: Pool<Child>,
}

impl Shell {
    pub fn new() -> Self {
        let handle_pool = Pool::<Child>::new(5);
        for _ in 0..5 {
            let handle = Command::new("powershell.exe")
                .arg("-NoLogo")
                .arg("-NoExit")
                .arg("-Command")
                .arg("-")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to start powershell.exe");
            handle_pool.try_add(handle).expect("Failed to add handle");
        }

        Self { handle_pool }
    }

    pub async fn execute(&mut self, cmd: impl AsRef<str>) -> Result<(String, String)> {
        let mut handle_obj = self.handle_pool.get().await?;
        let handle = handle_obj.as_mut();
        let out_boundary = create_boundary();
        let err_boundary = create_boundary();
        let full = format!(
            "{}; echo '{}'; [Console]::Error.WriteLine('{}')\r\n",
            cmd.as_ref(),
            out_boundary,
            err_boundary
        );
        let stdin = handle.stdin.as_mut().context("Failed to get stdin")?;
        let stdout = handle.stdout.as_mut().context("Failed to get stdout")?;
        let stderr = handle.stderr.as_mut().context("Failed to get stderr")?;
        stdin.write_all(full.as_bytes()).await?;
        let res = tokio::try_join!(
            read_streaming(stdout, &out_boundary),
            read_streaming(stderr, &err_boundary)
        );
        match res {
            Ok((sout, serr)) => Ok((sout, serr)),
            Err(err) => Err(err),
        }
    }
}

async fn read_streaming<T: AsyncRead + std::marker::Unpin>(
    stream: T,
    boundary: &str,
) -> Result<String> {
    let mut buf = String::new();
    let mut stream = BufReader::new(stream);
    loop {
        let mut line = String::new();
        stream.read_line(&mut line).await?;
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
    #[tokio::test]
    async fn test_pwsh() {
        let mut shell = Shell::new();
        let (sout, serr) = shell.execute("echo 1").await.unwrap();
        assert_eq!(sout, "1\r\n");
        assert_eq!(serr, "");
    }
    #[tokio::test]
    async fn test_pwsh_fail() {
        let mut shell = Shell::new();
        let (sout, serr) = shell
            .execute("this_command_does_not_exist_hopefully")
            .await
            .unwrap();
        assert_eq!(sout, "");
        assert_ne!(serr, "");
    }
}
