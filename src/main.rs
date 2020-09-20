use std::env::current_dir;
use std::fs::{self, File};
use std::path::{PathBuf};
use std::process::{Command, Output};
use std::str;
use std::time::{SystemTime};

use serde::{Deserialize, Serialize};

use amet::error::{Result};


#[derive(Serialize, Deserialize, Debug)]
struct CmdMeta {
    cmd: String,
    args: Vec<String>,
    description: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Cmd {
    name: String,
    command: CmdMeta,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CommandResult {
    name: String,
    status: bool,
    output: String,
}


impl CommandResult {
    pub fn new(name: String, o: Output) -> Self {
        CommandResult {
            name,
            status: o.status.success(),
            output: String::from_utf8(match o.status.success() {
                true => o.stdout,
                false => o.stderr
            }).unwrap_or_else(|_| "".to_string())
        }
    }
}


pub struct Amet {}


impl Amet {
    pub fn run(&self) -> Result<()> {
        dump(&default_file()?, &self.prepare(self.files()?)?)?;

        Ok(())
    }

    fn files(&self) -> Result<Vec<String>> {
        let files = get_files(
        current_dir()?
            .join("src")
            .join("cmds")
        )?;

        Ok(files)
    }

    fn prepare(&self, files: Vec<String>) -> Result<Vec<CommandResult>> {
         let command_list = files
            .into_iter()
            .flat_map(|i| -> Result<_> { Ok(read_file(i)?) })
            .flat_map(|cmds| cmds
                .into_iter()
                .map(|cmd|-> Result<_> { Ok(self.execute(&cmd)?) })
            )
            .flatten()
            .collect::<Vec<CommandResult>>();

        Ok(command_list)
    }

    fn execute(&self, data: &Cmd) -> Result<CommandResult> {
        Ok(
            CommandResult::new(
                data.name.clone(),
                Command::new(&data.command.cmd)
                    .args(&data.command.args)
                    .output()?
            )
        )
    }
}


fn get_files(path: impl Into<PathBuf>) -> Result<Vec<String>> {
    let files = fs::read_dir(&path.into())?
        .flat_map(|res| -> Result<_> { Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some("json".as_ref()))
        .flat_map(|path| -> Result<_> {
            Ok(path
                .into_os_string()
                .into_string()?
            )
        })
        .collect();

    Ok(files)
}


fn read_file(file: String) -> Result<Vec<Cmd>> {
    Ok(serde_json::from_reader(File::open(file)?)?)
}


fn default_file() -> Result<String> {
    Ok(format!(
        "{}.json",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
            .to_string()
    ))
}


fn dump<T: Serialize>(path: &str, value: &[T]) -> Result<()> {
    serde_json::to_writer_pretty(&mut File::create(path)?, value)?;

    Ok(())
}


fn main() {
    Amet{}.run().expect("");
}
