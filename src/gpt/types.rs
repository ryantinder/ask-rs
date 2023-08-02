use serde::{Serialize, Deserialize};
use core::fmt::Display;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prompt {
    pub name: String,
    pub content: String
}
impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Prompt Name: {}", self.name).as_str())
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub name: String,
    pub api_key: String,
    pub model: String
}
impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Model: {}, Name: {}", self.model, self.name).as_str())
    }
}
/// The engine version for ChatGPT
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Serialize )]
#[allow(non_camel_case_types)]
pub enum ChatGPTEngine {
    /// Standard engine: `gpt-3.5-turbo`
    #[default]
    Gpt35Turbo,
    /// Different version of standard engine: `gpt-3.5-turbo-0301`
    Gpt35Turbo_0301,
    /// Base GPT-4 model: `gpt-4`
    Gpt4,
    /// Version of GPT-4, able to remember 32,000 tokens: `gpt-4-32k`
    Gpt4_32k
}

impl Display for ChatGPTEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for ChatGPTEngine {
    fn as_ref(&self) -> &'static str {
        match self {
            ChatGPTEngine::Gpt35Turbo => "gpt-3.5-turbo",
            ChatGPTEngine::Gpt35Turbo_0301 => "gpt-3.5-turbo-0301",
            ChatGPTEngine::Gpt4 => "gpt-4",
            ChatGPTEngine::Gpt4_32k => "gpt-4-32k"
        }
    }
}
/// A single response chunk, returned from streamed request
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum ResponseChunk {
    /// A chunk of message content
    Content {
        /// Piece of message content
        delta: String,
        /// Index of the message. Used when `reply_count` is set to more than 1 in API config
        response_index: usize,
    },
    /// Marks beginning of a new message response, with no actual content yet
    BeginResponse {
        /// The respondent's role (usually `Assistant`)
        role: Role,
        /// Index of the message. Used when `reply_count` is set to more than 1 in API config
        response_index: usize,
    },
    /// Ends a single message response response
    CloseResponse {
        /// Index of the message finished. Used when `reply_count` is set to more than 1 in API config
        response_index: usize,
    },
    /// Marks end of stream
    Done,
}

/// A role of a message sender, can be:
/// - `System`, for starting system message, that sets the tone of model
/// - `Assistant`, for messages sent by ChatGPT
/// - `User`, for messages sent by user
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// A system message, automatically sent at the start to set the tone of the model
    System,
    /// A message sent by ChatGPT
    Assistant,
    /// A message sent by the user
    User,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct CompletionRequest<'a> {
    pub model: &'a str,
    pub messages: &'a Vec<Message>,
    pub stream: bool
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Eq)]
pub struct CompletionError {
    /// Message, describing the error
    pub message: String,
    /// The type of error. Example: `server_error`
    #[serde(rename = "type")]
    pub error_type: String,
}

/// A role of a message sender, can be:
/// - `System`, for starting system message, that sets the tone of model
/// - `Assistant`, for messages sent by ChatGPT
/// - `User`, for messages sent by user
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Eq)]
#[serde(untagged)]
pub enum ServerResponse {
    Error {
        error: CompletionError
    },
    Response(Completion)
}
#[derive(Debug, Eq, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Completion {
    model: String,
    choices: Vec<Choice>
}

impl Completion {
    pub fn to_string(&self) -> String {
        self.choices[0].message.content.clone()
    }
}

#[derive(Debug, Eq, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Choice {
    index: u32,
    message: Message,
    finish_reason: String
}
#[derive(Debug, Eq, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Message {
    pub role: Role,
    pub content: String
}

/// A part of a chunked inbound response
#[derive(Debug, Clone, Deserialize)]
pub struct InboundResponseChunk {
    /// All message chunks in this response part (only one usually)
    pub choices: Vec<InboundChunkChoice>,
}

/// A single message part of a chunked inbound response
#[derive(Debug, Clone, Deserialize)]
pub struct InboundChunkChoice {
    /// The part value of the response
    pub delta: InboundChunkPayload,
    /// Index of the message this chunk refers to
    pub index: usize,
}
/// Contains different chunked inbound response payloads
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum InboundChunkPayload {
    /// Begins a single message by announcing roles (usually `assistant`)
    AnnounceRoles {
        /// The announced role
        role: Role,
    },
    /// Streams a part of message content
    StreamContent {
        /// The part of content
        content: String,
    },
    /// Closes a single message
    Close {},
}


/*
{
    "id": "chatcmpl-7gwNfSi7JS21scYjCizSD3IPtiRVx",
    "object": "chat.completion",
    "created": 1690468547,
    "model": "gpt-4-0613",
    "choices": [
        {
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "As an artificial intelligence, I don't have feelings or emotions, but I'm functioning as expected! How can I assist you today?"
            },
            "finish_reason": "stop"
        }
    ],
    "usage": {
        "prompt_tokens": 11,
        "completion_tokens": 27,
        "total_tokens": 38
    }
}
*/
