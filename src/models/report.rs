use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreReport {
    pub id_prestador: String,
    pub id_veiculo:   String,
    pub km:           i64,
    pub timestamp:    String,
    pub relatorio:    String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id_prestador: String,
    pub id_veiculo:   String,
    pub timestamp:    String,
    pub chasis:       String,
    pub km:           i64,
    pub relatorio:    String,
    pub assinatura:   String,
}
