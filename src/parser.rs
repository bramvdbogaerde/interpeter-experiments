use crate::ast::Expr;
use crate::errors::Error;
use lexpr::Value;

fn compile_sequence(sequence: &Value, size: usize) -> Result<Vec<Expr>, Error> {
    match sequence {
        Value::Null => Ok(vec![ Expr::Nil ; size ]),
        Value::Cons(cns) => {
            let car_compiled = compile(cns.car())?;
            let mut rest_compiled = compile_sequence(cns.cdr(), size+1)?;
            rest_compiled[size] = car_compiled;
            Ok(rest_compiled)
        }
        _ => Err(Error::CompileError)
    }
}

fn compile_application(operator: &Value, operands: &Value) -> Result<Expr, Error> {
    let compiled_operator = compile(operator)?;
    let compiled_operands = compile_sequence(operands, 0)?;
    Ok(Expr::Apply { operator: Box::new(compiled_operator), operands: compiled_operands })
}

fn compile_parameters(params: &Value, size: usize) -> Result<Vec<String>, Error> {
    match params {
        Value::Null => Ok(vec![ "".to_string() ; size ]),
        Value::Cons(cns) => {
            let mut rest = compile_parameters(cns.cdr(), size+1)?;
            if let Some(name) = cns.car().as_symbol() {
                rest[size] = name.to_string();
                Ok(rest)
            } else {
                Err(Error::CompileError)
            }
        }
        _ => Err(Error::CompileError)
    }
}

fn compile_lambda(operands: &Value) -> Result<Expr, Error> {
    if let Some(cns) = operands.as_cons() {
        let arguments = compile_parameters(cns.car(), 0)?;
        let body = compile_sequence(cns.cdr(), 0)?;
        Ok(Expr::Lambda {
            arguments,
            body
        })
    } else {
        Err(Error::CompileError)
    }
}

fn compile_if(operands: &Value) -> Result<Expr, Error> {
    // condition is there
    if let Some(conditionAndRest) = operands.as_cons() {
        if let Some(consequentAndRest) = conditionAndRest.cdr().as_cons() {
            if let Some(alternativeAndRest) = consequentAndRest.cdr().as_cons() {
                if alternativeAndRest.cdr().is_null() {
                    let compiled_condition = compile(conditionAndRest.car())?;
                    let compiled_consequent = compile(consequentAndRest.car())?;
                    let compiled_alternative = compile(alternativeAndRest.car())?;
                    return Ok(Expr::If {
                        cond: Box::new(compiled_condition),
                        cnsq: Box::new(compiled_consequent),
                        alt: Box::new(compiled_alternative),
                    })
                }
            }
        }
    }

    Err(Error::CompileError)
}

fn compile_letrec(operands: &Value) -> Result<Expr, Error> {
    if let Some(cns) = operands.as_cons() {
        if let Some(binding) = cns.car().as_cons() {
            if let Some(name) = binding.car().as_symbol() {
                if let Some(residue) = binding.cdr().as_cons() {
                    let value = residue.car() ;
                    if residue.cdr().is_null() {
                        let compiled_value = compile(value)?;
                        let compiled_body = compile_sequence(cns.cdr(), 0)?;
                        if compiled_body.len() > 1 {
                            return Err(Error::CompileError);
                        }

                        return Ok(Expr::Letrec {
                            name: name.to_string(),
                            binding: Box::new(compiled_value),
                            body: Box::new(compiled_body[0].clone())
                        })
                    }
                }
            }
        }
    }

    Err(Error::CompileError)
}

fn compile_form(operator: &Value, operands: &Value) -> Result<Expr, Error> {
    if let Some(name) = operator.as_symbol() {
        match name {
            "lambda" => compile_lambda(operands),
            "if" => compile_if(operands),
            "letrec" => compile_letrec(operands),
            _ => compile_application(operator, operands),
        }
    } else {
        compile_application(operator, operands)
    }
}

pub fn compile(v: &Value) -> Result<Expr, Error> {
    match v {
        Value::Symbol(v) => Ok(Expr::Ident(v.to_string())),
        Value::Cons(cns) => compile_form(cns.car(), cns.cdr()),
        Value::Bool(v) => Ok(Expr::Bool(*v)),
        Value::Number(n) => 
            if let Some(n) = n.as_u64() {
                Ok(Expr::Num(n as usize))
            } else { Err(Error::CompileError) },
        Value::Nil => Ok(Expr::Nil),
        Value::String(s) => Ok(Expr::Str(s.to_string())),
        _ => Err(Error::CompileError)
    }
}

pub fn parse<'s>(from: &'s str) -> Result<Expr, Error> {
    let v = lexpr::from_str(from).map_err(|_| Error::ParseError)?;
    compile(&v)
}
