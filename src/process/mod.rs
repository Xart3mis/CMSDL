use std::io;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

pub struct ManagedProcess {
    pub child: Child,
}

impl ManagedProcess {
    pub fn start(cmd: PathBuf, args: &[&str]) -> io::Result<Self> {
        let mut command = Command::new(cmd);
        command.args(args);

        let nullout = Stdio::null();
        let nullerr = Stdio::null();

        command.stdout(nullout).stderr(nullerr).stdin(Stdio::null());

        let child = command.spawn()?;
        Ok(Self { child })
    }

    pub fn kill(&mut self) -> io::Result<()> {
        self.child.kill()
    }

    pub fn is_running(&mut self) -> bool {
        match self.child.try_wait() {
            Ok(Some(_status)) => false,
            Ok(None) => true,
            Err(_) => false,
        }
    }
}

impl Drop for ManagedProcess {
    fn drop(&mut self) {
        if self.is_running() {
            let _ = self.child.kill();
        }
    }
}
