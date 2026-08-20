use std::{ env, process::{Command, Output}, time
};

use fxhash::FxHashMap;
struct Fuzz {
    target: String,
    // msg: Vec<String>,
    msg: FxHashMap<String, u32>,
}

impl Fuzz {
    fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 || args.len() > 2 {
            eprintln!("Usage: {} <target_binary>", args[0]);
            std::process::exit(1);
        }
        let target = args[1].clone();
        // let output = Command::new(target).output();
        Fuzz {
            target,
            msg: FxHashMap::default(),
        }
    }
    fn save_bin_msg(&mut self, new: String) {
        if !self.msg.cont {
            self.msg.push(new);
        }
    }
    fn timeout_checker(){
        std::thread::sleep(time::Duration::from_millis(100));
        std::process::exit(1);
    }
    fn input_checker(){
        todo!("check if the input are argv or while the code is running");
    }
    
    fn lenght_checker (&mut self) {
        let mut str = String::new();
        for i in 0..10 {
            str.push('A');
            self.run_target(Some(str.as_str()));
           
       }
    }
    fn reaction_checker(mut self) {
        self.run_target(None);
        self.run_target(Some(""));
        self.lenght_checker();
        self.run_target(Some("aaaaaaaaawdwdqwdqw"));
        self.run_target(Some("Fpnxweoe"));
        println!("{:#?}", self.msg);
    }

    fn run_target(&mut self, args: Option<&str>) {
        let mut cmd = Command::new(&self.target);
        if let Some(s) = args {
            cmd.arg(s);
        }
        self.print(cmd.output().unwrap());
    }

    fn print(&mut self, cmd: Output) {
        let stdout_str = String::from_utf8_lossy(&cmd.stdout).trim().to_string();
        let stderr_str = String::from_utf8_lossy(&cmd.stderr).trim().to_string();

        let full_output = format!("{}{}", stdout_str, stderr_str);
        self.save_bin_msg(full_output.clone());
        // println!("{full_output}");
    }
}

fn main() {
    let fuzzer = Fuzz::new();

    fuzzer.reaction_checker();
}
