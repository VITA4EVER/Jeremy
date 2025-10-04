use log::info;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub status: String,
    pub data: Data,
}

#[derive(Deserialize)]
pub struct Data {
    pub resultType: String,
    pub result: Vec<ResultItem>,
}

#[derive(Deserialize)]
pub struct ResultItem {
    pub metric: Metric,
    pub value: (f64, String),
}

#[derive(Deserialize)]
pub struct Metric {
    #[serde(rename = "__name__")]
    pub __name__: String,
    pub instance: String,
    pub job: String,
    #[serde(default)] // skip this in cases where it does not exist.
    pub status: String,
}

pub async fn fetch_total_users() -> Result<i32, String> {
    let url = "http://localhost:9090/api/v1/query?query=mirage_total_users";
    let resp_wrapped = reqwest::get(url).await;
    match resp_wrapped {
        Ok(resp) => {
            let status = resp.status();

            if status != StatusCode::OK {
                return Err(format!("Status is not OK (status != 200; status == {})\nSomething went wrong with the request...", status))
            } else {
                let real_json_wrapped = resp.json::<Response>().await;
                match real_json_wrapped {
                    Ok(response_data) => {
                        if let Some(prefinal_result) = response_data.data.result.get(0) {
                            let final_result = prefinal_result.value.1.parse::<i32>();
                            match final_result {
                                Ok(needed_return_value) => {
                                    return Ok(needed_return_value)
                                }
                                Err(error) => {
                                    // Error when parsing the i32 from a String
                                    return Err(format!("Error when parsing the i32 from a String: {}", error))
                                }
                            }
                        } else {
                            return Err("No such index in the response_data.data.result vector!".to_string())
                        }
                    }
                    Err(error) => {
                        // Error when unwrapping the response data!
                        return Err(format!("{}", error))
                    }
                }
            }
        }
        Err(error) => {
            // Error when making the request.
            return Err(format!("{}", error))
        }
    }
}

pub async fn get_cheater_users() -> Result<i32, String> {
    let url = "http://localhost:9090/api/v1/query?query=mirage_cheater_users";
    let resp_wrapped = reqwest::get(url).await;
    match resp_wrapped {
        Ok(resp) => {
            let status = resp.status();

            if status != StatusCode::OK {
                return Err(format!("Status is not OK (status != 200; status == {})\nSomething went wrong with the request...", status))
            } else {
                let real_json_wrapped = resp.json::<Response>().await;
                match real_json_wrapped {
                    Ok(response_data) => {
                        if let Some(prefinal_result) = response_data.data.result.get(0) {
                            let final_result = prefinal_result.value.1.parse::<i32>();
                            match final_result {
                                Ok(needed_return_value) => {
                                    return Ok(needed_return_value)
                                }
                                Err(error) => {
                                    // Error when parsing the i32 from a String
                                    return Err(format!("Error when parsing the i32 from a String: {}", error))
                                }
                            }
                        } else {
                            return Err("No such index in the response_data.data.result vector!".to_string())
                        }
                    }
                    Err(error) => {
                        // Error when unwrapping the response data!
                        return Err(format!("{}", error))
                    }
                }
            }
        }
        Err(error) => {
            // Error when making the request.
            return Err(format!("{}", error))
        }
    }
}

// note: this specifically returns active users IN MULTIPLAYER.
pub async fn get_active_users() -> Result<i32, String> {
    let url = "http://localhost:9090/api/v1/query?query=mirage_active_users";
    let resp_wrapped = reqwest::get(url).await;
    match resp_wrapped {
        Ok(resp) => {
            let status = resp.status();

            if status != StatusCode::OK {
                return Err(format!("Status is not OK (status != 200; status == {})\nSomething went wrong with the request...", status))
            } else {
                let real_json_wrapped = resp.json::<Response>().await;
                match real_json_wrapped {
                    Ok(response_data) => {
                        for element in response_data.data.result.iter() {
                            info!("{}", element.metric.status); // for debugging, remove later
                            if element.metric.status == "PlayingMultiplayer".to_string() {
                                let wrapped_value = element.value.1.parse::<i32>();
                                match wrapped_value {
                                    Ok(result_to_return) => {
                                        return Ok(result_to_return)
                                    }
                                    Err(error) => {
                                        return Err(format!("Error when parsing the i32 from a String: {}", error))
                                    }
                                }
                            }
                        }

                        // No multiplayer field found:
                        return Err("No 'PlayingMultiplayer' value detected!'".to_string())
                    }
                    Err(error) => {
                        // Error when unwrapping the response data!
                        return Err(format!("{}", error))
                    }
                }
            }
        }
        Err(error) => {
            // Error when making the request.
            return Err(format!("{}", error))
        }
    }
}

// TODO: Make the get_matches_played function!
/*
pub async fn get_matches_played() -> Option<> {

}
 */