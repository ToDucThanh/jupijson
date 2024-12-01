use jupijson::{loads, JsonValue};

fn main() {
    let json_str: &str = r#"{
        "name": "To Duc Thanh",
        "nationality": "Viet Nam",
        "age": 24,
        "skills": ["Python", "JavaScript", "SQL", "Rust"],
        "address": {
            "city": "Ho Chi Minh",
            "zip": "70000"
        }
    }"#;

    match loads(json_str) {
        Ok(user) => {
            if let JsonValue::Object(obj) = user {

                if let Some(JsonValue::String(name)) = obj.get("name") {
                    println!("Name: {}", name);
                }

                if let Some(JsonValue::String(nationality)) = obj.get("nationality") {
                    println!("Nationality: {}", nationality);
                }

                if let Some(JsonValue::Number(age)) = obj.get("age") {
                    println!("Age: {}", age);
                }

                if let Some(JsonValue::Array(skills)) = obj.get("skills") {
                    println!("Skills:");
                    for skill in skills {
                        if let JsonValue::String(skill_str) = skill {
                            println!("  - {}", skill_str);
                        }
                    }
                }
                if let Some(JsonValue::Object(address)) = obj.get("address") {
                    println!("Address:");
                    if let Some(JsonValue::String(city)) = address.get("city") {
                        println!("  City: {}", city);
                    }
                    if let Some(JsonValue::String(zip)) = address.get("zip") {
                        println!("  ZIP: {}", zip);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error parsing JSON: {:?}", e),
    }
}