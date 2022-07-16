use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use hoi_client::Client;
use gpio_handler::GpioHandler;
use tungstenite::{connect, Message,WebSocket};
use tungstenite::client::AutoStream;
use serde_json::Value;


pub struct ClientHandler{
    last_trigger:DateTime<Utc>,
    client:Client,
    gpio_handler:GpioHandler;
}

impl ClientHandler{

    pub fn new(host:String,port:String,name:String)->Self{
        Self{
            last_trigger:Utc::now(),
            client:Client::new(host,port,name,"SolenoidController".to_owned(),"main".to_owned),
            gpio_handler:GpioHandler::new(17);
        }
    }
    
    pub fn main(&mut self){
        let websocket = self.client.connect_and_authenticate();
        if websocket.is_some(){
            let unwrapped_websocket = websocket.unwrap();
            while true{
                let msg_result = unwrapped_websocket.read_message();
                if msg_result.is_ok(){
                    let msg = msg_result.unwrap().into_text().unwrap();
                    self.route_message(msg,unwrapped_websocket);
                }
                else{
                    println!("issue with message result");
                }
            }
        }
        else{
            println!("issue gathering websocket connection");
        }
    }

    fn route_message(&mut self,message:String,socket:&mut WebSocket<AutoStream>){

        if message == "trigger_solenoid"{
            self.gpio_handler.trigger_relay_for_x_milliseconds(5000);
            socket.write_message(Message::Text("success".into()));
        }

        else if message == "deactivate"{
            self.enter_deactivate_loop(socket);
        }

        else if message == "passive_data"{
            socket.write_message(Message::Text(self.passive_data().into()));
        }
    }

    fn enter_deactivate_loop(&mut self,socket:&mut WebSocket<AutoStream>){
        while true{
            let msg_result = socket.read_message();
            if msg_result.is_ok(){
                let msg = msg_result.unwrap().into_text().unwrap();
                if msg == "activate"{
                    break;
                }
                else{
                    println!("Got non-activate message while deactivated!");
                }
            }
            else{
                println!("issue gathering message while deactivated");
            }
        }
    }

    fn passive_data(&mut self)->String{
        let data = json!({"alert_status":"alert_not_present","last_triggered":self.last_trigger.to_string()});
        return data.to_string();
    }
}