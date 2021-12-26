use crate::node;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref VARIABLES: Mutex<HashMap<String, node::Value>> = Mutex::new(HashMap::new());
}

pub struct VM<'a> {
    pub ast: Vec<node::Expr>,
    pub variables: HashMap<String, &'a node::Value>,
}

impl<'a> VM<'_> {
    pub fn new(ast: Vec<node::Expr>) -> VM<'a> {
        VM {
            ast: ast,
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        for expr in &self.ast {
            expr.run();
        }
    }
}

trait Run {
    fn run(&self) -> Option<node::Value> {
        None
    }
}

impl Run for node::Expr {
    fn run(&self) -> Option<node::Value> {
        match self {
            node::Expr::Call(call) => call.run(),
            node::Expr::List(list) => Some(node::Value::List(list.to_vec())),
        }
    }
}

impl Run for node::Call {
    fn run(&self) -> Option<node::Value> {
        match self.func_name {
            node::FuncName::Let => {
                if let node::Value::Identifier(name) = self.args.get(0).unwrap() {
                    VARIABLES
                        .lock()
                        .unwrap()
                        .insert(name.clone(), self.args.get(1).unwrap().clone());
                } else {
                    println!("failed in call.run");
                }

                None
            }
            node::FuncName::Plus => {
                let mut sum: usize = 0;

                for arg in &self.args {
                    sum += arg.sum();
                }

                Some(node::Value::Number(sum))
            }
            node::FuncName::Print => {
                for arg in &self.args {
                    println!("{}", arg.to_s());
                }

                None
            }
        }
    }
}

impl node::Value {
    fn to_s(&self) -> String {
        match self {
            node::Value::Identifier(name) => VARIABLES.lock().unwrap().get(name).unwrap().to_s(),
            node::Value::Number(number) => number.to_string(),
            node::Value::Literal(string) => string.clone(),
            node::Value::Expr(expr) => expr.run().unwrap().to_s(),
            _ => String::from("to_s is called"),
        }
    }

    fn sum(&self) -> usize {
        match self {
            node::Value::Identifier(name) => VARIABLES.lock().unwrap().get(name).unwrap().sum(),
            node::Value::Number(number) => *number,
            node::Value::Expr(expr) => expr.run().unwrap().sum(),
            node::Value::List(list) => {
                let mut s = 0;
                for i in list {
                    s += i.sum();
                }

                s
            }
            _ => 0,
        }
    }
}
