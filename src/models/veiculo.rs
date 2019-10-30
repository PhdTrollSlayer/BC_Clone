use crate::models::report::Report;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Veiculo {
    pub id: String, // Placa
    pub chasis: String,
    pub km_atual: i64,
    pub relatorios: Vec<Report>,
}

impl Veiculo {
    pub fn verificar(&mut self) -> Result<(()), String> {
        self.relatorios
            .sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
        let mut prev_timestamp: i64 = 0;
        let mut prev_km: i64 = 0;
        for r in self.relatorios.iter_mut() {
            let ts = r.timestamp.parse::<i64>();
            match ts {
                Ok(t) => {
                    if t < prev_timestamp {
                        return Err("Erro na validação das timestamps do veiculo".to_string());
                    } else {
                        if r.km < prev_km {
                            return Err("Erro na validação da quilometragem do veículo".to_string());
                        } else {
                            prev_timestamp = r.timestamp.parse::<i64>().unwrap();
                            prev_km = r.km.clone();
                            self.km_atual = prev_km.clone();
                        }
                    }
                }
                Err(_) => return Err("Erro na análise da timestamp fornecida".to_string()),
            }
        }

        Ok(())
    }
}
