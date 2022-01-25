mod blockchain;
mod models;
use models::PostJson::{PostTransaction};
use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};
use crate::models::block::{Block,Transaction};
use crate::blockchain::blockchain::{Blocks};
use std::thread;
use std::time::Duration;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[get("/")]
async fn index()->impl Responder{

    HttpResponse::Ok().body("Hello World!")
}

#[post("/post")]
async fn post_index(user: web::Json<PostTransaction>)->impl Responder{

    let mut chain: Vec<Block> = Vec::new();

    let post_data = PostTransaction{
        sender:user.sender.to_string().clone(),
        recipient:user.recipient.to_string().clone(),
        amount:user.amount,
    };

    let tmpdata = Transaction{
        sender: post_data.sender,
        recipient: post_data.recipient,
        amount: post_data.amount,
    };

    
    //println!("{:?}",tmpdata.create_block(&mut chain));

    thread::spawn(move ||{
        tmpdata.create_block(&mut chain);
    });
    

    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .service(index)
        .service(post_index)
    })
    .bind("0.0.0.0:81")?
    .run()
    .await
}