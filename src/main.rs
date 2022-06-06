use regex::Regex;

fn convert(caps: Option<regex::Captures>) -> (i32, i32, &str) {
    let caps = caps.unwrap();

    let cap_expression : &str = caps.get(0).unwrap().as_str();
    let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    return (left_value, right_value, cap_expression);
}

fn main() {
    let mut expression : String = String::new();

    // Regex
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?\~\s?(\d+)").unwrap();
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_subs = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();

    // Get user Data
    println!("Por favor introduce tu expresión: ");
    std::io::stdin().read_line(&mut expression).unwrap();
    
    // Product
    loop {
        let caps = re_mult.captures(expression.as_str()); // Otorga lo analizado: Some(Captures({0: Some("11+10"), 1: Some("11")... }))
        if caps.is_none() {
            break;
        }

        let (left_value, right_value, cap_expression) = convert(caps);
        
        let product = left_value * right_value;
        
        expression = expression.replace(cap_expression, &product.to_string());
    }
    // Division
    loop {
        let caps = re_div.captures(expression.as_str()); 
        
        if caps.is_none() {
            break;
        }

        let (left_value, right_value, cap_expression) = convert(caps);
        
        let division = left_value / right_value;
        
        expression = expression.replace(cap_expression, &division.to_string());
    }
    // Adition
    loop {
        let caps = re_add.captures(expression.as_str()); 
        
        if caps.is_none() {
            break;
        }

        let (left_value, right_value, cap_expression) = convert(caps);
        
        let addition = left_value + right_value;
        
        expression = expression.replace(cap_expression, &addition.to_string());
    }
    // Substraction
    loop {
        let caps = re_subs.captures(expression.as_str()); 
        
        if caps.is_none() {
            break;
        }

        let (left_value, right_value, cap_expression) = convert(caps);
        
        let subtraction = left_value - right_value;
        
        expression = expression.replace(cap_expression, &subtraction.to_string());
    }

    // Show the resolve
    println!("Resultado: {}", expression);
}
