#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use std::io::Split;

use rocket::Response;
use rocket::Rocket;
use rocket::response::content;
use rocket::http;
use rocket::Data;
use rocket::response::NamedFile;
use std::env;
use std::path::{Path, PathBuf};
use rocket::http::MediaType;
use image;



use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use std::fs::File;
use std::io::{ self, BufRead, BufReader };

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]

fn index() -> Result<NamedFile, std::io::Error> {

    let exepath = env::current_dir().unwrap();
    let indexpath =  exepath.join("static").join("index.html");
    println!("{:?}", indexpath);
    NamedFile::open(indexpath)

/*
fn index() -> Html<&'static str> {
    Html(r#"<title>GCD Calculator</title>
    <a href='exit'>Terminate this site</a>

    <form action='sendmail' method='post'>
        <input type='text' name='addr_' />
        <textarea name='message_'>Put some text here</textarea>
        <button type='submit'>Send mail</button>
    </form>
    "#
    )*/
}

#[get("/img/<filename>")]
fn img(filename: String) -> Option<NamedFile> {

    let exepath = env::current_dir().unwrap();
    let imgpath =  exepath.join("static").join("img").join(filename);
    println!("{:?}", imgpath);
    NamedFile::open(imgpath).ok()
}

#[get("/css/<filename>")]
fn css(filename: String) -> Option<NamedFile> {

    let exepath = env::current_dir().unwrap();
    let csspath =  exepath.join("static").join("css").join(filename);
    println!("{:?}", csspath);
    NamedFile::open(csspath).ok()
}

#[get("/js/<filename>")]
fn js(filename: String) -> Option<NamedFile> {

    let exepath = env::current_dir().unwrap();
    let jspath =  exepath.join("static").join("css").join(filename);
    println!("{:?}", jspath);
    NamedFile::open(jspath).ok()
}

#[post("/sendmail", data="<user_input>")]
fn sendmail(user_input : Data) -> String {

    let file = File::open("./creds".to_string()).unwrap(); 
    
    let lines = io::BufReader::new(file).lines(); 
    let mut getsender = true;
    let mut getsenderemail = true;
    let mut getpwd = true;
    //getting NAME <MAILBOX> string
    let mut sender: String = "".to_owned();
    let mut senderemail: String = "".to_owned();
    //getting app password from gmail
    let mut pwd:String = "".to_owned();
    for line in lines {
        if getsender { sender = line.unwrap().clone(); getsender = false; continue; }
        if getsenderemail { senderemail = line.unwrap().clone(); getsenderemail = false; continue; }
        if getpwd { pwd = line.unwrap().clone(); getpwd = false; break; }
    }

    let input_data = user_input.open();


    let data = std::io::read_to_string(input_data).unwrap();

    let data = urldecode::decode(data);

    let data = &data[..];

    let data_parts = data.split('&'); 

    let mut getting_email = true;
    let mut getting_text = true;

    let mut email = "";
    let mut text = "";

    for part in data_parts {
        if getting_email {
            let tmp = part.split('=');
            let mut first = true;
            for tmp_p in tmp {
                if first {
                    first = false;
                    continue;
                } else {
                    email = tmp_p;
                    break;
                }
            }
            getting_email = false;
            continue;
        }

        if getting_text {
            let tmp = part.split('=');
            let mut first = true;
            for tmp_p in tmp {
                if first {
                    first = false;
                    continue;
                } else {
                    text = tmp_p;
                    break;
                }
            }
            getting_text = false;
            break;
        }
    }

    let text = text.replace("+", " ");

    let clonetextdbg = text.clone();
    let emmail = Message::builder()
    .from(sender.parse().unwrap())
    .to(String::from(email).parse().unwrap())
    .subject("Happy new year")
    .header(ContentType::TEXT_PLAIN)
    .body(String::from(text))
    .unwrap();

    let creds = Credentials::new(senderemail.to_owned(), pwd.to_owned());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();

    match mailer.send(&emmail) {
        Ok(_) => format!("Message succefully sednded, msg text : {}", clonetextdbg),
        Err(e) => format!("Message not sended, {}\n\nuser {}  email {} pwd {}", e, sender, senderemail, pwd),
    }
}

#[get("/exit")]
fn exit() {
    std::process::exit(111);
}

fn main() {

    let cfg = rocket::config::Config::build(rocket::config::Environment::Development)
        .address("127.0.0.1")
        .port(80)   
        .extra("template_dir",  "web/templates")
        .unwrap();

    rocket::custom(cfg).mount("/", routes![index, exit, sendmail, img, css, js]).launch();

}