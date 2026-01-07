use crate::cli::ProviderType;
use crate::error::{ClxError, Result};
use crate::prompt::Prompt;
use crate::providers::ProviderInfo;
use genai::chat::{ChatMessage, ChatRequest};
use genai::resolver::{AuthData, Endpoint, ServiceTargetResolver};
use genai::{adapter::AdapterKind, Client, ClientBuilder, ModelIden, ServiceTarget};

pub struct Provider {
    client: Client,
    model: String,
}

impl Provider {
    pub fn new(provider_type: ProviderType, model: String, api_key: Option<String>) -> Result<Self> {
        let info = ProviderInfo::for_provider(provider_type);

        if let Some(key) = api_key {
            if !info.env_key.is_empty() {
                std::env::set_var(info.env_key, key);
            }
        }

        let client = match provider_type {
            ProviderType::Openai | ProviderType::Claude | ProviderType::Gemini => {
                Client::default()
            }
            ProviderType::Groq => build_openai_compatible_client(
                "https://api.groq.com/openai/v1/",
                "GROQ_API_KEY",
            ),
            ProviderType::Ollama => build_openai_compatible_client(
                "http://localhost:11434/v1/",
                "",
            ),
            ProviderType::Openrouter => build_openai_compatible_client(
                "https://openrouter.ai/api/v1/",
                "OPENROUTER_API_KEY",
            ),
            ProviderType::Deepseek => build_openai_compatible_client(
                "https://api.deepseek.com/v1/",
                "DEEPSEEK_API_KEY",
            ),
            ProviderType::Xai => build_openai_compatible_client(
                "https://api.x.ai/v1/",
                "XAI_API_KEY",
            ),
        };

        Ok(Self { client, model })
    }

    pub async fn generate(&self, prompt: Prompt) -> Result<String> {
        let chat_req = ChatRequest::new(vec![
            ChatMessage::system(prompt.system),
            ChatMessage::user(prompt.user),
        ]);

        let response = self
            .client
            .exec_chat(&self.model, chat_req, None)
            .await
            .map_err(|e| ClxError::Api(e.to_string()))?;

        response
            .first_text()
            .map(|s| s.to_string())
            .ok_or(ClxError::NoResponse)
    }
}

fn build_openai_compatible_client(endpoint: &'static str, env_key: &'static str) -> Client {
    let resolver = ServiceTargetResolver::from_resolver_fn(
        move |service_target: ServiceTarget| -> std::result::Result<ServiceTarget, genai::resolver::Error> {
            let ServiceTarget { model, .. } = service_target;
            let auth = if env_key.is_empty() {
                AuthData::from_single("ollama")
            } else {
                AuthData::from_env(env_key)
            };
            Ok(ServiceTarget {
                endpoint: Endpoint::from_static(endpoint),
                auth,
                model: ModelIden::new(AdapterKind::OpenAI, model.model_name),
            })
        },
    );
    ClientBuilder::default()
        .with_service_target_resolver(resolver)
        .build()
}
