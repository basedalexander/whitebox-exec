fn filter_data(params: Vec<String>, data: Vec<Object>) -> Vec<Object> {
    let mut result: Vec<Object> = Vec::new();

    for obj in data {
        for word in params[0].value {
            if obj.metadata.content.contains(word) {
                result.push(obj);
            }
        }
    }
    return result;
}