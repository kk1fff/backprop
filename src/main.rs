use std::collections::HashSet;

trait Function {
    fn derivative_over(&self, node_name: &str) -> f64;
    fn compute_value(&mut self) -> (Vec<String>, f64);
    fn has_node(&self, node_name: &str) -> bool;
}


struct ConstVal {
    val: f64,
    name: String,
}

impl ConstVal {
    fn new(v: f64, name: &str) -> ConstVal {
        ConstVal {
            val: v,
            name: String::from(name)
        }
    }
}

impl Function for ConstVal {
    fn derivative_over(&self, node_name: &str) -> f64 {
        if self.has_node(node_name) {
            1_f64
        } else {
            panic!("Not found: {:?}", node_name);
        }
    }
    fn compute_value(&mut self) -> (Vec<String>, f64) {
        (vec![self.name.clone()], self.val)
    }
    fn has_node(&self, node_name: &str) -> bool {
        self.name == node_name
    }
}


struct AddFunc<Op1: Function, Op2: Function> {
    op1: Op1,
    op2: Op2,
    val1: f64,
    val2: f64,
    name: String,
}

impl <Op1, Op2> AddFunc <Op1, Op2>
    where Op1: Function, Op2: Function {
    fn new(op1: Op1, op2: Op2, name: &str) -> AddFunc<Op1, Op2> {
        AddFunc {
            op1: op1,
            op2: op2,
            val1: 0_f64,
            val2: 0_f64,
            name: String::from(name)
        }
    }
}

impl <Op1, Op2> Function for AddFunc <Op1, Op2>
    where Op1: Function, Op2: Function {
    fn derivative_over(&self, node_name: &str) -> f64 {
        if self.op1.has_node(node_name) {
            1_f64 * self.op1.derivative_over(node_name)
        } else if self.op2.has_node(node_name) {
            1_f64 * self.op2.derivative_over(node_name)
        } else {
            panic!("Not found: {:?}", node_name);
        }
    }

    fn compute_value(&mut self) -> (Vec<String>, f64) {
        let (mut op1, val1) = self.op1.compute_value();
        let (mut op2, val2) = self.op2.compute_value();
        op1.append(&mut op2);
        self.val1 = val1;
        self.val2 = val2;
        (op1, val1+val2)
    }

    fn has_node(&self, node_name: &str) -> bool {
        self.op1.has_node(node_name) || self.op2.has_node(node_name)
    }
}

struct MultiplyFunc<Op1: Function, Op2: Function> {
    op1: Op1,
    op2: Op2,
    val1: f64,
    val2: f64,
    name: String,
    op_set: HashSet<String>,
}

impl <Op1, Op2> MultiplyFunc <Op1, Op2>
    where Op1: Function, Op2: Function {
    fn new(op1: Op1, op2: Op2, name: &str) -> MultiplyFunc<Op1, Op2> {
        MultiplyFunc {
            op1: op1,
            op2: op2,
            val1: 0_f64,
            val2: 0_f64,
            name: String::from(name),
            op_set: HashSet::new()
        }
    }
}

impl <Op1, Op2> Function for MultiplyFunc <Op1, Op2>
    where Op1: Function, Op2: Function {
    fn derivative_over(&self, node_name: &str) -> f64 {
        if self.op1.has_node(node_name) {
            self.val2 * self.op1.derivative_over(node_name)
        } else if self.op2.has_node(node_name) {
            self.val1 * self.op2.derivative_over(node_name)
        } else {
            panic!("Not found: {:?}", node_name);
        }
    }

    fn compute_value(&mut self) -> (Vec<String>, f64) {
        let (mut op1, val1) = self.op1.compute_value();
        let (mut op2, val2) = self.op2.compute_value();
        op1.append(&mut op2);
        self.val1 = val1;
        self.val2 = val2;
        (op1, self.val1 * self.val2)
    }

    fn has_node(&self, node_name: &str) -> bool {
        self.op1.has_node(node_name) || self.op2.has_node(node_name)
    }
}


fn main() {
    let a = ConstVal::new(10_f64, "A");
    let b = ConstVal::new(5_f64, "B");
    let c = ConstVal::new(20_f64, "C"); 
    let d = ConstVal::new(25_f64, "D");

    let f1 = AddFunc::new(a, b, "A+B");
    let f2 = MultiplyFunc::new(f1, c, "(A+B)C");

    let mut f_end = MultiplyFunc::new(f2, d, "(A+B)CD");
    let (_, v) = f_end.compute_value();
    println!("Val: {:?}", v);
    println!("Derive: {:?}", f_end.derivative_over("D"));
}
