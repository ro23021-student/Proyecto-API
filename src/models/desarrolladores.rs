use serde::{Deserialize, Serialize};//importacion de libreria


#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Desarrollador {//estructura de desarrollador
    //campos del desarrollador
   pub id_desarrollador: i32,
   pub nombre: String,
   pub rol_principal: Option<String>,
   pub nivel: Option<String>,
}


#[derive(Deserialize)]
pub struct CrearDesarrollador {//estructura para recibir datos
   pub nombre: String,
   pub rol_principal: Option<String>,
   pub nivel: Option<String>,
}
