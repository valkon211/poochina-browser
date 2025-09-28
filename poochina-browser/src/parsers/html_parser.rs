// Импорты - подключаем необходимые модули
use std::collections::HashMap;

// === ОПРЕДЕЛЕНИЕ СТРУКТУР ДАННЫХ ===

// Перечисление для типов узлов DOM
#[derive(Debug, PartialEq)]
enum NodeType {
    Text(String),        // Текстовый узел содержит строку
    Element(ElementData), // Элемент содержит данные элемента
    Document,            // Корневой узел документа
}

// Структура для данных элемента HTML
#[derive(Debug, PartialEq)]
struct ElementData {
    tag_name: String,           // Имя тега (div, p, span и т.д.)
    attributes: HashMap<String, String>, // Атрибуты тега
}

// Структура узла DOM
#[derive(Debug, PartialEq)]
pub struct Node {
    node_type: NodeType, // Тип узла
    children: Vec<Node>, // Дочерние узлы
}

// === ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ ===

// Создает текстовый узел
fn create_text_node(data: &str) -> Node {
    Node {
        node_type: NodeType::Text(data.to_string()),
        children: Vec::new(), // Текстовые узлы не имеют детей
    }
}

// Создает элементный узел
fn create_element_node(tag_name: &str, attributes: HashMap<String, String>, children: Vec<Node>) -> Node {
    Node {
        node_type: NodeType::Element(ElementData {
            tag_name: tag_name.to_string(),
            attributes,
        }),
        children,
    }
}

// === ПАРСЕР HTML ===

pub struct HtmlParser {
    input: String,    // Входная HTML строка
    position: usize,  // Текущая позиция в строке
}

impl HtmlParser {
    // Конструктор парсера
    pub fn new(input: &str) -> Self {
        HtmlParser {
            input: input.to_string(),
            position: 0,
        }
    }

    // Проверяет, достигли ли мы конца строки
    fn eof(&self) -> bool {
        self.position >= self.input.len()
    }

    // Получает текущий символ без перемещения позиции
    fn current_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    // Пропускает пробельные символы
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    // Читает символ и перемещает позицию вперед
    fn consume_char(&mut self) -> Option<char> {
        if self.eof() {
            return None;
        }
        
        // Получаем следующий символ и его длину в байтах
        let next_char = self.current_char().unwrap();
        let char_len = next_char.len_utf8();
        
        // Перемещаем позицию
        self.position += char_len;
        
        Some(next_char)
    }

    // Читает последовательность символов, удовлетворяющую условию
    fn consume_while<F>(&mut self, condition: F) -> String 
    where 
        F: Fn(char) -> bool, // F - функция, принимающая char и возвращающая bool
    {
        let mut result = String::new();
        
        // Читаем символы, пока условие выполняется
        while let Some(c) = self.current_char() {
            if condition(c) {
                result.push(self.consume_char().unwrap());
            } else {
                break;
            }
        }
        
        result
    }

