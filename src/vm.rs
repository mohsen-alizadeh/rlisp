use crate::node;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref VARIABLES: Mutex<HashMap<String, node::Arg>> = Mutex::new(HashMap::new());
}

pub struct VM<'a> {
    pub ast: Vec<node::Expr>,
    pub variables: HashMap<String, &'a node::Arg>,
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

        // for i in 0..=self.ast.len() {
        //     self.run_expr(&self.ast[0]);
        // }

        // for i in 0..=self.ast.len() {
        //
        //     // println!("expr");
        //     // // self.run_test();
        //     // self.run_expr(&self.ast[0]);
        // }
        println!("VARIABLES: {:?}", VARIABLES.lock().unwrap());
    }

    // pub fn run_test(&mut self) {
    //
    // fn run_expr(&'a mut self, expr: &'a node::Expr) {
    //     match expr {
    //         node::Expr::Call(call) => self.run_call(call),
    //         _ => (),
    //     }
    // }
    //
    // fn run_call(&'a mut self, call: &'a node::Call) {
    //     match call.func_name {
    //         node::FuncName::Let => {
    //             if let node::Arg::Identifier(name) = call.args.value.get(0).unwrap() {
    //                 let value = call.args.value.get(1).unwrap();
    //                 // self.variables.insert(name.value, value.clone());
    //             } else {
    //                 println!("error on arg");
    //             }
    //         }
    //         _ => (),
    //     }
    // }
}

trait Run {
    fn run(&self) {
        println!("run trait");
    }
}

impl Run for node::Expr {
    fn run(&self) {
        match self {
            node::Expr::Call(call) => call.run(),
            _ => (),
        }
    }
}

impl Run for node::Call {
    fn run(&self) {
        match self.func_name {
            node::FuncName::Let => {
                if let node::Arg::Identifier(name) = self.args.value.get(0).unwrap() {
                    VARIABLES
                        .lock()
                        .unwrap()
                        .insert(name.value.clone(), self.args.value.get(1).unwrap().clone());
                } else {
                    println!("failed in call.run");
                }
            }
            node::FuncName::Plus => {
                println!("plus")
                let sum: usize = 0;

                for arg in &self.args.value {
                    match value {

                    }
                }
            }
            _ => (),
        }
    }
}



// impl Run {
//     fn run() {
//
//     }
// }
