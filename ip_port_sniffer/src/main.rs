use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::process;


struct Arguments {
    flag: String,
    ipaddr: IpAddr, //to make sure ip address is valid
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4});
        }
        else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!("Usage: -j to select the number of threads than you want \r\n -h or -help to show this help message");
                return Err("help");
            } 
            else if flag.contains("-h") || flag.contains("--help") {
                return Err("Too many arguments");
            } 
            else if flag.contains("-j"){
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IP address; must be IPV4 or IPV6")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number")
                };
                return Ok(Arguments{threads, flag, ipaddr});
            } 
            else {
                return Err("Invalid syntax");
            }
        }
    }
}

fn main() {
    println!("Hello, rust!");
    let args: Vec<String> = env::args().collect();

    //to debug arguments
    // for i in &args {
    //     println!("{}", i);
    // }

    // println!("{:?}", args);

    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err|{
            if err.contains("help"){
                process::exit(0);
            }
            else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

}
