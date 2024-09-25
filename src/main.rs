use serde_json::json;
use xtb::timestamp_to_datetime;
use xtb::xapi_definitions::commands_main::*;
use xtb::xapi_definitions::commands_stream::*;
use std::thread;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt, split};

use cliclack::{intro, outro, input, password};

static XAPI_ADDRESS: &str = "xapi.xtb.com";
static XAPI_PORT: &str = "5124";
static XAPI_PORT_STREAM: &str = "5125";

const MAX_RETRIES: u32 = 3;

use xtb::XApiClient;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    intro("User credentials")?;
    let user_id: String = input("User Id").default_input("16794637").placeholder("***").interact()?;
    let password = password("Password").mask('▪').interact()?;
    outro("OK")?;

    let login_req = Request::Login(
        LoginRequest {
            user_id: user_id.into(),
            password: password.into(),
            app_id: "test".into(),
            app_name: "XTB_test".into(),
        }
    );

    let mut xapi_client = XApiClient::new(XAPI_ADDRESS, XAPI_PORT).await?;
    let mut xapi_client_stream = XApiClient::new(XAPI_ADDRESS, XAPI_PORT_STREAM).await?;

    for i in 0..MAX_RETRIES {
        match xapi_client
            .execute_command(&login_req)
            .await
        {
            Ok(_) => {
                println!("Connection to socket successful");
                break;
            }
            Err(e) => {
                eprintln!("Failed to write to socket [{}]; err = {:?}", i, e);
                thread::sleep(Duration::from_millis(200));
                if i < MAX_RETRIES {
                    continue;
                } else {
                    return Err("Connection error")?;
                }
            }
        }
    }

    let response_login =  xapi_client.response_login().await?;

    let address = format!("{}:{}", XAPI_ADDRESS, XAPI_PORT_STREAM);
    println!("address: {}", address);

    let req = json!({
        "command": "getSymbol",
        "arguments": {
            "symbol": "RHM.DE_4"
        },
    });
    println!("\nRequest-> {}", req);
    xapi_client.socket.write_all(&req.to_string().as_bytes()).await?;

    let mut buf = vec![0; 1024];
    let n = xapi_client.socket.read(&mut buf).await?;
    let str = String::from_utf8((&buf[..n]).to_vec())?;
    println!("Response raw <- {:?}", str);

    let get_symbol = Request::GetSymbol(
        GetSymbol {
            symbol: "RHM.DE_9".into(),
        }
    );
    xapi_client.execute_command(&get_symbol).await?;
    let mut response_raw = String::new(); 
    xapi_client.get_response_raw(&mut response_raw, 1024).await?;
    println!("Response raw <- {:?}", response_raw);

    //Get commission_def
    let get_commission_def = 
        Request::GetCommissionDef(
            GetCommissionDef { 
                symbol: "GOLD".into(), 
                volume: 20000000.0, 
            } 
    );

    let commission_def = 
        xapi_client.response_data::<GetCommissionDefResponse>(&get_commission_def).await?; 
    println!("Commission def: {:?}", commission_def);
    println!("Commission def: {:?}", commission_def.return_data.commission);

    //GetCurrentUserData    
    thread::sleep(Duration::from_millis(200));
    let get_current_user_data = 
        Request::GetCurrentUserData (
            GetCurrentUserData {} 
        ); 

  //  xapi_client.execute_command(&get_current_user_data).await?;
    let get_current_user_data_response = 
        xapi_client.response_data::<GetCurrentUserDataResponse>(&get_current_user_data).await?;

    println!("GetCurrentUserData:\n{:?}", get_current_user_data_response);
    
    let request_stream = RequestStream::GetTickPrices(
        GetTickPrices {
            stream_session_id: String::from(&response_login.stream_session_id).into(),
            symbol: "RHM.DE_4".into(),
            min_arrival_time: Some(5000),
            max_level: Some(1), 
        }
    );
    xapi_client_stream.execute_command(&request_stream).await?;


    let request_stream = RequestStream::GetBalance(
        GetBalance {
            stream_session_id: String::from(&response_login.stream_session_id).into(),
        }
    );
    xapi_client_stream.execute_command(&request_stream).await?;


    let get_candles = RequestStream::GetCandles(
        GetCandles {
            stream_session_id: String::from(&response_login.stream_session_id).into(),
            symbol: "EURUSD".into(),
        }
    );
    xapi_client_stream.execute_command(&get_candles).await?;
    

    let request_stream = RequestStream::GetKeepAlive(
        GetKeepAlive {
            stream_session_id: String::from(&response_login.stream_session_id).into(),
        }
    );
    xapi_client_stream.execute_command(&request_stream).await?;

    tokio::spawn(async move {
        let mut buf = vec![0; 1024];
        let mut retries = 0;
        loop {
           // thread::sleep(Duration::from_millis(200));
            match xapi_client_stream.socket.read(&mut buf).await {
                Ok(0) => {
                    println!("spawn 0");
                    retries += 1;
                }
                Ok(n) => {
                    let str = String::from_utf8((&buf[..n]).to_vec());
                    //println!("Response raw [{}]: {:?}", n, str);

                    match str {
                        Ok(str) => {
                            let separate_replies: Vec<&str> = str.split_terminator("\n\n").collect();
                            for rep in separate_replies {
                                match serde_json::from_str::<ResponseStream>(rep) {
                                    Ok(res) => {
                                        match res {
                                            ResponseStream::TickPrices(tick) => {
                                                println!("Tick prices [ask]: {}", tick.data.ask);
                                                println!("Tick prices [bid]: {}", tick.data.bid);
                                                println!("Tick prices [low]: {}", tick.data.low);
                                                println!("Tick prices [high]: {}", tick.data.high);
                                            }
                                            ResponseStream::KeepAlive(keep) => {
                                                println!("Keep alive [timestamp][{}][{}]"
                                                            , keep.data.timestamp
                                                            , timestamp_to_datetime(keep.data.timestamp)
                                                        );

                                            }
                                            ResponseStream::Candle(candle) => {
                                                println!("Candle [close]: {}", candle.data.open);
                                            }
                                            _ => {
                                                println!("Response stream: {:?}", res);
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!("Failed to convert response request stream-> {:?}\n{:?}", str, err);
                                    }
                                }
                            }
                        }
                        Err(err) => {
                                    eprintln!("Not valid UTF8 {:?}", err);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                }
            }
            if retries >= MAX_RETRIES {
                println!("Exeeded max pesponse retries!");
                break;
            };
        }
    });

    loop {}

}
