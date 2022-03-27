use std::process::exit;

pub enum AutoGraderExit {
  NoError,
  ConfigFileNotFound,
  ConfigParseFailed,
  ConnectRunnerFailed,
  StartRunnerFailed,
  RuntimeRunnerFailed,
  RESTServerRuntimeFailed,
  RESTServerStartFailed,
}

impl AutoGraderExit {
  fn printErr(self, msg: &str) -> () {
    println!("----- Autograder exitted with error -----");
    println!("!!! {} !!!", msg);
  }
  pub fn exit(self) -> ! {
    match self {
      AutoGraderExit::NoError => exit(0),
      AutoGraderExit::ConfigFileNotFound => {
        self.printErr("Config file not found");
        exit(2);
      }
      _ => {
        self.printErr("There's some err");
        exit(1)
      }
    }
  }
}
