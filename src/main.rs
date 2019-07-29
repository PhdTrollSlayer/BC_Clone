#![feature(proc_macro_hygiene, decl_macro)]
mod blockchain;
mod criptografia;
mod models;

use blockchain::*;
use models::report::*;

use std::sync::{RwLock, Mutex};

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::http::RawStr;
use rocket::response::content::Json;
use rocket::response::status;
use rocket::http::Status;

#[allow(deprecated, unreachable_code)]
fn main() {
    let mut bc = Blockchain::inicializar();

    let r = Report {
        id_prestador: "89".to_string(),
        id_veiculo: "3213".to_string(),
        timestamp: "12312312".to_string(),
        chasis: "123123".to_string(),
        km: 0,
        relatorio: "".to_string(),
        assinatura: "123".to_string(),
    };

    rocket::ignite()
           .mount("/", routes![consultar_placa, submeter_relatorio])
           .manage(RwLock::new(bc))
           .launch();
}

#[post("/submeter_relatorio", data="<data>")]
    fn submeter_relatorio(bc: State<RwLock<Blockchain>>, data: String) -> status::Custom<String> {
    let response: status::Custom<&str>;

    let pr: Result<Report, _> = serde_json::from_str(&data);

    match pr {
        Ok(s) => {
            let mut x = bc.write().unwrap();
            x.inserir_report(s);

        }
        Err(e) => {
            return status::Custom(Status::BadRequest, "Err: #003 = Formatação do relatório inválida!".to_string());
        }
    }

    status::Custom(Status::BadRequest, "Err: #001 Requisição mal feita!".to_string())

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
