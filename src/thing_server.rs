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

use futures::future::{ok, Either, Ready};
use hostname;
use libmdns;
use uuid::Uuid;


#[cfg(feature = "ssl")]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::marker::{Send, Sync};
use std::sync::{Arc , RwLock,RwLockWriteGuard};
use std::task::{Context, Poll};
use std::clone::Clone;
use std::collections::BTreeMap;

use super::objects::thing_object::ThingObject;
use super::affordances::interaction_affordance::{InteractionAffordance};
//use super::affordances::property_affordance::{PropertyAffordance};
use super::affordances::form::{Form, FormOperationType};
use super::objects::property_object::PropertyObject;
use super::objects::event_object::EventObject;
use super::objects::action_object::ActionObject;
use super::objects::notifiable_object::NotifiableObject;
use super::objects::observable_object::ObservableObject;


use web::{Bytes, post, Query};
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
    things                  : BTreeMap<String, Arc<RwLock<ThingObject>>>,
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
        
        let state = req.app_data::<web::Data<Arc<RwLock<AppState>>>>();

        if state.is_none() {
            return Either::Right(ok(
                req.into_response(HttpResponse::Forbidden().finish().into_body())
            ));
        }

        let state = state.unwrap();

        let host = req.headers().get("Host");

        let res = state.read().unwrap().validate_host(host);

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
    app_state : Arc<RwLock<AppState>>

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
//property handling through plain GET/POST/PUT
fn handle_get_property(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>, bytes : Bytes) -> HttpResponse {
    handle_property(req,state,"GET".to_string(), bytes)
}
fn handle_post_property(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>, bytes : Bytes) -> HttpResponse {
    handle_property(req,state,"POST".to_string(), bytes)
}
fn handle_put_property(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>, bytes : Bytes) -> HttpResponse {
    handle_property(req,state,"PUT".to_string(), bytes   )
}

