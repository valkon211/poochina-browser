use std::collections::HashMap;

// Перечисление для типов узлов DOM
#[derive(Debug, PartialEq)]
pub enum NodeType {
    Text(String),        // Текстовый узел содержит строку
    Element(ElementData), // Элемент содержит данные элемента
    Document,            // Корневой узел документа
}

// Структура для данных элемента HTML
#[derive(Debug, PartialEq)]
pub struct ElementData {
    pub tag_name: String,           // Имя тега (div, p, span и т.д.)
    pub attributes: HashMap<String, String>, // Атрибуты тега
}

// Структура узла DOM
#[derive(Debug, PartialEq)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

impl Node {
    // Рекурсивный поиск элемента по ID
    pub fn find_by_id(&self, id: &str) -> Option<&Node> {
        // Проверяем текущий узел
        if let NodeType::Element(elem_data) = &self.node_type {
            if let Some(elem_id) = elem_data.attributes.get("id") {
                if elem_id == id {
                    return Some(self);
                }
            }
        }
        
        // Рекурсивно проверяем детей
        for child in &self.children {
            if let Some(found) = child.find_by_id(id) {
                return Some(found);
            }
        }
        
        None
    }
    
    // Поиск всех элементов по имени тега
    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<&Node> {
        let mut results = Vec::new();
        
        // Проверяем текущий узел
        if let NodeType::Element(elem_data) = &self.node_type {
            if elem_data.tag_name == tag_name {
                results.push(self);
            }
        }
        
        // Рекурсивно проверяем детей
        for child in &self.children {
            results.extend(child.get_elements_by_tag_name(tag_name));
        }
        
        results
    }
}

// === ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ ===

// Создает текстовый узел
pub fn create_text_node(data: &str) -> Node {
    Node {
        node_type: NodeType::Text(data.to_string()),
        children: Vec::new(), // Текстовые узлы не имеют детей
    }
}

// Создает элементный узел
pub fn create_element_node(tag_name: &str, attributes: HashMap<String, String>, children: Vec<Node>) -> Node {
    Node {
        node_type: NodeType::Element(ElementData {
            tag_name: tag_name.to_string(),
            attributes,
        }),
        children,
    }
}