use std::collections::HashMap;

pub fn process_file(map: &mut HashMap<String, String>, file: &str) {
    // file is a file with a extension of .pdf
    let file_name = match file.split("/").last() {
        Some(file_name) => file_name,
        None => return,
    };
    let file_name = match file_name.split(".").next() {
        Some(file_name) => file_name,
        None => return,
    };
    map.insert(file_name.to_string(), file.to_string());
    map.insert(file_name.to_lowercase(), file.to_string());
    format_past_paper(file_name).iter().for_each(|file1| {
        map.insert(file1.to_string(), file.to_string());
    });
}

pub fn format_past_paper(file: &str) -> Vec<String> {
    let file = file.split("_").collect::<Vec<&str>>();
    if file.len() != 4 {
        return vec![];
    }

    let year = file[1].chars().skip(1).collect::<String>();
    let month = match file[1].chars().next() {
        Some('s') => "M/J",
        Some('w') => "O/N",
        Some('m') => "F/M",
        _ => return vec![],
    };
    let paper = file[2];
    let paper = match paper {
        "qp" => "QP",
        "ms" => "MS",
        _ => return vec![],
    };
    let subject = file[0];

    let paper_code = file[3];

    let formatted = format!("{}/{}/{}/{}/{}", subject, paper_code, month, year, paper);
    let formatted = formatted.to_lowercase();
    // if is QP, without QP as well is valid
    if paper == "QP" {
        let fra = formatted
            .split("/")
            .take(5)
            .collect::<Vec<&str>>()
            .join("/");
        return vec![
            formatted.clone(),
            formatted.clone() + "-qp",
            formatted.clone() + "/qp",
            formatted.clone() + "-ms" + "-qp",
            formatted + "/ms" + "/qp",
            fra.clone(),
            fra.clone() + "-qp",
            fra + "/qp",
            file.join("_") + "-qp",
        ];
    }
    let fra = formatted
        .split("/")
        .take(5)
        .collect::<Vec<&str>>()
        .join("/");
    vec![
        formatted.clone(),
        formatted.clone() + "-ms",
        formatted + "/ms",
        fra.clone() + "-ms",
        fra + "/ms",
        file.join("_") + "-ms",
    ]
}

// HashMap extension to get a value by key without case sensitivity
pub trait CaseInsensitiveGet {
    /// Get a value by key without case sensitivity.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice representing the key to search for.
    ///
    /// # Returns
    ///
    /// An option containing the value if the key is found, or `None` if the key is not found.
    ///
    /// # Example
    ///
    /// ```
    /// let mut map = HashMap::new();
    /// map.insert("key".to_string(), "value".to_string());
    /// assert_eq!(map.get_case_insensitive("KEY").unwrap(), "value");
    /// ```
    fn get_case_insensitive(&self, key: &str) -> Option<&String>;
}

impl CaseInsensitiveGet for HashMap<String, String> {
    fn get_case_insensitive(&self, key: &str) -> Option<&String> {
        self.get(&key.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_file() {
        let mut map: HashMap<String, String> = HashMap::new();
        process_file(&mut map, "static/9709_s19_qp_11.pdf");
        dbg!(&map);
        assert_eq!(
            map.get_case_insensitive("9709/11/M/J/19/QP").unwrap(),
            "static/9709_s19_qp_11.pdf"
        );
        assert_eq!(
            map.get_case_insensitive("9709/11/M/J/19").unwrap(),
            "static/9709_s19_qp_11.pdf"
        );

        process_file(&mut map, "9709_s19_ms_11.pdf");
        assert_eq!(
            map.get_case_insensitive("9709/11/M/J/19/MS").unwrap(),
            "9709_s19_ms_11.pdf"
        );

        process_file(&mut map, "9709_w19_qp_11.pdf");
        assert_eq!(
            map.get_case_insensitive("9709/11/O/N/19/QP").unwrap(),
            "9709_w19_qp_11.pdf"
        );
        process_file(&mut map, "9709_w19_ms_11.pdf");
        assert_eq!(
            map.get_case_insensitive("9709/11/O/N/19/MS").unwrap(),
            "9709_w19_ms_11.pdf"
        );

        process_file(&mut map, "9709_m19_qp_11.pdf");
        assert_eq!(
            map.get_case_insensitive("9709/11/F/M/19/QP").unwrap(),
            "9709_m19_qp_11.pdf"
        );
        process_file(&mut map, "9709_m19_ms_11.pdf");
        assert_eq!(
            map.get_case_insensitive("9709/11/F/M/19/MS").unwrap(),
            "9709_m19_ms_11.pdf"
        );

        process_file(&mut map, "random_file.pdf");
        assert_eq!(
            map.get_case_insensitive("random_file").unwrap(),
            "random_file.pdf"
        );
    }
}
