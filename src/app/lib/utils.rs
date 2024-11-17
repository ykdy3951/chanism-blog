pub fn class_names (
    base_classes: &[&str],
    conditional_classes: &[(&str, bool)],
) -> String {
    let mut class_names = Vec::new();

    for base_class in base_classes {
        if base_class.is_empty() {
            continue;
        }
        class_names.push(base_class.to_string());
    }

    for (class_name, condition) in conditional_classes {
        if *condition {
            class_names.push(class_name.to_string());
        }
    }

    class_names.join(" ")
}