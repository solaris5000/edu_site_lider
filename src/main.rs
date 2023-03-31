#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use std::io::Split;

use rocket::response::content::Html;
use rocket::Data;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html(r"<title>GCD Calculator</title>
    <a href='exit'>Terminate this site</a>

    <form action='sendmail' method='post'>
        <input type='text' name='addr_' />
        <input type='text' name='message_' />
        <button type='submit'>Send mail</button>
    </form>"
    )
}


#[post("/sendmail", data="<user_input>")]
fn sendmail(user_input : Data) -> String {
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


    let clonetextdbg = text.clone();
    let emmail = Message::builder()
    .from("NoBody <edu.solaris5000tv@gmail.com>".parse().unwrap())
    .to(String::from(email).parse().unwrap())
    .subject("Happy new year")
    .header(ContentType::TEXT_PLAIN)
    .body(String::from(text))
    .unwrap();

    let creds = Credentials::new("solaris5000tv@gmail.com".to_owned(), "teadhemoxwmtzhqd".to_owned());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();

    match mailer.send(&emmail) {
        Ok(_) => format!("Message succefully sednded, msg text : {}", clonetextdbg),
        Err(e) => format!("Message not sended, {}", e),
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

    rocket::custom(cfg).mount("/", routes![index, hello, exit, sendmail]).launch();

}