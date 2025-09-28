mod parsers;

use parsers::html_parser::HtmlParser;
use parsers::html_parser::print_dom;

fn main() {
    println!("=== ПРОСТОЙ HTML-ПАРСЕР НА RUST ===\n");
    
    // Тестовый HTML
    let html = r#"
        <div id="main" class="container">
            <h1>Заголовок</h1>
            <p>Это <strong>жирный</strong> текст.</p>
            <input type="text" disabled>
            <ul>
                <li>Пункт 1</li>
                <li>Пункт 2</li>
            </ul>
        </div>
    "#;
    
    println!("Исходный HTML:\n{}\n", html);
    
    // Создаем и запускаем парсер
    let mut parser = HtmlParser::new(html);
    let dom = parser.parse();
    
    println!("=== ПОСТРОЕННОЕ DOM-ДЕРЕВО ===");
    print_dom(&dom, 0);
    
    // Демонстрация поиска
    println!("\n=== ПОИСК ЭЛЕМЕНТОВ ===");
    
    if let Some(main_div) = dom.find_by_id("main") {
        println!("Найден элемент с id='main':");
        print_dom(main_div, 1);
    }
    
    let list_items = dom.get_elements_by_tag_name("li");
    println!("Найдено {} элементов <li>:", list_items.len());
    for item in list_items {
        print_dom(item, 1);
    }
}