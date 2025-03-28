use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

/// A simple Lambda request structure with just one field
/// that tells the Lambda what is expected of it.
#[derive(Deserialize)]
struct Request {
  token: String,
  signatureVerified: bool,
  protocols: Vec<String>,
  protocolData: Value,
  connectionMetadata: Map<String, Value>,
}

/// Event types that tell our Lambda what to do do.
#[derive(Deserialize, PartialEq)]
enum Prococol {
  tls,
  http,
  mqtt,
}

/// Event types that tell our Lambda what to do do.
#[derive(Deserialize, PartialEq)]
enum EventType {
  Response,
  ExternalError,
  SimpleError,
  CustomError,
  Panic,
}
#[derive(Debug, Serialize)]
struct Policy {
  Version: String,
  // Statement:
}
/// A simple Lambda response structure.
#[derive(Serialize)]
struct Response {
  isAuthenticated: bool,
  principalId: String,
  disconnectAfterInSeconds: u32,
  refreshAfterInSeconds: u32,
    policyDocuments: Value,
//   req_id: String,
//   msg: String,
}

#[derive(Debug, Serialize)]
struct CustomError {
  is_authenticated: bool,
  req_id: String,
  msg: String,
}

impl std::error::Error for CustomError {
  // this implementation required `Debug` and `Display` traits
}

impl std::fmt::Display for CustomError {
  /// Display the error struct as a JSON string
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let err_as_json = json!(self).to_string();
    write!(f, "{}", err_as_json)
  }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  // The runtime logging can be enabled here by initializing `tracing` with `tracing-subscriber`
  // While `tracing` is used internally, `log` can be used as well if preferred.
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    // this needs to be set to false, otherwise ANSI color codes will
    // show up in a confusing manner in CloudWatch logs.
    .with_ansi(false)
    // disabling time is handy because CloudWatch will add the ingestion time.
    .without_time()
    .init();

  // call the actual handler of the request
  let func = service_fn(func);
  lambda_runtime::run(func).await?;
  Ok(())
}

/// The actual handler of the Lambda request.
pub(crate) async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
  let (event, ctx) = event.into_parts();
  // tracing::info!("{} {}", "Request is", event,);
  let e: Request = serde_json::from_value(event)?;
  // tracing::info!("{} {}",  "Token is", event["token"],);
  // tracing::info!("{} {}",  "signature is", event["signatureVerified"],);
  // tracing::info!("{} {}",  "protocols is", event["protocols"],);
  // tracing::info!("{} {}",  "protocolData is", event["protocolData"],);
  // tracing::info!("{} {}",  "connectionMetadata is", event["connectionMetadata"],);
  tracing::info!("{}", e.token);
  tracing::info!("{}", e.signatureVerified);
  tracing::info!("{:?}", e.protocols);

  match e.protocols.as_slice() {
    [first, middle @ .., last] => {}
    _ => {}
  }

  tracing::info!("{}", e.protocolData);
  //   tracing::info!("{}", e.connectionMetadata.identifying);

  if let [first, middle @ .., last] = e.protocols.as_slice() {
    tracing::info!("{} {:?} {}", first, middle, last);
    if middle.is_empty() && last == "http" {
      tracing::info!("HTTPS");
    } else if middle.is_empty() && last == "mqtt" {
      tracing::info!("MQTT over TLS");
    } else if middle.len() == 1 && middle[0] == "http" && last == "mqtt" {
      tracing::info!("MQTT over Websocket");
    }
  }
  tracing::info!("Unknown protocal {:?}", e.protocols);
  //   tracing::warn!("Unkonwn protocal {}", e.protocols);
  //   match e.protocols.as_slice(){
  //       // https
  //     //   [tls, http] => {
  //     //     tracing::info!("HTTPS protocal: {} {} ", tls, http,);
  //     //   },
  //       // mqtts
  //     //   [first,  second] => {
  //     //     tracing::info!("MQTT over TLS protocal: {} {}", first,  second);
  //     //   },
  //       // wss
  //       [tls, http, mqtt] => {
  //         tracing::info!("MQTT over Websocket protocal: {} {} {}", tls, http, mqtt);
  //       },
  //       _ => tracing::info!("protocol don't know, ignore!")
  //   }
  //   match serde_json::from_value::<Request>(event)?.signatureVerified {
  //     true => {
  //       tracing::info!("true")
  //     }
  //     false => {
  //       tracing::info!("false")
  //     }
  //   }

  let resp = Response {
    isAuthenticated: true,
    principalId: "xxxxx".to_string(),
    disconnectAfterInSeconds: 86400,
    refreshAfterInSeconds: 300,
    policyDocuments: json!([
      {
        "Version": "2012-10-17",
        "Statement": [
           {
              "Action": "iot:Publish",
              "Effect": "Allow",
              "Resource": "arn:aws:iot:us-east-1:<your_aws_account_id>:topic/customauthtesting"
            }
         ]
       }
    ]),
    // req_id: ctx.request_id,
    // msg: (stage + "OK!").into(),
    // msg: ctx.env_config.function_name + "OK!".into(),
  };

  return Ok(json!(resp));
  //   tracing::info!(token, "token is");

  //   let resp = Response {
  //     req_id: ctx.request_id,
  //     // msg: (stage + "OK!").into(),
  //     msg: "OK!".into(),
  //   };

  //   return Ok(json!(resp));

  //   check what action was requested
  //   match serde_json::from_value::<Request>(event)?.event_type {
  //       EventType::SimpleError => {
  //           // generate a simple text message error using `simple_error` crate
  //           return Err(Box::new(simple_error::SimpleError::new("A simple error as requested!")));
  //       }
  //       EventType::CustomError => {
  //           // generate a custom error using our own structure
  //           let cust_err = CustomError {
  //               is_authenticated: ctx.identity.is_some(),
  //               req_id: ctx.request_id,
  //               msg: "A custom error as requested!".into(),
  //           };
  //           return Err(Box::new(cust_err));
  //       }
  //       EventType::ExternalError => {
  //           // try to open a non-existent file to get an error and propagate it with `?`
  //           let _file = File::open("non-existent-file.txt")?;

  //           // it should never execute past the above line
  //           unreachable!();
  //       }
  //       EventType::Panic => {
  //           panic!();
  //       }
  //       EventType::Response => {
  //           // let stage = std::env::var("STAGE").expect("Missing STAGE env var");
  //           // generate and return an OK response in JSON format
  //           let resp = Response {
  //               req_id: ctx.request_id,
  //               // msg: (stage + "OK!").into(),
  //               msg: ctx.env_config.function_name + "OK!".into(),
  //           };

  //           return Ok(json!(resp));
  //       }
  //   }
}
