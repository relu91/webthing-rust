use actix;
//use actix::prelude::*;
use actix_service::{Service, Transform};
use actix_web;
use actix_web::dev::{Server, ServiceRequest, ServiceResponse};
use actix_web::guard;
use actix_web::http::HeaderValue;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;use futures::future::{ok, Either, Ready};
use hostname;
use libmdns;
#[cfg(feature = "ssl")]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
//use serde_json;
//use serde_json::json;
use std::marker::{Send, Sync};
use std::sync::{Arc, RwLock/*, Weak*/};
use std::task::{Context, Poll};
//use std::time::Duration;
//use uuid::Uuid;
use std::clone::Clone;
//use std::collections::BTreeSet;
use std::collections::BTreeMap;
//use url::String;
use super::objects::thing_object::ThingObject;
use super::affordances::interaction_affordance::{InteractionAffordance};
use super::affordances::property_affordance::{PropertyAffordance};
use super::affordances::form::{Form, FormOperationType};

//use super::affordances::thing_description::ThingDescription;
const SERVICE_TYPE: &str = "_webthing._tcp";

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
    things                  : BTreeMap<String, ThingObject>,
    hosts                   : Vec<String>,
    disable_host_validation: bool,
    registered_props        : BTreeMap<String,ThingEndpointInfo>,
    registered_acts         : BTreeMap<String,ThingEndpointInfo>,
    registered_evts         : BTreeMap<String,ThingEndpointInfo>,   
    registered_base_forms   : BTreeMap<String,String>
}
impl AppState { 
    fn validate_host(&self,host: Option<&HeaderValue>) -> Result<(), ()> {
        if self.disable_host_validation {
            Ok(())
        } else if host.is_none() {
            Err(())
        } else {
            match host.unwrap().to_str() {
                Ok(host) => {
                    if self.hosts.contains(&host.to_lowercase()) {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                Err(_) => Err(()),
            }
        }
    }
}

//host validator
struct HostValidator;

impl<S, B> Transform<S> for HostValidator
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = HostValidatorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(HostValidatorMiddleware { service })
    }
}

struct HostValidatorMiddleware<S> {
    service: S,
}

impl<S, B> Service for HostValidatorMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {

//        Either::Left(self.service.call(req))
        
        let state = req.app_data::<web::Data<Arc<AppState>>>();

        if state.is_none() {
            return Either::Right(ok(
                req.into_response(HttpResponse::Forbidden().finish().into_body())
            ));
        }

        let state = state.unwrap();

        let host = req.headers().get("Host");

        let res = state.validate_host(host);

        match res  {
            Ok(_) => Either::Left(self.service.call(req)),
            Err(_) => Either::Right(ok(
                req.into_response(HttpResponse::Forbidden().finish().into_body())
            )),
        }
        
    }
}

/*
pub enum ThingObjectType {
    totAction,
    totEvent,
    totProperty
}
*/

///1
pub struct ThingServer  {
    base_path: Arc<String>,
    port: Arc<Option<u16>>,
    hostname: Arc<Option<String>>,
    dns_service: Arc<Option<libmdns::Service>>,
    #[allow(dead_code)]
    ssl_options: Arc<Option<(String, String)>>,
    //app_state : Arc<RwLock<AppState>>
    app_state : Arc<AppState>

    //generator_arc: Arc<Box<dyn ActionGenerator>>,
}

impl Clone for ThingServer {
    
