
pub mod fy{
    use libretranslate::{translate_url, Language};
    
    #[tokio::main]
    pub async fn fy_handle(input: String) -> String {
        let source = Language::English;
        let target = Language::Chinese;
        let url = "https://translate.argosopentech.com".to_string();
        
        let data = translate_url(source, target, input, url, None).await;

        let ret_str = match data {
            Ok(data) => data.output.to_string(),
            Err(error) => error.to_string(),
        };

        ret_str
    }
}
