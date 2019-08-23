#![feature(proc_macro_hygiene, decl_macro)]
mod blockchain;
mod criptografia;
mod models;

use blockchain::*;
use models::report::*;

use std::sync::{RwLock};

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::http::RawStr;
use rocket::response::content::Json;
use rocket::response::status;
use rocket::http::Status;

#[allow(deprecated, unreachable_code)]
fn main() {
    let bc = Blockchain::inicializar();

    rocket::ignite()
           .mount("/", routes![
               consultar_placa, 
               submeter_relatorio, 
               prestadores,
               todos_veiculos,
               login,
           ])
           .manage(RwLock::new(bc))
           .launch();
}

#[get("/login/<api_key>")]
// TODO: Embalar o Json em um HTTPStatus
fn login(bc: State<RwLock<Blockchain>>, api_key: &RawStr) -> Json<String> {
    // Confirma API key e retorna a struct Prestador procurada
    match bc.read().unwrap().confirm_api_key(&api_key) {
        Ok(s) => {
            Json(s)
        }
        Err(_) => {
            Json(r#"{"status": "Api Key não reconhecida"}"#.to_string())
        }
    }
}

#[get("/prestadores")]
fn prestadores(bc: State<RwLock<Blockchain>>) -> Json<String> {
    Json(bc.read().unwrap().get_all_prestadores())
}

#[get("/consulta/*")]
fn todos_veiculos(bc: State<RwLock<Blockchain>>) -> Json<String> {
    Json(bc.read().unwrap().get_all_veiculos())
}


#[post("/submeter_relatorio", data="<data>")]
fn submeter_relatorio(bc: State<RwLock<Blockchain>>, data: String) -> status::Custom<String> {
    let _response: status::Custom<&str>;

    let pr: Result<Report, _> = serde_json::from_str(&data);

    match pr {
        Ok(s) => {
            let mut x = bc.write().unwrap();
            match x.inserir_report(s) {
                Ok(()) => return status::Custom(Status::Ok, "Ok: #001 = Relatório submetido!".to_string()),

                Err(e) => return status::Custom(Status::BadRequest, format!("Err: #005 = {}", e))
            }
 
        }
        Err(_) => {
            return status::Custom(Status::BadRequest, "Err: #003 = Formatação do relatório inválida!".to_string());
        }
    }
}

#[get("/consulta/<placa>")]
fn consultar_placa(bc: State<RwLock<Blockchain>>, placa: &RawStr) -> Json<String> {
    let resultado = bc.read().unwrap().consultar_veiuculo(placa);

    match resultado {
        Some(v) => {
            Json(serde_json::to_string(&v).unwrap())
        },
        None => {Json("{}".to_string())}
    }
}
