use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id_prestador: String,
    pub id_veiculo:   String,
    pub timestamp:    String,
    pub chasis:       String,
    pub km:           i64,
    pub relatorio:    String,
}