    fn clone(&self) -> Self {
        ThingServer {
            base_path   : self.base_path.clone(),
            port        : self.port.clone(),
            hostname    : self.hostname.clone(),
            dns_service : self.dns_service.clone(),
            #[allow(dead_code)]
            ssl_options : self.ssl_options.clone(),
            app_state   : self.app_state.clone()
        }
    }
}
//event handling through plain GET/POST/PUT
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
//property handling through plain GET/POST/PUT
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
    let app = state.as_ref().clone();
    let u = req.path();

    if app.registered_props.contains_key(u) == false {
        return HttpResponse::NotFound().finish();
    }

    
    let r  = app.registered_props.get(u);
    if r.is_none() {
        return HttpResponse::NotFound().finish();
    }
    let thing_info = r.unwrap();
    
    let thing_name : &String = &thing_info.thing_name;
    let obj_name : &String = &thing_info.object_name;

    let s_thing_obj = app.things.get(thing_name);

    if s_thing_obj.is_none() {
        return HttpResponse::NotFound().finish();
    }


    let thing_obj = &s_thing_obj.unwrap();

    //go into forms

    let s_po = thing_obj.get_property(obj_name);
    
    if s_po.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let po = s_po.unwrap();

    let def = po.get_definition();

    let mut thisMethod  : Option<String> = None;
    let mut opid : Option<FormOperationType> = None;
    let mut thisForm : Option<&Form> = None;

    let mut found : bool = false;

    for f in def.get_forms() {
        let thisPath = f.get_href().to_string();
        if thisPath == u {
            thisMethod = f.get_method_name().clone();
            if thisMethod.is_none() {
                let opList = f.get_operation_list();
                if opList.len() > 0 {
                    let opid = opList.iter().next().unwrap();
                    match opid {
                        FormOperationType::ReadProperty => thisMethod = Some("GET".to_string()),
                        FormOperationType::WriteProperty => thisMethod = Some("PUT".to_string()),
                        _ => ()
                    }


                }
            }

            if thisMethod.is_some() && thisMethod.unwrap() == method {
                found = true;
                thisForm = Some(&f);
                break;
            }

        }
    }

    if found == false {
        return HttpResponse::NotFound().finish();
    }

    //now, do some access checking



    //eventually build response
    let value = po.get_value();
    let mut ret = serde_json::Map::new();
    
    if value.is_some() {
        ret.insert(obj_name.to_string(), value.clone().unwrap());
    }else {
        ret.insert(obj_name.to_string(), serde_json::Value::Null);
    }

    HttpResponse::Ok().json(ret)


//    HttpResponse::NotFound().finish()
    
}
//action handling through plain GET/POST/PUT
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
//root form handling through plain GET/POST/PUT
fn handle_get_base_form(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_base_form(req,state,"GET".to_string())
}
fn handle_post_base_form(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_base_form(req,state,"POST".to_string())
}
fn handle_put_base_form(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    handle_base_form(req,state,"PUT".to_string())        
}

fn handle_base_form(req: HttpRequest, state: web::Data<Arc<AppState>>, method : String) -> HttpResponse {
    HttpResponse::NotFound().finish()
}
/// Handle websocket on /.
async fn handle_ws_thing(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound().finish())
/*    
    let u = req.
    u.to_ur
    if state.registered_evts
*/
/*    
    let thing_id = req.match_info().get("thing_id");

    match state.get_thing(thing_id) {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(thing) => {
            let thing_id = match thing_id {
                None => 0,
                Some(id) => id.parse::<usize>().unwrap(),
            };
            let ws = ThingWebSocket {
                id: Uuid::new_v4().to_string(),
                thing_id: thing_id,
                things: state.get_things(),
                action_generator: state.get_action_generator(),
            };
            thing.write().unwrap().add_subscriber(ws.get_id());
            ws::start(ws, &req, stream)
        }
    
    }
*/    
}    

