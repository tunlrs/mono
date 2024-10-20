#[taurpc::procedures(export_to = "../src/lib/tauri/bindings.ts")]
trait Api {
    async fn handle_create_user(body: tunlrs_dtos::CreateUser) -> Result<tunlrs_dtos::User, String>;
}

#[derive(Clone)]
struct ApiImpl;

impl Api for ApiImpl {
    type handle_create_userFut = std::pin::Pin<Box<dyn std::future::Future<Output = Result<tunlrs_dtos::User, String>> + Send>>;

    fn handle_create_user(self, body: tunlrs_dtos::CreateUser) -> Self::handle_create_userFut {
        Box::pin(async move {
            let client = tauri_plugin_http::reqwest::Client::new();
            let res = client
                .post("http://localhost:3000/users")
                .json(&body)
                .send()
                .await
                .map_err(|err| err.to_string())?;
            let user = res
                .json::<tunlrs_dtos::User>()
                .await
                .map_err(|err| err.to_string())?;
            Ok(user)
        })
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        // .invoke_handler(tauri::generate_handler![handle_create_user])
        .invoke_handler(taurpc::create_ipc_handler(ApiImpl.into_handler()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
