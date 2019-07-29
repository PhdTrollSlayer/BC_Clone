use crate::models::report::Report;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Veiculo {
    pub id:         String, // Placa
    pub chasis:     String,
    pub km_atual:   i64,
    pub relatorios: Vec<Report>
}

impl Veiculo {
    pub fn verificar(&mut self) {
        self.relatorios.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
        let mut prev_timestamp: i64 = 0;
        let mut prev_km: i64 = 0;
        for r in self.relatorios.iter_mut() {
            if r.timestamp.parse::<i64>().unwrap() < prev_timestamp {
                panic!("Erro na validação das timestamps do veiculo")
            } else {
                if r.km < prev_km {
                    panic!("Erro na validação da quilometragem do veículo")
                } else {
                    prev_timestamp = r.timestamp.parse::<i64>().unwrap();
                    prev_km = r.km.clone();
                    self.km_atual = prev_km.clone();
                }
            }
        }


        println!("Sucesso na validação do veiculo!");
    }

}
