pub fn add_params_to_url(url:String, params: Vec<(String, Option<String>)>) -> String {
    let mut url_params = String::new();
    let mut seperate_char = '?';
    for i in 0..params.len() {
        if let Some(value) = &params[i].1 {
            if seperate_char == '?' && url_params.len() > 0 {
                seperate_char = '&';
            }
            url_params += &(seperate_char.to_string() + &params[i].0 + "=" + &value);
        } else {
            continue;
        }
    }
    url + &url_params
}