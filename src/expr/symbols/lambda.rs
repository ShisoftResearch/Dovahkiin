use super::*;
use super::bindings::*;
use types::Value;

pub static LAMBDA_TAG_ID: u64 = hash_ident!(LAMBDA) as u64;

pub fn lambda_placeholder(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let mut replacement_list = Vec::with_capacity(1 + exprs.len());
    let params = exprs.remove(0);
    let params_list = if let SExpr::Vec(symbols) = params {
        let mut list = Vec::new();
        for symbol in symbols {
            list.push(match symbol {
                SExpr::Symbol(name) => SExpr::ISymbol(hash_str(&name)),
                SExpr::ISymbol(id) => SExpr::ISymbol(id),
                _ => return Err(format!("lambda can only bind to symbols, found {:?}", symbol))
            });
        }
        list
    } else {
        return Err(format!("lambda form should be vector, found {:?}", params))
    };
    replacement_list.push(SExpr::ISymbol(LAMBDA_TAG_ID));
    replacement_list.push(SExpr::Vec(params_list));
    replacement_list.append(&mut exprs);
    Ok(SExpr::List(replacement_list))
}

pub fn eval_lambda(placeholder: &SExpr, params: Vec<SExpr>) -> Result<SExpr, String> {
    let mut lambda_expr_list = if let &SExpr::List(ref list) = placeholder { list } else {
        return Err(format!("Lambda expression list required, found {:?}", placeholder));
    };
    let lambda_tag = lambda_expr_list.first().ok_or_else(|| format!("Lambda expression should not be empty"))?;
    match lambda_tag {
        &SExpr::ISymbol(tag_id) if tag_id == LAMBDA_TAG_ID => {},
         _ => return Err(format!("Expect lambda expression, found first element {:?}", lambda_tag))
    }
    let params_form = lambda_expr_list.get(1).ok_or_else(|| format!("Lambda expression should have parameter form"))?;
    let params_list = if let &SExpr::Vec(ref list) = params_form { list } else {
        return Err(format!("Lambda expression should have a vector parameter form, found {:?}", params_form));
    };
    if params_list.len() != params.len() {
        return Err(format!("Parameter number does not match. Expected {} for lambda but found {}", params_list.len(), params.len()));
    }
    { // bind parameters
        let mut param_pos = 0;
        for param in params {
            let lambda_param = params_list.get(param_pos).unwrap();
            if let &SExpr::ISymbol(id) = lambda_param {
                bind(id, param);
            }
            param_pos += 1;
        }
    }
    if lambda_expr_list.len() < 3 {
        return Err(String::from("Lambda expression should have a body"));
    }
    let mut last_result = SExpr::Value(Value::Null);
    for body_idx in 2..lambda_expr_list.len() { // eval function body by cloning expression
        last_result = lambda_expr_list.get(body_idx).cloned().unwrap().eval()?;
    }
    for lambda_param in params_list { // unbind parameters
        if let &SExpr::ISymbol(id) = lambda_param {
                unbind(id);
        }
    }
    Ok(last_result)
}