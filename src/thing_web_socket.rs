/*
use actix;
use actix::prelude::*;
use actix_service::{Service, Transform};
use actix_web;
use actix_web::dev::{Server, ServiceRequest, ServiceResponse};
use actix_web::guard;
use actix_web::http::HeaderValue;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::Duration;


struct ThingWebSocket {
    id: String,
    thing_id: usize
}

impl Actor for ThingWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl ThingWebSocket {
    /// Get the ID of this websocket.
    fn get_id(&self) -> String {
        self.id.clone()
    }

     /// Drain all message queues associated with this websocket.
    fn drain_queue(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_later(Duration::from_millis(200), |act, ctx| {
            let thing = act.get_thing();
            let mut thing = thing.write().unwrap();

            let drains = thing.drain_queue(act.get_id());
            for iter in drains {
                for message in iter {
                    ctx.text(message);
                }
            }

            act.drain_queue(ctx);
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ThingWebSocket {
    fn started(&mut self, ctx: &mut Self::Context) {
        self.drain_queue(ctx);
    }

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Text(text)) => {
                let message = serde_json::from_str(&text);
                if message.is_err() {
                    return ctx.text(
                        r#"
                        {
                            "messageType": "error",
                            "data": {
                                "status": "400 Bad Request",
                                "message": "Parsing request failed"
                            }
                        }"#,
                    );
                }

                let message: serde_json::Value = message.unwrap();
                if !message.is_object() {
                    return ctx.text(
                        r#"
                        {
                            "messageType": "error",
                            "data": {
                                "status": "400 Bad Request",
                                "message": "Parsing request failed"
                            }
                        }"#,
                    );
                }

                let message = message.as_object().unwrap();

                if !message.contains_key("messageType") || !message.contains_key("data") {
                    return ctx.text(
                        r#"
                        {
                            "messageType": "error",
                            "data": {
                                "status": "400 Bad Request",
                                "message": "Invalid message"
                            }
                        }"#,
                    );
                }

                let msg_type = message.get("messageType").unwrap().as_str();
                let data = message.get("data").unwrap().as_object();
                if msg_type.is_none() || data.is_none() {
                    return ctx.text(
                        r#"
                        {
                            "messageType": "error",
                            "data": {
                                "status": "400 Bad Request",
                                "message": "Invalid message"
                            }
                        }"#,
                    );
                }

                let msg_type = msg_type.unwrap();
                let data = data.unwrap();
                let thing = self.get_thing();

                match msg_type {
                    "setProperty" => {
                        for (property_name, property_value) in data.iter() {
                            let result = thing
                                .write()
                                .unwrap()
                                .set_property(property_name.to_string(), property_value.clone());

                            if result.is_err() {
                                let err = result.unwrap_err();
                                return ctx.text(format!(
                                    r#"
                                    {{
                                        "messageType": "error",
                                        "data": {{
                                            "status": "400 Bad Request",
                                            "message": "{}"
                                        }}
                                    }}"#,
                                    err
                                ));
                            }
                        }
                    }
                    "requestAction" => {
                        for (action_name, action_params) in data.iter() {
                            let input = action_params.get("input");
                            let action = self.action_generator.generate(
                                Arc::downgrade(&self.get_thing()),
                                action_name.to_string(),
                                input,
                            );

                            if action.is_none() {
                                return ctx.text(format!(
                                    r#"
                                {{
                                    "messageType": "error",
                                    "data": {{
                                        "status": "400 Bad Request",
                                        "message": "Invalid action request",
                                        "request": {}
                                    }}
                                }}"#,
                                    text
                                ));
                            }

                            let action = action.unwrap();
                            let id = action.get_id();
                            let action = Arc::new(RwLock::new(action));

                            {
                                let mut thing = thing.write().unwrap();
                                let result = thing.add_action(action.clone(), input);

                                if result.is_err() {
                                    return ctx.text(format!(
                                        r#"
                                    {{
                                        "messageType": "error",
                                        "data": {{
                                            "status": "400 Bad Request",
                                            "message": "Failed to start action: {}"
                                        }}
                                    }}"#,
                                        result.unwrap_err()
                                    ));
                                }
                            }

                            thing
                                .write()
                                .unwrap()
                                .start_action(action_name.to_string(), id);
                        }
                    }
                    "addEventSubscription" => {
                        for event_name in data.keys() {
                            thing
                                .write()
                                .unwrap()
                                .add_event_subscriber(event_name.to_string(), self.get_id());
                        }
                    }
                    unknown => {
                        return ctx.text(format!(
                            r#"
                            {{
                                "messageType": "error",
                                "data": {{
                                    "status": "400 Bad Request",
                                    "message": "Unknown messageType: {}"
                                }}
                            }}"#,
                            unknown
                        ));
                    }
                }
            }
            Ok(ws::Message::Close(_)) => {
                let thing = self.get_thing();
                thing.write().unwrap().remove_subscriber(self.get_id());
            }
            _ => (),
        }
    }
}
*/