impl ThingServer {
    ///1
    ///1
    pub fn new(
        base_path:  String,
        disable_host_validation : bool,
        port:       Option<u16>,
        hostname:   Option<String>,
        #[allow(dead_code)]
        ssl_options: Option<(String, String)>,
        objs       : BTreeMap<String, ThingObject>
    ) -> Self {
        let mut  ret = ThingServer {
            base_path   :   Arc::new(base_path),
            port        :   Arc::new(port) ,
            hostname    :   Arc::new(hostname),
            ssl_options :   Arc::new(ssl_options),
            dns_service :   Arc::new(None),
            app_state   :   Arc::new( 
                AppState { 
                    things: objs,
                    hosts: Vec::new(),
                    disable_host_validation: disable_host_validation,
                    registered_acts : BTreeMap::new(),
                    registered_props : BTreeMap::new(),
                    registered_evts: BTreeMap::new(),
                    registered_base_forms : BTreeMap::new()
                }  
            )
        };

        let app_state = Arc::get_mut(&mut ret.app_state).unwrap();


        //loads configured urls
        for (s,to) in app_state.things.iter() {
            let td = to.get_thing_description();

            for (n,p) in td.get_properties().iter() {
                for f in p.get_forms().iter() {
                    let u  = f.get_href();
                    app_state.registered_props.insert(u.to_string(),ThingEndpointInfo::new(s,n));
                }
            }

            for (n,a) in td.get_actions().iter() {
                for f in a.get_forms().iter() {
                    let u  = f.get_href();
                    app_state.registered_acts.insert(u.to_string(),ThingEndpointInfo::new(s,n));
                }

            }

            for (n,e) in td.get_events().iter() {
                for f in e.get_forms().iter() {
                    let u  = f.get_href();
                    app_state.registered_evts.insert(u.to_string(),ThingEndpointInfo::new(s,n));
                    

                }

            }

            //and base forms
            for f in td.get_forms().iter() {
                let u  = f.get_href();
                app_state.registered_base_forms.insert(u.to_string(),s.to_string());
            }
        }


        ret
    }

    ///1
    pub fn start(
        &mut self,
        configure: Option<Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync + 'static>>
    ) -> Server {
        let port = match *self.port {
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
        if self.hostname.is_some() {
            let name = self.clone().hostname.as_ref().clone().unwrap().to_lowercase();
            hosts.push(name.clone());
            hosts.push(format!("{}:{}", name, port));
        }

        
        //let bp = self.base_path.clone();
        let app_state = self.app_state.clone();
        
        let http_server = HttpServer::new(move || { 
            
            let mut web_app_factory =  App::new()
                .data(app_state.clone())
                .app_data(app_state.clone())
                .wrap(middleware::Logger::default())
                .wrap(HostValidator)
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
    
/*    

            let webAppFactory  = if configure.is_some() {
                let configure = configure.clone().unwrap();
                unsafe { webAppFactory.configure(&*Arc::into_raw(configure)) }
            } else {
                webAppFactory 
            };
*/                   
            
           // let  x = webAppFactory.service(web::resource("/"));

            //loads url

                        

            //register all routes
            for (u,_t) in &app_state.registered_acts {
                let s = &u.to_string();
                web_app_factory = web_app_factory.service(
                    web::resource(s)
                    .route(web::get().to(handle_get_action))
                    .route(web::put().to(handle_put_action))
                    .route( web::post().to(handle_post_action))
                );

            }

            for (u,_t) in &app_state.registered_props {
                
                let k : String  = u.to_string();
                web_app_factory = web_app_factory.service(
                    web::resource(&k)
                    .route(web::get().to(handle_get_property))
                    .route(web::put().to(handle_put_property))
                    .route( web::post().to(handle_post_property))
                );


            }
            for (u,_t) in &app_state.registered_evts {
                let s = &u.to_string();
                web_app_factory = web_app_factory.service(
                    web::resource(s)
                    .route(web::get().to(handle_get_event))
                    .route(web::put().to(handle_put_event))
                    .route( web::post().to(handle_post_event))
                    .route(
                        web::route()
                            .guard(guard::Get())
                            .guard(guard::Header("upgrade", "websocket"))
                            .to(handle_ws_thing)
                    )
                );
                //for event, adds also web socket handling



            }

            for (u,_t) in &app_state.registered_base_forms {
                let s = &u.to_string();
                web_app_factory = web_app_factory.service(
                    web::resource(s)
                    .route(web::get().to(handle_get_base_form))
                    .route(web::put().to(handle_put_base_form))
                    .route( web::post().to(handle_post_base_form))
                );

            }

            web_app_factory//.service(web::resource("/"))

            
        });
        

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
            self.dns_service = Arc::new(
                Some(responder.register(SERVICE_TYPE.to_owned(), "WSIOT".to_string(), port, &["path=/"])));
            
            http_server
                .bind(format!("0.0.0.0:{}", port))
                .expect("Failed to bind socket")
                .run()
        }
     
    }
}

