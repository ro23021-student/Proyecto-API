use axum::{Router, routing::{get, post, put, delete}, extract::{Path, State}, Json};// Importaciones necesarias de Axum para crear rutas HTTP y manejar peticiones
use sqlx::PgPool;// Conexion a PostgreSQL
use crate::models::desarrolladores::CrearDesarrollador;// Struct usado para recibir datos
use crate::service::desarrolladores_service as service;// Importamos el service


pub fn desarrolladores_router(pool: PgPool) -> Router {// Función principal que crea las rutas del módulo desarrolladores
   Router::new()
       .route("/desarrolladores", get(listar).post(crear))
       .route("/desarrolladores/{id}", get(buscar).put(actualizar).delete(eliminar))
       .with_state(pool)
}


async fn listar(State(pool): State<PgPool>) -> Json<serde_json::Value> {// LISTAR DESARROLLADORES
   Json(serde_json::json!(service::listar(&pool).await))
}


async fn buscar(State(pool): State<PgPool>, Path(id): Path<i32>) -> Json<serde_json::Value> {// BUSCAR POR ID
   Json(serde_json::json!(service::buscar(&pool, id).await))
}


async fn crear(State(pool): State<PgPool>, Json(datos): Json<CrearDesarrollador>) -> Json<serde_json::Value> {// CREAR DESARROLLADOR
   Json(serde_json::json!(service::crear(&pool, datos).await))
}


async fn actualizar(State(pool): State<PgPool>, Path(id): Path<i32>, Json(datos): Json<CrearDesarrollador>) -> Json<serde_json::Value> {// ACTUALIZAR DESARROLLADOR
   Json(serde_json::json!(service::actualizar(&pool, id, datos).await))
}


async fn eliminar(State(pool): State<PgPool>, Path(id): Path<i32>) -> Json<serde_json::Value> {// ELIMINAR DESARROLLADOR
   let eliminado = service::eliminar(&pool, id).await;
   if eliminado {
       Json(serde_json::json!({"mensaje": "Desarrollador eliminado correctamente"}))
   } else {
       Json(serde_json::json!({"error": "No se pudo eliminar, verifique el id o registros relacionados"}))
   }
}
