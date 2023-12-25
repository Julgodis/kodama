pub mod data {
    use crate::kodama::Kodama;
    use crate::Result;
    use axum::{extract::State, Json};
    use kodama_api::record::{DataRequest, DataResponse};

    pub async fn handler(
        State(database_path): State<String>,
        Json(input): Json<DataRequest>,
    ) -> Result<Json<DataResponse>> {
        let entries = Kodama::instance(database_path)?.record_entries(
            &input.project_name,
            &input.service_name,
            &input.record_name,
        )?;
        Ok(Json(DataResponse { entries }))
    }
}

pub mod list {
    use crate::kodama::Kodama;
    use crate::Result;
    use axum::{extract::State, Json};
    use kodama_api::record::{ListRequest, ListResponse};

    pub async fn handler(
        State(database_path): State<String>,
        Json(input): Json<ListRequest>,
    ) -> Result<Json<ListResponse>> {
        let records = Kodama::instance(database_path)?
            .record_list(&input.project_name, &input.service_name)?;
        Ok(Json(ListResponse { records }))
    }
}
