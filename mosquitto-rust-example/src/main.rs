use std::time::Duration;
use anyhow::Result;
use futures_util::StreamExt;
use mqtt::{QOS_2, QOS_1};
extern crate paho_mqtt as mqtt;

#[tokio::main]
async fn main() -> Result<()>{
    //发送一个消息
    publish().await?;

    // 连接一个客户端接收消息
    receive().await?;
    
    Ok(())
}

async fn receive() -> Result<()>{
    let mut cli = mqtt::AsyncClient::new("tcp://47.114.117.22:8883")?;

     // Get message stream before connecting.
     let mut strm = cli.get_stream(25);

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .user_name("test2")
        .password("123456test2")
        .finalize();

    cli.connect(conn_opts).await?;

    cli.subscribe("/message", QOS_1).await?;

    println!("Waiting for messages...");

    while let Some(msg_opt) = strm.next().await {
        if let Some(msg) = msg_opt {
            println!("{}", msg);
        }
        else {
            // A "None" means we were disconnected. Try to reconnect...
            println!("Lost connection. Attempting reconnect.");
            while let Err(err) = cli.reconnect().await {
                println!("Error reconnecting: {}", err);
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
        }
    }
    
    cli.disconnect(None).await?;
    Ok(())
}

async fn publish() -> Result<()>{
    let cli = mqtt::AsyncClient::new("tcp://47.114.117.22:8883")?;

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .user_name("test1")
        .password("123456test1")
        .finalize();

    cli.connect(conn_opts).await?;

    // Create a message and publish it
    let msg = mqtt::Message::new_retained("/message", "你接收到了吗？", QOS_1);
    cli.publish(msg).await?;
    println!("消息已发送.");

    // Disconnect from the broker
    cli.disconnect(None).await?;
    Ok(())
}
