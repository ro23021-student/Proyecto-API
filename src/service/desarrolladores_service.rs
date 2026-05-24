use sqlx::PgPool;// conexion a PostgreSQL
use crate::models::desarrolladores::{Desarrollador, CrearDesarrollador};//importacion del modelo desarrolladores
use crate::repository::desarrolladores_repository as repo;//importacion de archivo repositoy 


pub async fn listar(pool: &PgPool) -> Vec<Desarrollador> {//funcion publica que lista a todos los desarrolladores
   repo::obtener_todos(pool).await
}


pub async fn buscar(pool: &PgPool, id: i32) -> Option<Desarrollador> {//Busca un desarrollador especificamente con su id
   repo::obtener_por_id(pool, id).await
}


pub async fn crear(pool: &PgPool, datos: CrearDesarrollador) -> Desarrollador {//Recibe datos y crea un desarrollador.
   repo::crear(pool, datos).await
}


pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearDesarrollador) -> Desarrollador {//Actualiza los datos de un desarrollador ya existente
   repo::actualizar(pool, id, datos).await
}


pub async fn eliminar(pool: &PgPool, id: i32) -> bool {//Elimina los datos de un desarrollador ya existente
   repo::eliminar(pool, id).await
}
