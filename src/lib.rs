use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use webpki_roots;

use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;
use tokio_rustls::{rustls, TlsConnector};

use std::sync::Arc; use serde::{Deserialize, Serialize};

pub mod xapi_definitions;
use xapi_definitions::*;
use xapi_definitions::commands_main::*;

use chrono::prelude::*;

pub fn timestamp_to_datetime(timestamp: i64) -> String {
    let datetime = DateTime::from_timestamp_millis(timestamp);
    let offset_in_sec = Local::now().offset().local_minus_utc();
    let offset = FixedOffset::east_opt(offset_in_sec).unwrap();
    
    match datetime {
        Some(datetime) => datetime.with_timezone(&offset).format("%Y-%m-%d %H:%M:%S").to_string(),
        None => "".to_string(),
    }
}

pub struct XApiClient {
    pub socket: TlsStream<TcpStream>,
}

impl XApiClient {
    pub async fn new(xapi_address: &str, xapi_port: &str) -> Result<Self, Box<dyn Error>> {

        let root_cert_store = rustls::RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.iter().cloned().collect(),
        };

        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();

        let address = format!("{}:{}", xapi_address, xapi_port);

        let connector = TlsConnector::from(Arc::new(config));
        let socket = TcpStream::connect(address.as_str()).await?;
        let socket 
            = connector.connect(pki_types::ServerName::try_from(xapi_address)?.to_owned(), socket).await?;

        Ok(Self { socket: socket })
    }

    pub async fn execute_command<T: Execute + Serialize>(
        &mut self,
        request: &T,
    ) -> Result<(), Box<dyn Error>> {
        let request = request.command()?;
        self.socket.write_all(&request.as_bytes()).await?;

        Ok(())
    }

    async fn get_response<T: ValidResponse + Serialize + for<'de> Deserialize<'de>> (
        &mut self,
        response_size: usize,
    ) -> Result<Response<T>, Box<dyn Error>> {
        let mut buf = vec![0; response_size];

        match self.socket.read(&mut buf).await {
            // Return value of `Ok(0)` signifies that the remote has closed
            Ok(0) => {
                eprintln!("Connection closed");
                return Err("error")?;
            }
            Ok(n) => {
                let str =  String::from_utf8((&buf[..n]).to_vec())?;
                println!("Response raw [{}]: {:?}", n, str);
                match serde_json::from_str::<Response<T>>(&str) {
                    Ok(res) => {
                        Ok(res)
                    }
                    Err(err) => {
                        //eprintln!("Failed to convert response request -> {:?}\n{:?}", str, err);
                        let error = format!("Failed to convert response request -> {}. {}", str, err);
                        Err(error)?
                    }
                }
            }
            Err(e) => {
                // Unexpected socket error. There isn't much we can do here so just stop processing.
                eprintln!("Failed to read from socket err = {:?}", e);
                return Err(e)?;
            }
        }
    }

//    pub async fn response_login <T: ValidResponse + Serialize + for<'de> Deserialize<'de>> (
    pub async fn response_login (
        &mut self,
    ) -> Result <LoginResponse, Box<dyn Error>> {
        match self.get_response::<LoginResponse>(1024).await? {
            Response::Login(res) => {
                Ok(res)
            }
            Response::Error(err) => {
                eprintln!("Login error: {:?}", err);
                let error = format!("Login error: {} - {}", err.error_code, err.error_descr);
               // return Err(error)?;
                Err(error)?
            }
            _ => {
                Err("Response does not match request")?
            }
        }
    }

//    pub async fn response_data <T: ValidResponse + Serialize + for<'de> Deserialize<'de>> (
    pub async fn response_data <T> (
        &mut self,
        request: &Request,
    ) -> Result <GetResponse<T>, Box<dyn Error>> 
    where
        T: ValidResponse + Serialize + for<'de> Deserialize<'de>,
    {
        self.execute_command(request).await?;

        match self.get_response::<T>(1024).await? {
            Response::Data(res) => {
                Ok(res)
            }
            Response::Error(err) => {
                //eprintln!("Login error: {:?}", err);
                let error = format!("Data error: {} - {}", err.error_code, err.error_descr);
                Err(error)?
            }
            _ => {
                Err("Response does not match request")?
            }
        }
    }

    pub async fn get_response_raw (
        &mut self,
        response_raw: &mut String,
        response_size: usize,
    ) -> Result<(), Box<dyn Error>> {
        let mut buf = vec![0; response_size];
        let n = self.socket.read(&mut buf).await?;
        let str = String::from_utf8((&buf[..n]).to_vec())?;
        response_raw.push_str(&str);
        //println!("Response raw <- {:?}", str);
        Ok(())
    }

}
