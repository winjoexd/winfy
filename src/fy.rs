

pub mod fy{
    use libretranslate::{translate_url, Language};
    #[tokio::main]
    pub async fn fy_handle(input: String) -> String {
        let source = Language::English;
        let target = Language::Chinese;
        let url = "https://translate.argosopentech.com".to_string();
        
        let data = translate_url(source, target, input, url, None).await.unwrap();

        data.output.to_string()
    }
}
