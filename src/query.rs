use std::collections::HashMap;

pub fn parse_query(path: &str) -> (String, HashMap<String, String>) {
	let mut query = HashMap::new();
	if let Some((path, query_str)) = path.split_once('?') {
		for pair in query_str.split('&') {
			let mut parts = pair.split('=');
			let key = parts.next().unwrap();
			let value = parts.next().unwrap_or("");
			query.insert(key.into(), value.into());
		}
		return (path.to_owned(), query);
	}
	(path.to_owned(), query)
}
