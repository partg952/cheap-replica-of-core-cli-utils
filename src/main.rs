use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::process;
struct Command<'a> {
    args:&'a Vec<String>
}
fn read_file(filename:&String) -> Result<String,io::Error>{
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}
fn write_file(filename:&String,contents:&String) -> Result<(),io::Error> {
    let mut file = File::options().append(true).open(filename)?;
    writeln!(&mut file,"{}",contents);
    Ok(())
}
impl<'a> Command<'a> {
    fn new(arguments:&Vec<String>) -> Result<Command,String>{
       
        return Ok(Command {
            args:arguments
        })

    } 
    fn echo(&self) {
        for i in self.args[2..self.args.len()].iter() {
            print!("{} ",i);
        }
    }
    fn cat(&self) -> Result<(),io::Error>{
        if self.args.len() > 3 {
            let filename1 = &self.args[2];
            let filename2 = &self.args[3];
            let file1_contents = read_file(&filename1)?;
            write_file(&filename2,&file1_contents)?;
            Ok(())
        }
        else{
            let filename1 = &self.args[2];
            let contents = read_file(&filename1)?;
            println!("{}",contents);
            Ok(())
        }
    }
    fn ls(&self) -> Result<(),io::Error>{
       let paths = fs::read_dir(self.args[2].clone())?;
       for i in paths {
           println!("{:?}",i.unwrap().path().display());
       }
       Ok(())
    }
    fn find(&self) -> Result<(),io::Error> {
        let path = &self.args[2];
        let filename = &self.args[3];
        let dir = fs::read_dir(path)?;
        for file in dir {
            let iterator = file.unwrap().path().display().to_string();
            if "./".to_string()+filename == iterator {
                println!("item found:{}",iterator);
                break;
            }

        }
        
        return Ok(());

    }
}
fn main() {
    let args:Vec<String> = env::args().collect(); 
    let command = Command::new(&args).unwrap_or_else(|err| {
        println!("{}",err);
        process::exit(1);
    });
    match args[1].as_str() {
        "echo" => {
            command.echo();
        },
        "cat" => {
            command.cat();
        },  
        "ls" => {
            command.ls();
        },
        "find" => {
            command.find();
        },
        &_ => {
            eprintln!("No valid command");
        }
    } 
}
