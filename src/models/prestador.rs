use crate::models::veiculo::Veiculo;

use rand::{Rng};
use rand::distributions::{Alphanumeric};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Prestador {
    pub nome: String,
    pub id: String,
    pub api_key: String,
    pub veiculos_presentes: Vec<Veiculo>,
}

impl Prestador {
    pub fn new(nome: &str) -> Prestador {
        let api_key = rand::thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        let id = rand::thread_rng().sample_iter(&Alphanumeric).take(16).collect();
        Prestador {
            nome: nome.to_string(),
            id: id,
            api_key: api_key,
            veiculos_presentes: Vec::new(),
        }
    }
}
