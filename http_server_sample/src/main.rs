
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;
extern crate random;

use hyper::server::{Request, Response, Server};
use std::io::Write;
use prometheus::{Counter,CounterVec, Encoder, Gauge, HistogramVec, TextEncoder};
use random::Source;
const ADDR:&str = "127.0.0.1:9891";

lazy_static! {
    static ref HTTP_COUNTER: Counter = register_counter!(opts!(
        "requests_total",
        "Total number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    
    static ref HTTP_METHOD_COUNTER: CounterVec = register_counter_vec!(
        "requests_method_total",
        "Total number of HTTP requests made group by uri.",
        &["uri"]
    )
    .unwrap();
    static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(opts!(
        "response_size_bytes",
        "The HTTP response sizes in bytes.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    )
    .unwrap();
}

fn get_prometheus_info()->Vec<u8>{
     let encoder = TextEncoder::new();
     let mut buffer = vec![];
     let metric_families = prometheus::gather();
     encoder.encode(&metric_families, &mut buffer).unwrap();
     buffer
}

fn get_request_message(req:&Request)->Vec<u8> {
     let mut buffer = vec![];
     writeln!(&mut buffer,"Welcome {}",random::default().read::<i64>()).unwrap();
     writeln!(&mut buffer,"Current Request Info").unwrap();
     writeln!(&mut buffer,"RemoteAddr:{}",req.remote_addr).unwrap();
     writeln!(&mut buffer,"Method:{}",req.method).unwrap();
     writeln!(&mut buffer,"Version:{}",req.version).unwrap();
     writeln!(&mut buffer,"try GET http://{}/metrics",ADDR).unwrap();
     writeln!(&mut buffer).unwrap();
     writeln!(&mut buffer).unwrap();
     buffer
}

fn on_start_request(uri:&str,collect:bool)->Option<prometheus::HistogramTimer> {
    if !collect{
        return None;
    }
    HTTP_COUNTER.inc();
    HTTP_METHOD_COUNTER.with_label_values(&[uri]).inc();
    Some(HTTP_REQ_HISTOGRAM.with_label_values(&[uri]).start_timer())
}

fn on_finish_request(timer:Option<prometheus::HistogramTimer>,content_len:usize) {
     if timer.is_none(){
         return;
     }
     timer.unwrap().observe_duration();
     HTTP_BODY_GAUGE.set(content_len as f64);
}


fn main() {
    println!("listening addr {:?}", ADDR);
    let enable_prometheus = true;
    Server::http(ADDR)
        .unwrap()
        .handle(move |req: Request, res: Response|{
            let uri = format!("{} {}",req.method,req.uri);
            let timer = on_start_request(&uri,enable_prometheus);
            let response = if enable_prometheus && uri.contains("metrics"){
                get_prometheus_info()
            } else {
                get_request_message(&req)
            };
            res.send(&response).unwrap();
            on_finish_request(timer,response.len());
        })
        .unwrap();
}