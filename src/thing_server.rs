
use actix;
use actix::prelude::*;
use actix_service::{Service, Transform};
use actix_web;
use actix_web::dev::{Server, ServiceRequest, ServiceResponse};
use actix_web::guard;
use actix_web::http::HeaderValue;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use futures::future::{ok, Either, Ready};
use hostname;
use libmdns;
#[cfg(feature = "ssl")]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde_json;
use serde_json::json;
use std::marker::{Send, Sync};
use std::sync::{Arc, RwLock, Weak};
use std::task::{Context, Poll};
use std::time::Duration;
use uuid::Uuid;
use std::clone::Clone;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use url::Url;
use super::objects::thing_object::ThingObject;
use super::affordances::thing_description::ThingDescription;

#[derive(Debug,Clone)]
struct ThingEndpointInfo {
    thing_name : String,
    object_name: String 
}
impl ThingEndpointInfo {
    pub fn new (tn : &String, on : &String ) -> Self  {
        ThingEndpointInfo {
            thing_name : tn.clone(),
            object_name : on.clone()
        }
    }
}
struct AppState {
    things: BTreeMap<String, ThingObject>,
    hosts: Vec<String>,
    disable_host_validation: bool,
    registered_props        : BTreeMap<Url,ThingEndpointInfo>,
    registered_acts         : BTreeMap<Url,ThingEndpointInfo>,
    registered_evts         : BTreeMap<Url,ThingEndpointInfo>,   
    registered_base_forms   : BTreeMap<Url,String>
}



pub enum ThingObjectType {
    totAction,
    totEvent,
    totProperty
}


pub struct ThingServer  {
    base_path: String,
    port: Option<u16>,
    hostname: Option<String>,
    dns_service: Option<libmdns::Service>,
    #[allow(dead_code)]
    ssl_options: Option<(String, String)>,
    app_state : Arc<AppState>

    //generator_arc: Arc<Box<dyn ActionGenerator>>,
}

fn handle_get_event(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_event(req,state,"GET".to_string())
}
fn handle_post_event(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_event(req,state,"POST".to_string())
}
fn handle_put_event(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_event(req,state,"PUT".to_string())   
}

fn handle_event(req: HttpRequest, state: web::Data<Arc<AppState>>, method : String) -> HttpResponse {
    HttpResponse::NotFound().finish()
}

fn handle_get_property(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_property(req,state,"GET".to_string())
}
fn handle_post_property(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_property(req,state,"POST".to_string())
}
fn handle_put_property(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_property(req,state,"PUT".to_string())   
}

fn handle_property(req: HttpRequest, state: web::Data<Arc<AppState>>, method : String) -> HttpResponse {
    HttpResponse::NotFound().finish()
}

fn handle_get_action(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_action(req,state,"GET".to_string())
}
fn handle_post_action(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_action(req,state,"POST".to_string())
}
fn handle_put_action(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_action(req,state,"PUT".to_string())        
}

fn handle_action(req: HttpRequest, state: web::Data<Arc<AppState>>, method : String) -> HttpResponse {
    HttpResponse::NotFound().finish()
}

impl ThingServer {
    pub fn get_request_type(&self, url : &Url) -> Option<ThingObjectType> {
        let mut ret :  Option<ThingObjectType>  = None;
        let s = url.to_string();

        if self.app_state.registered_acts.contains_key(url) {
            ret = Some(ThingObjectType::totAction);
        } 

        if ret.is_none() && self.app_state.registered_props.contains_key(url) {
            ret = Some(ThingObjectType::totProperty);
        }

        if ret.is_none() && self.app_state.registered_evts.contains_key(url) {
            ret = Some(ThingObjectType::totEvent);
        }

        ret
    }

