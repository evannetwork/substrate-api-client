use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket,ErrorEvent};
use super::{ResultE,OnMessageFn};
use futures_signals::signal::Mutable;
use futures::channel::mpsc::{
  Receiver,
  Sender,
  channel
};
use futures::sink::{Sink, SinkExt};
macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn start_rpc_client_thread(
  url: String,
  jsonreq: String,
  result_in: Mutable<String>,
  on_message_fn: OnMessageFn,
) {
  console_log!("url: {}",&url);
  let ws = WebSocket::new(&url).unwrap();
  let ws_c = ws.clone();
  let on_message = {
    Closure::wrap(Box::new(move |evt: MessageEvent| {
        let msgg = evt.data()
                    .as_string()
        .expect("Can't convert received data to a string");
        console_log!("{}",&msgg);
        let res_e = (on_message_fn)(&msgg);
        console_log!("got res_e");
        match res_e {
          ResultE::None=>{
            console_log!("none");
          },
          ResultE::Close=>{
            console_log!("close");
            ws_c.close_with_code(1000).unwrap();
          },
          ResultE::S(s)=>{
            console_log!("s {}",&s);
            result_in.set(s);
          },
          ResultE::SClose(s)=>{
            console_log!("sclose");
            result_in.set(s);
            ws_c.close_with_code(1000).unwrap();
          }
        }
    }) as Box<dyn FnMut(MessageEvent)>)
  };
  
  ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
  on_message.forget();
  let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
      console_log!("error event: {:?}", e);
  }) as Box<dyn FnMut(ErrorEvent)>);
  ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
  onerror_callback.forget();
  let cloned_ws = ws.clone();
  let onopen_callback = Closure::wrap(Box::new(move |_| {
      match cloned_ws.send_with_str(&jsonreq) {
          Ok(_) => console_log!("message successfully sent"),
          Err(err) => console_log!("error sending message: {:?}", err),
      }
  }) as Box<dyn FnMut(JsValue)>);
  ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
  onopen_callback.forget();

}


pub async fn start_rpc_client_thread_sender(
  url: String,
  jsonreq: String,
  result_in: Sender<String>,
  on_message_fn: OnMessageFn,
) {
  console_log!("SENDER url: {}",&url);
  let ws = WebSocket::new(&url).unwrap();
  let ws_c = ws.clone();
  let on_message = {
    Closure::wrap(Box::new(move |evt: MessageEvent| {
        let msgg = evt.data()
                    .as_string()
        .expect("Can't convert received data to a string");
        console_log!("{}",&msgg);
        let res_e = (on_message_fn)(&msgg);
        console_log!("SENDER got res_e");
        match res_e {
          ResultE::None=>{
            console_log!("none");
          },
          ResultE::Close=>{
            console_log!("close");
            ws_c.close_with_code(1000).unwrap();
          },
          ResultE::S(s)=>{
            console_log!("SENDER s {}",&s);
            result_in.clone().try_send(s);
          },
          ResultE::SClose(s)=>{
            console_log!("SENDER sclose");
            result_in.clone().try_send(s);
            ws_c.close_with_code(1000).unwrap();
          }
        }
    }) as Box<dyn FnMut(MessageEvent)>)
  };
  
  ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
  on_message.forget();
  let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
      console_log!("error event: {:?}", e);
  }) as Box<dyn FnMut(ErrorEvent)>);
  ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
  onerror_callback.forget();
  let cloned_ws = ws.clone();
  let onopen_callback = Closure::wrap(Box::new(move |_| {
      match cloned_ws.send_with_str(&jsonreq) {
          Ok(_) => console_log!("message successfully sent"),
          Err(err) => console_log!("error sending message: {:?}", err),
      }
  }) as Box<dyn FnMut(JsValue)>);
  ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
  onopen_callback.forget();

}