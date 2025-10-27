use std::io;
use std::path::PathBuf;
use tokio::process::{Child, Command};
use tokio::runtime::Handle;
use futures::executor;

pub struct ManagedProcess {
    pub child: Child,
}

impl ManagedProcess {
    pub fn start(cmd: PathBuf) -> io::Result<Self> {
        let mut command = Command::new(cmd);

        let child = command.spawn()?;
        Ok(Self { child })
    }

    pub async fn kill(&mut self) {
        self.child.kill().await.unwrap();
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
            let handle = Handle::current();
            let _ = handle.enter();

            executor::block_on(self.child.kill()).unwrap();
        }
    }
}