    pub fn start(
        &mut self,
        configure: Option<Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync + 'static>>,
    ) {
        let port = match self.port {
            Some(p) => p,
            None => 80,
        };

        let mut hosts = vec!["localhost".to_owned(), format!("localhost:{}", port)];

        let system_hostname = hostname::get();
        if system_hostname.is_ok() {
            let name = system_hostname
                .unwrap()
                .into_string()
                .unwrap()
                .to_lowercase();
            hosts.push(format!("{}.local", name));
            hosts.push(format!("{}.local:{}", name, port));
        }
/*
        for address in get_addresses() {
            hosts.push(address.clone());
            hosts.push(format!("{}:{}", address, port));
        }
*/
        if self.hostname.is_some() {
            let name = self.hostname.clone().unwrap().to_lowercase();
            hosts.push(name.clone());
            hosts.push(format!("{}:{}", name, port));
        }
/*
        let name = match &self.things {
            ThingsType::Single(thing) => thing.read().unwrap().get_title(),
            ThingsType::Multiple(_, name) => name.to_owned(),
        };

        match &mut self.things {
            ThingsType::Multiple(ref mut things, _) => {
                for (idx, thing) in things.iter_mut().enumerate() {
                    let mut thing = thing.write().unwrap();
                    thing.set_href_prefix(format!("{}/{}", self.base_path, idx));
                }
            }
            ThingsType::Single(ref mut thing) => {
                thing
                    .write()
                    .unwrap()
                    .set_href_prefix(self.base_path.clone());
            }
        }

        let single = match &self.things {
            ThingsType::Multiple(_, _) => false,
            ThingsType::Single(_) => true,
        };
*/        

        let bp = self.base_path.clone();
        let server = HttpServer::new(move || {
            
        let bp = bp.clone();
        let app = App::new()
            .data(self.app_state)
            .wrap(middleware::Logger::default())
            //.wrap(HostValidator)
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("Access-Control-Allow-Origin", "*")
                    .header(
                        "Access-Control-Allow-Methods",
                        "GET, HEAD, PUT, POST, DELETE, OPTIONS",
                    )
                    .header(
                        "Access-Control-Allow-Headers",
                        "Origin, Content-Type, Accept, X-Requested-With",
                    ),
            );

        let app = if configure.is_some() {
            let configure = configure.clone().unwrap();
            unsafe { app.configure(&*Arc::into_raw(configure)) }
        } else {
            app
        };

        //load list of url 
        for (s,to) in self.app_state.things.iter() {
            let td = to.get_thing_description();

            for (n,p) in td.get_properties().iter() {
                for f in p.get_forms().iter() {
                    let u  = f.get_href();
                    self.app_state.registered_props.insert(u.clone(),ThingEndpointInfo::new(s,n));
                }
            }

            for (n,a) in td.get_actions().iter() {
                for f in a.get_forms().iter() {
                    let u  = f.get_href();
                    self.app_state.registered_acts.insert(u.clone(),ThingEndpointInfo::new(s,n));
                }

            }

            for (n,e) in td.get_events().iter() {
                for f in e.get_forms().iter() {
                    let u  = f.get_href();
                    self.app_state.registered_evts.insert(u.clone(),ThingEndpointInfo::new(s,n));
                }

            }

            //and base forms
            for f in td.get_forms().iter() {
                let u  = f.get_href();
                self.app_state.registered_base_forms.insert(u.clone(),s.to_string());
            }
        }

        //register all routes
        for (u,t) in self.app_state.registered_acts {
            app.route(&u.to_string(), web::get().to(handle_get_action));
            app.route(&u.to_string(), web::put().to(handle_put_action));
            app.route(&u.to_string(), web::post().to(handle_post_action));

        }

        for (u,t) in self.app_state.registered_props {
            app.route(&u.to_string(), web::get().to(handle_get_property));
            app.route(&u.to_string(), web::put().to(handle_put_property));
            app.route(&u.to_string(), web::post().to(handle_post_property));

        }
        for (u,t) in self.app_state.registered_evts {
            app.route(&u.to_string(), web::get().to(handle_get_event));
            app.route(&u.to_string(), web::put().to(handle_put_event));
            app.route(&u.to_string(), web::post().to(handle_post_event));

        }

        for u in self.app_state.registered_base_forms {
            //TO DO!!
        }
            


        let responder = libmdns::Responder::new().unwrap();

        #[cfg(feature = "ssl")]
        match self.ssl_options {
            Some(ref o) => {
                self.dns_service = Some(responder.register(
                    SERVICE_TYPE.to_owned(),
                    name.clone(),
                    port,
                    &["path=/", "tls=1"],
                ));

                let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
                builder
                    .set_private_key_file(o.0.clone(), SslFiletype::PEM)
                    .unwrap();
                builder.set_certificate_chain_file(o.1.clone()).unwrap();
                server
                    .bind_openssl(format!("0.0.0.0:{}", port), builder)
                    .expect("Failed to bind socket")
                    .run()
            }
            None => {
                self.dns_service = Some(responder.register(
                    SERVICE_TYPE.to_owned(),
                    name.clone(),
                    port,
                    &["path=/"],
                ));
                server
                    .bind(format!("0.0.0.0:{}", port))
                    .expect("Failed to bind socket")
                    .run()
            }
        }

        #[cfg(not(feature = "ssl"))]
        {
            self.dns_service =
                Some(responder.register(SERVICE_TYPE.to_owned(), name.clone(), port, &["path=/"]));
            server
                .bind(format!("0.0.0.0:{}", port))
                .expect("Failed to bind socket")
                .run()
        }
    }

    }
}

