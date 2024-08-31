use std::{io::{stdin, stdout, Write}, process::exit};
use crossterm::{cursor, execute, style::Stylize, terminal::{self, ClearType}};
use futures_util::{SinkExt, StreamExt};
use reqwest::Url;
use sodiumoxide::crypto::secretbox;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>
{
    print!("Enter your name\n>");
    let _ = stdout().flush();
    let mut usrName = String::new();
    std::io::stdin().read_line(&mut usrName).expect("could not take user input");
    let usrName = usrName.trim();
    match usrName
    {
        "SERVER"=>
        {
            println!("User cannot be named SERVER");
            exit(1);
        },
        ""=>
        {
            println!("This cannot be empty");
            exit(1);
        },
        "ADMIN"=>
        {
            println!("User cannot be named admin");
            exit(1);
        },
        "Kam4T0r" | "Kam4Tor" | "czerwonyy"=>
        {
            println!("Reserved for creators");
            exit(1);
        },
        "kozlos_91"=>
        {
            println!("nuh uh");
            exit(1);
        }
        _=>{}
    }
    print!("enter encryption key: ");
    let _ = stdout().flush();
    let mut k = String::new();
    stdin().read_line(&mut k).unwrap();
    let k = k.trim();
    let key:Vec<i32> = k.split(",").map(|x| x.parse().unwrap()).collect();
    let key: [u8;32] = key.iter().map(|x| *x as u8).collect::<Vec<u8>>().try_into().unwrap();
    
    let client = reqwest::Client::builder().user_agent("Mozilla/5.0").build()?;
    let res = client.get("https://utilsy.glitch.me/rat.txt").send().await?;
    let servIP = res.text().await?;
    let servIP = servIP.trim();

    let addr = "ws://localhost:2208";
    let url = Url::parse(&addr)?;

    
    let (mut socket, _response) =  connect_async(url.to_string()).await.expect("Couldn't establish connection");
    let _ = tokio::spawn(async move
        {
            loop
            {
                while let Some(msg) = socket.next().await
                {
                    match msg
                    {
                        Ok(Message::Text(msg)) =>
                        {
                            execute!(
                                stdout(),
                                cursor::MoveDown(1),
                                cursor::SavePosition,
                                cursor::MoveToColumn(0),
                                cursor::MoveUp(1),
                                terminal::Clear(ClearType::CurrentLine),
                            ).unwrap();
                            let mkey = [99,30,131,40,254,172,57,248,16,20,2,225,135,61,221,73,20,92,153,50,93,200,175,41,21,43,187,207,38,194,178,162];
                            let key = secretbox::Key::from_slice(&key).unwrap();
                            let nonce = secretbox::Nonce([0u8;24]);
                            let msg: Vec<u8> = msg.trim_start_matches("[").trim_end_matches("]").split(",").map(|x| x.trim().parse::<u8>().unwrap()).collect();
                            let msg = secretbox::open(&msg, &nonce, &key);
                            let msg = String::from_utf8(msg.unwrap()).unwrap();
                            println!("{}",msg.trim());
                            execute!(
                                stdout(),
                                cursor::RestorePosition,
                            ).unwrap();
                        },
                        Err(e)=>{println!("Error occurred {}",e)},
                        _=>{}
                    }
                }
            }
        });
    let (mut socket, _response) =  connect_async(url.to_string()).await.expect("Couldn't establish connection");
    loop
    {
        print!(">");
        let _ = stdout().flush();
        let mut msg = String::new();
        stdin().read_line(&mut msg).unwrap();
        let mkey = [99,30,131,40,254,172,57,248,16,20,2,225,135,61,221,73,20,92,153,50,93,200,175,41,21,43,187,207,38,194,178,162];
        let key = secretbox::Key::from_slice(&key).unwrap();
        let nonce = secretbox::Nonce([0u8;24]);
        let ct = secretbox::seal(msg.as_bytes(), &nonce, &key);
        let _ = socket.send(Message::text(format!("{:?}",ct).to_string())).await?;
    }
}