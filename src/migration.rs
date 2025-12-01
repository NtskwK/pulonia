use serde_json::{Value, json};
use std::collections::HashMap;

/// 根据文档中的迁移协议 v1 生成迁移记录
pub fn generate_migration(before_inner: &Value, after_inner: &Value) -> Value {
    let before_files = flatten_to_map(before_inner, String::new());
    let after_files = flatten_to_map(after_inner, String::new());

    let mut update = json!({});
    let mut deleted = Vec::new();

    // 处理所有在 after 中的文件（新增或修改）
    for (path, new_hash) in &after_files {
        match before_files.get(path) {
            Some(old_hash) if old_hash != new_hash => {
                // 文件被修改
                add_to_update_tree(&mut update, path, new_hash);
            }
            None => {
                // 文件被添加
                add_to_update_tree(&mut update, path, new_hash);
            }
            _ => {
                // 文件未变化，不需要处理
            }
        }
    }

    // 处理被删除的文件
    for path in before_files.keys() {
        if !after_files.contains_key(path) {
            deleted.push(path.clone());
        }
    }

    json!({
        "version": "1.0",
        "update": update,
        "deleted": deleted
    })
}

/// 将路径添加到更新树中
fn add_to_update_tree(tree: &mut Value, path: &str, hash: &str) {
    let parts: Vec<&str> = path.split('/').collect();
    add_to_tree_recursive(tree, &parts, hash, 0);
}

/// 递归辅助函数，用于添加路径到更新树
fn add_to_tree_recursive(node: &mut Value, parts: &[&str], hash: &str, index: usize) {
    if index >= parts.len() {
        return;
    }

    let part = parts[index];

    // 确保当前节点是一个对象
    if !node.is_object() {
        *node = json!({});
    }

    if index == parts.len() - 1 {
        // 最后一个部分，插入文件信息
        if let Some(obj) = node.as_object_mut() {
            obj.insert(
                part.to_string(),
                json!({
                    "hash": hash
                }),
            );
        }
    } else {
        // 中间路径，递归处理
        if let Some(obj) = node.as_object_mut() {
            let part_string = part.to_string();

            // 确保子节点存在
            if !obj.contains_key(&part_string) {
                obj.insert(part_string.clone(), json!({}));
            }

            // 递归处理下一级
            if let Some(child) = obj.get_mut(&part_string) {
                add_to_tree_recursive(child, parts, hash, index + 1);
            }
        }
    }
}

/// 将嵌套的 JSON 结构展平为路径 -> 哈希的映射
fn flatten_to_map(value: &Value, current_path: String) -> HashMap<String, String> {
    let mut result = HashMap::new();

    if let Some(obj) = value.as_object() {
        if let Some(hash_val) = obj.get("hash") {
            if let Some(hash) = hash_val.as_str() {
                // 这是一个有哈希值的节点
                if !current_path.is_empty() {
                    result.insert(current_path.clone(), hash.to_string());
                }
            }
        }

        if let Some(children) = obj.get("child").and_then(|v| v.as_array()) {
            // 这是一个有子节点的目录
            for child in children {
                if let Some(child_obj) = child.as_object() {
                    for (name, child_value) in child_obj {
                        let child_path = if current_path.is_empty() {
                            name.clone()
                        } else {
                            format!("{}/{}", current_path, name)
                        };

                        let child_files = flatten_to_map(child_value, child_path);
                        result.extend(child_files);
                    }
                }
            }
        }
    }

    result
}
