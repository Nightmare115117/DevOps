pub struct empleados{
    id_emplado: i64,
    id_usuario: i64,
    nombre: String,
    apellido_paterno: result<String>,
    apellido_materno: result<String>,
    telefono: result<String>,
    rfc: String,
    fecha_ingreso: date::NaiveDate,
    foto_perfil: result<String>,
    created_at: date::NaiveDateTime,
    is_active: bool,
    id_rol: i64,
    id_plantaDestino: i64,
}

impl model{
    
}