    // Парсит имя тега (последовательность букв, цифр, дефисов)
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric() || c == '-')
    }

    // Парсит текст до следующего тега
    fn parse_text(&mut self) -> String {
        self.consume_while(|c| c != '<')
    }

    // Парсит атрибуты тега
    fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        
        loop {
            self.skip_whitespace();
            
            // Если следующий символ '>' или '/', заканчиваем парсинг атрибутов
            if let Some(c) = self.current_char() {
                if c == '>' || c == '/' {
                    break;
                }
            }
            
            // Парсим имя атрибута
            let attr_name = self.parse_tag_name();
            
            // Пропускаем '='
            if self.consume_char() != Some('=') {
                // Атрибут без значения (например, 'disabled')
                attributes.insert(attr_name, "".to_string());
                continue;
            }
            
            // Парсим значение атрибута в кавычках
            let quote_char = self.consume_char().unwrap(); // Должен быть " или '
            let attr_value = self.consume_while(|c| c != quote_char);
            self.consume_char(); // Пропускаем закрывающую кавычку
            
            attributes.insert(attr_name, attr_value);
        }
        
        attributes
    }

    // Парсит один узел (элемент или текст)
    fn parse_node(&mut self) -> Option<Node> {
        // Пропускаем пробелы в начале
        self.skip_whitespace();
        
        if self.eof() {
            return None;
        }
        
        // Если начинается с '<' - это элемент
        if self.current_char() == Some('<') {
            self.parse_element()
        } else {
            // Иначе - это текст
            Some(self.parse_text_node())
        }
    }

    // Парсит текстовый узел
    fn parse_text_node(&mut self) -> Node {
        let text_content = self.parse_text();
        create_text_node(text_content.trim()) // Убираем лишние пробелы
    }

    // Парсит элемент (тег с атрибутами и содержимым)
    fn parse_element(&mut self) -> Option<Node> {
        // Должно начинаться с '<'
        if self.consume_char() != Some('<') {
            return None;
        }
        
        // Проверяем закрывающий тег
        if self.current_char() == Some('/') {
            self.consume_char(); // Пропускаем '/'
            
            // Пропускаем до '>'
            self.consume_while(|c| c != '>');
            self.consume_char(); // Пропускаем '>'
            
            // Закрывающий тег - возвращаем None для завершения рекурсии
            return None;
        }
        
        // Парсим имя тега
        let tag_name = self.parse_tag_name();
        
        // Парсим атрибуты
        let attributes = self.parse_attributes();
        
        // Проверяем самозакрывающийся тег
        let is_self_closing = if self.current_char() == Some('/') {
            self.consume_char(); // Пропускаем '/'
            true
        } else {
            false
        };
        
        // Пропускаем '>'
        if self.consume_char() != Some('>') {
            return None;
        }
        
        // Для самозакрывающихся тегов нет дочерних узлов
        if is_self_closing {
            return Some(create_element_node(&tag_name, attributes, Vec::new()));
        }
        
        // Парсим дочерние узлы
        let mut children = Vec::new();
        
        loop {
            // Пропускаем пробелы между узлами
            self.skip_whitespace();
            
            if self.eof() {
                break;
            }
            
            // Если встречаем закрывающий тег - выходим
            if self.input[self.position..].starts_with("</") {
                break;
            }
            
            // Парсим следующий узел
            if let Some(child_node) = self.parse_node() {
                children.push(child_node);
            }
        }
        
        // Пропускаем закрывающий тег
        if self.input[self.position..].starts_with("</") {
            self.consume_char(); // '<'
            self.consume_char(); // '/'
            self.parse_tag_name(); // Имя тега (пропускаем)
            self.consume_while(|c| c != '>');
            self.consume_char(); // '>'
        }
        
        Some(create_element_node(&tag_name, attributes, children))
    }

    // Основная функция парсинга
    pub fn parse(&mut self) -> Node {
        let mut children = Vec::new();
        
        // Парсим все узлы до конца документа
        while let Some(node) = self.parse_node() {
            children.push(node);
        }
        
        // Создаем корневой узел документа
        Node {
            node_type: NodeType::Document,
            children,
        }
    }
}

// === ФУНКЦИИ ДЛЯ РАБОТЫ С DOM ===

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

// === ВИЗУАЛИЗАЦИЯ DOM ===

// Рекурсивно печатает DOM-дерево
pub fn print_dom(node: &Node, indent_level: usize) {
    let indent = "  ".repeat(indent_level); // Создаем отступ
    
    match &node.node_type {
        NodeType::Document => {
            println!("{}Document", indent);
        },
        NodeType::Text(text) => {
            if !text.trim().is_empty() {
                println!("{}Text: '{}'", indent, text);
            }
        },
        NodeType::Element(elem_data) => {
            print!("{}<{}", indent, elem_data.tag_name);
            
            // Печатаем атрибуты
            for (key, value) in &elem_data.attributes {
                if value.is_empty() {
                    print!(" {}", key);
                } else {
                    print!(" {}=\"{}\"", key, value);
                }
            }
            
            if node.children.is_empty() {
                println!(" />");
            } else {
                println!(">");
                
                // Рекурсивно печатаем детей
                for child in &node.children {
                    print_dom(child, indent_level + 1);
                }
                
                println!("{}</{}>", indent, elem_data.tag_name);
            }
        }
    }
}