fn handle_property(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>, method : String, bytes : Bytes) -> HttpResponse {
    let mut app : &mut AppState = &mut state.as_ref().write().unwrap();
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

    let  s_thing_obj = app.things.get_mut(thing_name);

    if s_thing_obj.is_none() {
        return HttpResponse::NotFound().finish();
    }


    let  w_thing_obj = &mut s_thing_obj.unwrap();

    //go into forms

    let mut thing_obj =  w_thing_obj.write().unwrap();
    let s_po : Option<&mut PropertyObject> =  thing_obj.get_property_mut(obj_name);
    
    if s_po.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let mut po = s_po.unwrap();

    let def = po.get_definition();
    //do some access checking

    


    let mut this_method  : Option<String> = None;
    let mut opid : Option<FormOperationType> = None;
    let mut this_form : Option<&Form> = None;

    let mut found : bool = false;

    for f in def.get_forms() {
        let this_path = f.get_href().to_string();
        if this_path == u {
            this_method = f.get_method_name().clone();
            if this_method.is_none() {
                let op_list = f.get_operation_list();
                if op_list.len() > 0 {
                    let zopid = op_list.iter().next().unwrap();
                    match zopid {
                        FormOperationType::ReadProperty => this_method = Some("GET".to_string()),
                        FormOperationType::WriteProperty => this_method = Some("PUT".to_string()),
                        _ => ()
                    }
                    opid = Some(zopid);
                    


                }
            }

            if this_method.is_some() && this_method.unwrap() == method {
                found = true;
                this_form = Some(&f);
                break;
            }

        }
    }

    if found == false {
        return HttpResponse::NotFound().finish();
    }

    //now, do some access checking
    let mut ro : bool = false;
    let mut wo : bool = false;

    match def.get_readonly()  {
        None => ro = false,
        Some(x) => ro = x,
    }
    match def.get_writeonly()  {
        None => wo = false,
        Some(x) => wo = x,
    }
    
    if opid.is_some() {
        if (opid.unwrap() == FormOperationType::WriteProperty && ro == true) ||
            (opid.unwrap() == FormOperationType::ReadProperty && wo == true ) {
            return HttpResponse::Forbidden().finish();
        }
    }

    //eventually build response
    if opid.unwrap() ==  FormOperationType::WriteProperty {
        let bodyRes = String::from_utf8(bytes.to_vec());
        if bodyRes.is_err() {
            return HttpResponse::BadRequest().finish();
        }
        let body = bodyRes.unwrap();
        
        let parsed  : serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");

        po.set_value(&Some(parsed));




    }
    
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
fn handle_get_action(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>, bytes : Bytes) -> HttpResponse {
    handle_action(req,state,"GET".to_string(),bytes)
}
fn handle_post_action(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>, bytes : Bytes) -> HttpResponse {
    handle_action(req,state,"POST".to_string(),bytes)
}
fn handle_put_action(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>, bytes : Bytes) -> HttpResponse {
    handle_action(req,state,"PUT".to_string(),bytes)        
}

fn handle_action(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>, method : String, bytes : Bytes) -> HttpResponse {
    let mut app : &mut AppState = &mut state.as_ref().write().unwrap();
    let u = req.path();



    if app.registered_acts.contains_key(u) == false {
        return HttpResponse::NotFound().finish();
    }

    
    let r  = app.registered_acts.get(u);
    if r.is_none() {
        return HttpResponse::NotFound().finish();
    }
    let thing_info = r.unwrap();
    
    let thing_name : &String = &thing_info.thing_name;
    let obj_name : &String = &thing_info.object_name;

    let     s_thing_obj = app.things.get_mut(thing_name);

    if s_thing_obj.is_none() {
        return HttpResponse::NotFound().finish();
    }


    let      w_thing_obj = &s_thing_obj.unwrap();
    let      r_thing_obj =  w_thing_obj.read().unwrap();

    let     thing_obj : &ThingObject = &r_thing_obj;

    //go into forms
    
    let  s_po : Option<& ActionObject> =  thing_obj.get_action(obj_name);
    
    if s_po.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let po = s_po.unwrap();

    let def = po.get_definition();
    //do some access checking

    


    let mut this_method  : Option<String> = None;
    let mut opid : Option<FormOperationType> = None;
    let mut this_form : Option<&Form> = None;

    let mut found : bool = false;

    for f in def.get_forms() {
        let this_path = f.get_href().to_string();
        if this_path == u {
            this_method = f.get_method_name().clone();
            if this_method.is_none() {
                this_method = Some("POST".to_string());
            }

            if this_method.is_some() && this_method.unwrap() == method {
                found = true;
                this_form = Some(&f);
                break;
            }

        }
    }

    if found == false {
        return HttpResponse::NotFound().finish();
    }

    //try to do something
    po.handle(&r_thing_obj);

    HttpResponse::Ok().finish()

}
//root form handling through plain GET/POST/PUT
fn handle_get_base_form(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>) -> HttpResponse {
    handle_base_form(req,state,"GET".to_string())
}
fn handle_post_base_form(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>) -> HttpResponse {
    handle_base_form(req,state,"POST".to_string())
}
fn handle_put_base_form(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>) -> HttpResponse {
    handle_base_form(req,state,"PUT".to_string())        
}

fn handle_base_form(req: HttpRequest, state: web::Data<Arc<RwLock<AppState>>>, method : String) -> HttpResponse {
    HttpResponse::NotFound().finish()
}
/// Handle websocket on /.
async fn handle_ws_thing(
    req: HttpRequest,
    state: web::Data<Arc<RwLock<AppState>>>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {

    let app : &mut AppState = &mut state.as_ref().write().unwrap();
    let u = req.path();
    


    if app.registered_evts.contains_key(u) == false {
        return Ok(HttpResponse::NotFound().finish());
    }

    
    let r  = app.registered_evts.get(u);
    if r.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let thing_info = r.unwrap();
    
    let thing_name : &String = &thing_info.thing_name;
    let obj_name : &String = &thing_info.object_name;

    let s_thing_obj = app.things.get_mut(thing_name);

    if s_thing_obj.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }


    let  thing_obj = &mut s_thing_obj.unwrap();
    let mut to = thing_obj.write().unwrap();
    //go into forms

    let s_po : Option<&mut EventObject> =  to.get_event_mut(obj_name);
    
    if s_po.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let mut po = s_po.unwrap();

    let def = po.get_definition();

    let thing_id = req.match_info().get("thing_id");

    let thing_id = match thing_id {
        None => 0,
        Some(id) => id.parse::<usize>().unwrap(),
    };
    let ws = ThingWebSocket {
        id: Uuid::new_v4().to_string(),
        thing_id: thing_id,
        thing_name : thing_info.thing_name.clone(),
        object_name : thing_info.object_name.clone(),
        app_state : state.as_ref().clone(),
        url : u.to_string().clone()
    };

    

    ws::start(ws, &req, stream)



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
        objs       : BTreeMap<String, Arc<RwLock<ThingObject>>>
    ) -> Self {
        let mut  ret = ThingServer {
            base_path   :   Arc::new(base_path),
            port        :   Arc::new(port) ,
            hostname    :   Arc::new(hostname),
            ssl_options :   Arc::new(ssl_options),
            dns_service :   Arc::new(None),
            app_state   :   Arc::new( 
                RwLock::new(
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
            )
        };

        let mut app_state : &mut AppState = &mut ret.app_state.write().unwrap();


        //loads configured urls
        

        for (s,to) in  app_state.things.iter_mut() {
            let mtt = &mut to.write().unwrap();
            let mttt = &mut mtt.get_thing_description();

            let td = &mut mttt.read().unwrap();

            for (np,p) in td.get_properties().iter() {
                for fp in p.get_forms().iter() {
                    let up  = fp.get_href();
                    app_state.registered_props.insert(up.to_string(),ThingEndpointInfo::new(s,np));
                }
            }

            for (na,a) in td.get_actions().iter() {
                for fa in a.get_forms().iter() {
                    let ua  = fa.get_href();
                    app_state.registered_acts.insert(ua.to_string(),ThingEndpointInfo::new(s,na));
                }

            }

            for (ne,e) in td.get_events().iter() {
                for fe in e.get_forms().iter() {
                    let ue = fe.get_href();
                    app_state.registered_evts.insert(ue.to_string(),ThingEndpointInfo::new(s,ne));
                    

                }

            }

            //and base forms
            for ff in td.get_forms().iter() {
                let uf  = ff.get_href();
                app_state.registered_base_forms.insert(uf.to_string(),s.to_string());
            }
        }


        ret.clone()
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
                ) ;

    
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
            for (u,_t) in &app_state.read().unwrap().registered_acts {
                let s = &u.to_string();
                web_app_factory = web_app_factory.service(
                    web::resource(s)
                    .route(web::get().to(handle_get_action))
                    .route(web::put().to(handle_put_action))
                    .route( web::post().to(handle_post_action))
                );

            }

            for (u,_t) in &app_state.read().unwrap().registered_props {
                
                let k : String  = u.to_string();
                web_app_factory = web_app_factory.service(
                    web::resource(&k)
                    .route(web::get().to(handle_get_property))
                    .route(web::put().to(handle_put_property))
                    .route( web::post().to(handle_post_property))
                );


            }
            for (u,_t) in &app_state.read().unwrap().registered_evts {
                let s = &u.to_string();
                web_app_factory = web_app_factory.service(
                    web::resource(s)
                    //.route(web::get().to(handle_get_event))
                    //.route(web::put().to(handle_put_event))
                    //.route( web::post().to(handle_post_event))
                    .route(
                        web::route()
                            .guard(guard::Get())
                            .guard(guard::Header("upgrade", "websocket"))
                            .to(handle_ws_thing)
                    )
                );
                //for event, adds also web socket handling



            }

            for (u,_t) in &app_state.read().unwrap().registered_base_forms {
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

// WEB SOCKET HANDLING
///1
pub struct ThingWebSocket {
    id: String,
    thing_id: usize,
    thing_name: String,
    object_name : String,
    url : String,
    app_state: Arc<RwLock<AppState>>
    
}

impl Actor for ThingWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl ThingWebSocket {
    /// Drain all message queues associated with this websocket.
    fn drain_queue(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let name = self.thing_name.clone();
        let object_name = self.object_name.clone();
        

        ctx.run_later(Duration::from_millis(200), move |act, ctx| {
            let s_appstate = act.app_state.clone();
            let appstate = s_appstate.write().unwrap();
            let mut thing = appstate.things.get(&name).unwrap().write().unwrap();

            let mut evt : &EventObject = thing.get_event_mut(&object_name).unwrap();
            

            //let mut thing = thing.write().unwrap();

            let drains = thing.drain_queue(act.get_id(),&object_name);
            for message in drains {
                ctx.text(message);
            }
            act.drain_queue(ctx);
        });
    }
    

    /// Get the ID of this websocket.
    fn get_id(&self) -> String {
        self.id.clone()
    }


    fn remove_subscriber(&mut self) {
         
        let mut app_state : &mut AppState = &mut self.app_state.write().unwrap();

        let u : &String = &mut self.url.clone();

        if app_state.registered_props.contains_key(u) {
            let a_thing : Arc<RwLock<ThingObject>>   = match app_state.things.get_mut(&self.thing_name) {
                None => return ,
                Some(x) =>  x.clone()
            };
   
            let thing : &mut ThingObject = &mut a_thing.write().unwrap();

            let property : &mut PropertyObject = match thing.get_property_mut(&self.object_name) {
                None => return,
                Some(x) => x
            };
    
            property.remove_subscriber(&self.id);
    
        }

        if app_state.registered_evts.contains_key(u) {

            let a_thing : Arc<RwLock<ThingObject>>   = match app_state.things.get_mut(&self.thing_name) {
                None => return ,
                Some(x) =>  x.clone()
            };
   
            let thing : &mut ThingObject = &mut a_thing.write().unwrap();    
            let event : &mut EventObject = match thing.get_event_mut(&self.object_name) {
                None => return,
                Some(x) => x
            };
    
            event.remove_subscriber(&self.id);
    
        }

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
                let parsed_res : Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&text);
                if parsed_res.is_ok() {
                    let parsed  = parsed_res.unwrap();
                    let opt_type = parsed.get("type");

                    let mut type_str = "";

                    if opt_type.is_some() {
                        let v = opt_type.unwrap();
                        if v.is_string() {
                            type_str = v.as_str().unwrap().clone();
                        }                        
                    }

                    if type_str == "subscribeevent" {
                        let app_state : &mut AppState = &mut self.app_state.write().unwrap();
                        let dezire = app_state.things.get_mut(&self.thing_name);
                        
                        let a_thing : Arc<RwLock<ThingObject>>  ;
                        if dezire.is_none() {
                            return;
                        }

                        a_thing = dezire.unwrap().clone();
               
                        let thing : &mut ThingObject = &mut a_thing.write().unwrap();

                        let o_event = thing.get_event_mut(&self.object_name);
                        if o_event.is_none() {
                            return;
                        }

                        let event : &mut EventObject = o_event.unwrap();

                        event.add_subscriber(&self.id);

                    }
                    if type_str == "observeproperty"  {
                        let lr = &mut self.app_state.write();
                        let app_state : &mut AppState = &mut  lr.as_deref_mut().unwrap();

                        let a_thing : Arc<RwLock<ThingObject>>   = match app_state.things.get_mut(&self.thing_name) {
                            None => return ,
                            Some(x) =>  x.clone()
                        };
               
                        let thing : &mut ThingObject = &mut a_thing.write().unwrap();
                    
                        let prop : &mut PropertyObject = match thing.get_property_mut(&self.object_name) {
                            None => return,
                            Some(x) => x
                        };
                
                        prop.add_subscriber(&self.id);

                    }
                    if type_str == "unobserveproperty" {
                        let lr = &mut self.app_state.write();
                        {
                            let app_state : &mut AppState = &mut  lr.as_deref_mut().unwrap();

                            let a_thing : Arc<RwLock<ThingObject>>   = match app_state.things.get_mut(&self.thing_name) {
                                None => return ,
                                Some(x) =>  x.clone()
                            };
                   
                            let thing : &mut ThingObject = &mut a_thing.write().unwrap();
                        
                            let prop : &mut PropertyObject = match thing.get_property_mut(&self.object_name) {
                                None => return,
                                Some(x) => x
                            };
                    
                            prop.remove_subscriber(&self.id);
                    
                            //self.remove_property(&mut app_state);
                        }
                    }

                    if type_str == "unsubscribeevent" {
                        let app_state : &mut AppState = &mut self.app_state.write().unwrap();
                        
                        let a_thing : Arc<RwLock<ThingObject>>   = match app_state.things.get_mut(&self.thing_name) {
                            None => return ,
                            Some(x) =>  x.clone()
                        };
               
                        let thing : &mut ThingObject = &mut a_thing.write().unwrap();


                        let event : &mut EventObject = match thing.get_event_mut(&self.object_name) {
                            None => return,
                            Some(x) => x
                        };

                        event.remove_subscriber(&self.id);

                    }
                }
                

            }
            Ok(ws::Message::Close(_)) => {
                self.remove_subscriber();
            }
            _ => (),
        }
    }    

}
