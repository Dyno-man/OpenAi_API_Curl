use curl::easy::{Easy, List};
use std::env;
use serde_json::{json, Value, Error};
use dotenv::dotenv;
use serde::{Serialize, Deserialize};

//OpenAi Api url
const OPENAI_URL: &'static str = "https://api.openai.com/v1/chat/completions";

#[derive(Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde{rename= "user"}]
    User,
}

//Struct for modifying the OpenAi Model content, you can select either a user message or a system message
#[derive(Serialize, Deserialize)]
pub struct Message {
    role: Role,
    content: String,
}

//Implementations for system struct
impl Message {
    //Create a new message Struct
    pub fn new(role: Role, content: String) -> Self {
        Self {role, content}
    }
    
    //Set the message content
    pub fn set_message_content(&mut self, content: String) {
        self.content = content;
    }

    //Get the system content
    pub fn get_content(&mut self) -> &String {
        &self.content
    }

    //Get the system role
    pub fn get_role(&mut self) -> &Role {
        &self.role
    }

}   

//Send your message to the OpenAi Api
pub fn send_message(system: &mut Message, user: &mut Message) -> String{
    let jsonify = json!({
        "model": "gpt-4o-mini",
        "messages": [
            {
                "role": system.get_role(),
                "content": system.get_content(),
            },
            {
                "role": user.get_role(),
                "content": user.get_content(),
            }
        ],
    });


    //Create an Easy_curl struct
    let mut easy = Easy::new();

    //Set the easy url
    easy.url(&OPENAI_URL).unwrap();

    //Set post to True to post the message to the OpenAI API
    easy.post(true).unwrap();

    //Convert jsonify into a String
    let jsonify_to_string = jsonify.to_string();

    //Post the jsonify_to_string as bytes
    easy.post_fields_copy(jsonify_to_string.as_bytes()).unwrap();

    //Add the header to the easy String
    let mut header = List::new();
    header.append("Content-Type: application/json").unwrap();
    header.append(&format!("Authorization: Bearer {}", get_api_key())).unwrap();
    easy.http_headers(header).unwrap();

    //Handle response with a vector
    let mut response = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data|{
            response.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();

        if let Err(e) = transfer.perform() {
            eprintln!("Error making request {}", e);
            return String::from("Error making request!");
        }

    }

    let response_code = easy.response_code().unwrap();
    let response_string = String::from_utf8_lossy(&response).to_string();

    if response_code != 200 {
        return format!("There was an unexpected error code: {}", response_code)
    } else {
        let temp_str = handle_response(&response_string).expect("Failed to handle response string");
        return temp_str
    }

}

fn handle_response(response: &str) -> Result<String, Error> {
    let response_json: Value = serde_json::from_str(response)?;

    if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
        Ok(content.to_string())
    } else {
        Err(serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to extract response content",
        )))
    }
}

fn get_api_key() -> String {
    env::var("OPENAI_API_KEY").expect("API key not found!")
}