use super::bindings::*;
use super::*;
use types::Value;

pub fn lambda_placeholder(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let params = exprs.remove(0);
    let params_list = if let SExpr::Vec(symbols) = params {
        let mut list = Vec::new();
        for symbol in symbols {
            list.push(match symbol {
                SExpr::Symbol(name) => SExpr::ISymbol(hash_str(&name), name),
                SExpr::ISymbol(id, name) => SExpr::ISymbol(id, name),
                _ => {
                    return Err(format!(
                        "lambda can only bind to symbols, found {:?}",
                        symbol
                    ))
                }
            });
        }
        list
    } else {
        return Err(format!("lambda form should be vector, found {:?}", params));
    };
    Ok(SExpr::LAMBDA(params_list, exprs))
}

pub fn eval_lambda(lambda_expr: &SExpr, params: Vec<SExpr>) -> Result<SExpr, String> {
    if let &SExpr::LAMBDA(ref params_list, ref body) = lambda_expr {
        {
            // bind parameters
            let mut param_pos = 0;
            for param in params {
                let lambda_param = params_list.get(param_pos).unwrap();
                if let &SExpr::ISymbol(id, _) = lambda_param {
                    bind(id, param);
                } else {
                    return Err(format!(
                        "Expect ISymbol for lambda form, found {:?}",
                        lambda_param
                    ));
                }
                param_pos += 1;
            }
        }
        let mut last_result = SExpr::Value(Value::Null);
        for body_line in body {
            // eval function body by cloning expression
            last_result = body_line.clone().eval()?;
        }
        for lambda_param in params_list {
            // unbind parameters
            if let &SExpr::ISymbol(id, _) = lambda_param {
                unbind(id);
            }
        }
        Ok(last_result)
    } else {
        return Err(format!("Expect lambda expression, found {:?}", lambda_expr));
    }
}
