use actix_web::{get, web, App, HttpServer, Responder};
use mqtt_async_client::client::{Client, Subscribe, SubscribeTopic};
use std::{str::FromStr, time::Duration};
use anyhow::Result;
use tokio::spawn;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    spawn(async{
        let res = subscribe().await;
        println!("连接失败:{:?}", res);
    });

    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn subscribe() -> Result<()> {

    let username = "1572981452067311617";
    let password = "676b10a321982351fbc9ee325bc1d3d5";
    let topic = "1572981452067311617|sub";
    let host = "101.35.80.206";
    let port = 1884;
    
    let url = url::Url::from_str(&format!("mqtt://{host}:{port}"))?;
    println!("创建MQTT连接： {host}:{port} username={username}, password={password}");
    
    let mut c = Client::builder()
    .set_client_id(Some(username.to_owned()))
    .set_url(url)?
    .set_username(Some(username.to_owned()))
    .set_password(Some(password.as_bytes().to_owned()))
    .set_operation_timeout(Duration::from_secs(10))
    .set_connect_retry_delay(Duration::from_secs(1))
    .build()?;
    
    c.connect().await?;
    
    println!("开始连接MQTT服务器...");
    
    // Subscribe
    let subopts = Subscribe::new(vec![
        SubscribeTopic { qos: mqtt_async_client::client::QoS::AtMostOnce, topic_path: topic.to_owned() }
        ]);
    let subres = c.subscribe(subopts).await?;
    subres.any_failures()?;
    println!("mqtt su 订阅成功...");

    loop{
        match c.read_subscriptions().await{
            Ok(r) => {
                let payload = String::from_utf8_lossy(r.payload()).to_string();
                println!("接收到消息: {payload}");
            }
            Err(e) => {
                eprintln!("read_subscriptions: {:?}", e);
                break;
            }
        }
    }
    c.disconnect().await?;

    Ok(())
}