use std::env;
use std::io::{stdin, stdout, Write};

use whoami;

use g_shell::parser;
use g_shell::process;

fn main() {
    sh_loop();
}

fn sh_loop() {
    loop {
        print!(
            "{}@{}:{} > ",
            whoami::username(),
            whoami::hostname(),
            env::current_dir().unwrap().display()
        );
        stdout().flush().unwrap();

        //コマンドラインを取得
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Faild to read line");
        line.remove(line.len() - 1);

        //コマンドの解析
        let mut command = parser::parser::CommandParse::new();
        command.run(line);

        //コマンドの実行とプロセスの生成
        let mut process = process::process::Process::new(&command);
        match process.argvs_execute(){
            Ok(_) => {}
            Err(e) => {
                eprint!("{}", e);
            }
        }
    }
}